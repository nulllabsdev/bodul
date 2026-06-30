use std::error::Error;
use std::sync::Arc;

use crate::database;
use crate::database::{DatabaseConfig, DbPool};
use crate::sitemap_discovery::io::ProcessedSitemapRepository;
use crate::sitemap_discovery::io::SitemapRetrievalRepository;
use crate::sitemap_discovery::io::{
    GroupSitemapContent, GroupedSitemapContentRepository, ProcessSitemap, RawSitemapDocumentRepository,
    RequestSitemapRetrieval, SitemapContentGrouped, SitemapProcessed, SitemapRetrieved,
};
use kernel::ApplicationCommand;
use kernel::io::CommandError as MulacCommandError;
use poem::IntoResponse;
use poem::http::StatusCode;
use serde::{Deserialize, Serialize};

pub mod io {
    pub use super::{
        ApiError,
        AppCommand,
        AppError,
        AppState,
        ErrorBody,
        MulacHandle,
        MulacState,
        MvpEvent,
        NewCommandEnvelope,
        boot,
        interpret_dispatch_error,
        start_mulac, //
    };
    pub use kernel::io::{run_command_worker, run_event_worker};
}

pub fn boot() -> Result<MulacHandle, Box<dyn Error>> {
    let database_config = DatabaseConfig::from_env();
    let pool = database::connect(&database_config)?;

    database::run_migrations(&pool)?;

    Ok(start_mulac(pool, 0)?)
}

// ************************************************************************************************
// ************************************************************************************************
// ************************************************************************************************
// ************************************************************************************************

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub mulac: MulacState,
}

impl AppState {
    pub fn new(pool: DbPool, mulac: MulacState) -> Self {
        Self { pool, mulac }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorBody {
    pub error: String,
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("validation failed: {0}")]
    Validation(String),
    #[error("storage error: {0}")]
    Storage(String),
}

pub type ApiError = poem::Error;

impl From<AppError> for poem::Error {
    fn from(error: AppError) -> Self {
        let status = match error {
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::Storage(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        poem::Error::from_response(
            (
                status,
                poem::web::Json(ErrorBody {
                    error: error.to_string(),
                }),
            )
                .into_response(),
        )
    }
}

pub fn interpret_dispatch_error(error: kernel::KernelError) -> AppError {
    if let kernel::KernelError::Command(MulacCommandError::HandlerExecution(message)) = &error
        && let Some(message) = message.strip_prefix("validation failed: ")
    {
        return AppError::Validation(message.to_string());
    }

    AppError::Storage(format!("command dispatch failed: {error}"))
}

#[derive(Debug, Clone)]
pub enum AppCommand {
    RequestSitemapRetrieval(RequestSitemapRetrieval),
    ProcessSitemap(ProcessSitemap),
    GroupSitemapContent(GroupSitemapContent),
}

impl serde::Serialize for AppCommand {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::RequestSitemapRetrieval(command) => command.serialize(serializer),
            Self::ProcessSitemap(command) => command.serialize(serializer),
            Self::GroupSitemapContent(command) => command.serialize(serializer),
        }
    }
}

impl ApplicationCommand for AppCommand {
    fn command_type(&self) -> &'static str {
        match self {
            Self::RequestSitemapRetrieval(command) => command.command_type(),
            Self::ProcessSitemap(command) => command.command_type(),
            Self::GroupSitemapContent(command) => command.command_type(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum MvpEvent {
    SitemapRetrieved(SitemapRetrieved),
    SitemapProcessed(SitemapProcessed),
    SitemapContentGrouped(SitemapContentGrouped),
}

impl kernel::ApplicationEvent for MvpEvent {
    fn event_type(&self) -> &'static str {
        match self {
            Self::SitemapRetrieved(event) => event.event_type(),
            Self::SitemapProcessed(event) => event.event_type(),
            Self::SitemapContentGrouped(event) => event.event_type(),
        }
    }
}

pub type NewCommandEnvelope = kernel::NewCommandEnvelope<AppCommand>;
pub type MulacState = kernel::PersistentKernelState;
pub type MulacHandle = kernel::PersistentKernelHandle;

pub fn start_mulac(pool: DbPool, drain_rounds: usize) -> Result<MulacHandle, kernel::KernelError> {
    use crate::sitemap_discovery::io::GroupSitemapContentHandler;
    use crate::sitemap_discovery::io::SitemapContentGroupedSubscriber;
    use crate::sitemap_discovery::io::{ProcessSitemapHandler, SitemapProcessedSubscriber};
    use crate::sitemap_discovery::io::{RequestSitemapRetrievalHandler, SitemapRetrievedSubscriber};

    use crate::sitemap_discovery::io::{
        GROUP_SITEMAP_CONTENT_COMMAND,
        PROCESS_SITEMAP_COMMAND,
        REQUEST_SITEMAP_RETRIEVAL_COMMAND,
        SITEMAP_CONTENT_GROUPED_EVENT, //
        SITEMAP_PROCESSED_EVENT,
        SITEMAP_RETRIEVED_EVENT,
    };

    kernel::boot(kernel::KernelConfig::default())
        .command_handler(
            REQUEST_SITEMAP_RETRIEVAL_COMMAND,
            Arc::new(RequestSitemapRetrievalHandler::new(
                SitemapRetrievalRepository::new(pool.clone()),
                RawSitemapDocumentRepository::new(pool.clone()),
            )),
        )
        .command_handler(
            PROCESS_SITEMAP_COMMAND,
            Arc::new(ProcessSitemapHandler::new(
                SitemapRetrievalRepository::new(pool.clone()),
                RawSitemapDocumentRepository::new(pool.clone()),
                ProcessedSitemapRepository::new(pool.clone()),
            )),
        )
        .command_handler(
            GROUP_SITEMAP_CONTENT_COMMAND,
            Arc::new(GroupSitemapContentHandler::new(
                SitemapRetrievalRepository::new(pool.clone()),
                ProcessedSitemapRepository::new(pool.clone()),
                GroupedSitemapContentRepository::new(pool.clone()),
            )),
        )
        .event_subscriber_with_command_gateway(SITEMAP_RETRIEVED_EVENT, "process-sitemap", |command_gateway| {
            Arc::new(SitemapRetrievedSubscriber::new(command_gateway)) as Arc<dyn kernel::EventSubscriberPort>
        })
        .event_subscriber_with_command_gateway(SITEMAP_PROCESSED_EVENT, "group-sitemap-content", |command_gateway| {
            Arc::new(SitemapProcessedSubscriber::new(command_gateway)) as Arc<dyn kernel::EventSubscriberPort>
        })
        .event_subscriber(
            SITEMAP_CONTENT_GROUPED_EVENT,
            "sitemap-content-grouped-terminal",
            Arc::new(SitemapContentGroupedSubscriber) as Arc<dyn kernel::EventSubscriberPort>,
        )
        .start_persistent(pool, drain_rounds)
}
