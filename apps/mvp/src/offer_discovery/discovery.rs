pub mod io {
    pub use super::eventing::OfferDiscoverySubscriber;
}

mod eventing {
    use super::super::download::io::DownloadOfferPage;
    use crate::assembly::io::{AppCommand, MvpEvent};
    use crate::database::DbPool;
    use crate::schema::command_entries;
    use crate::sitemap_discovery::io::GroupedSitemapContentRepository;
    use diesel::prelude::*;
    use kernel::io::NewCommandMetadata;
    use kernel::{EventError, EventSubscriberPort, NewEventEnvelope};
    use uuid::Uuid;

    /// Reacts to `SitemapContentGrouped`: fans every grouped product URL out into its own
    /// `DownloadOfferPage` command. All command rows are inserted in a single transaction so the
    /// fan-out is atomic (all offer pages enqueued, or none).
    pub struct OfferDiscoverySubscriber {
        pool: DbPool,
        grouped_content_repo: GroupedSitemapContentRepository,
    }

    impl OfferDiscoverySubscriber {
        pub fn new(pool: DbPool, grouped_content_repo: GroupedSitemapContentRepository) -> Self {
            Self {
                pool,
                grouped_content_repo,
            }
        }
    }

    /// A `DownloadOfferPage` command materialized into the columns of a `command_entries` row.
    struct PendingCommand {
        id: Uuid,
        payload: String,
        meta: serde_json::Value,
    }

    impl EventSubscriberPort for OfferDiscoverySubscriber {
        fn handle(&self, envelope: &NewEventEnvelope) -> Result<(), EventError> {
            let event: MvpEvent = serde_json::from_str(&envelope.payload)
                .map_err(|error| EventError::SubscriberExecution(error.to_string()))?;
            let MvpEvent::SitemapContentGrouped(event) = event else {
                return Err(EventError::SubscriberExecution(
                    "unexpected event for offer discovery subscriber".to_string(),
                ));
            };

            if event.product_count == 0 {
                return Ok(());
            }

            let grouped_content = self
                .grouped_content_repo
                .load(event.grouped_content_id)
                .map_err(|error| EventError::SubscriberExecution(error.to_string()))?;
            let urls = grouped_content.links.product;
            if urls.is_empty() {
                return Ok(());
            }

            let correlation_id = envelope.metadata.as_ref().and_then(|metadata| metadata.correlation_id);
            let causation_id = envelope.metadata.as_ref().map(|metadata| metadata.event_id);

            // Materialize the command rows first so all fallible (de)serialization happens outside
            // the transaction, keeping the transaction body infallible except for storage errors.
            let pending = urls
                .into_iter()
                .map(|url| {
                    let command_id = Uuid::now_v7();
                    let command = AppCommand::DownloadOfferPage(DownloadOfferPage {
                        grouped_content_id: event.grouped_content_id,
                        retailer_code: event.retailer_code,
                        url,
                    });
                    let payload = serde_json::to_string(&command)
                        .map_err(|error| EventError::SubscriberExecution(error.to_string()))?;
                    let meta = serde_json::to_value(NewCommandMetadata {
                        command_id,
                        correlation_id,
                        causation_id,
                        source: Some("event:SitemapContentGrouped".to_string()),
                    })
                    .map_err(|error| EventError::SubscriberExecution(error.to_string()))?;
                    Ok(PendingCommand {
                        id: command_id,
                        payload,
                        meta,
                    })
                })
                .collect::<Result<Vec<_>, EventError>>()?;

            let mut connection = self
                .pool
                .get()
                .map_err(|error| EventError::SubscriberExecution(error.to_string()))?;

            connection
                .transaction::<(), diesel::result::Error, _>(|connection| {
                    for command in &pending {
                        diesel::insert_into(command_entries::table)
                            .values((
                                command_entries::id.eq(command.id),
                                command_entries::command_type
                                    .eq(super::super::download::io::DOWNLOAD_OFFER_PAGE_COMMAND),
                                command_entries::payload.eq(&command.payload),
                                command_entries::meta.eq(Some(&command.meta)),
                            ))
                            .on_conflict_do_nothing()
                            .execute(connection)?;
                    }
                    Ok(())
                })
                .map_err(|error| EventError::SubscriberExecution(error.to_string()))
        }
    }
}
