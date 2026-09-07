use std::env;
use std::time::Duration;

use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use diesel::{RunQueryDsl, sql_query};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

const DEFAULT_DATABASE_URL: &str = "postgres://bodul:bodul@localhost:5432/bodul";
const DEFAULT_MAX_CONNECTIONS: u32 = 20;
const DEFAULT_CONNECT_TIMEOUT_SECONDS: u64 = 30;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub connect_timeout: Duration,
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        let url = env::var("DATABASE_URL")
            .or_else(|_| env::var("BODUL_DATABASE_URL"))
            .unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_string());
        let max_connections = env::var("BODUL_DATABASE_MAX_CONNECTIONS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(DEFAULT_MAX_CONNECTIONS);
        let connect_timeout = env::var("BODUL_DATABASE_CONNECT_TIMEOUT_SECONDS")
            .ok()
            .and_then(|value| value.parse().ok())
            .map(Duration::from_secs)
            .unwrap_or_else(|| Duration::from_secs(DEFAULT_CONNECT_TIMEOUT_SECONDS));

        Self {
            url,
            max_connections,
            connect_timeout,
        }
    }
}

pub fn connect(config: &DatabaseConfig) -> Result<DbPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(config.url.clone());
    Pool::builder()
        .max_size(config.max_connections)
        .connection_timeout(config.connect_timeout)
        .build(manager)
}

pub fn health_check(pool: &DbPool) -> Result<(), DatabaseError> {
    let mut connection = pool.get()?;
    sql_query("SELECT 1").execute(&mut connection)?;
    Ok(())
}

pub fn run_migrations(pool: &DbPool) -> Result<(), DatabaseError> {
    let mut connection = pool.get()?;
    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|error| DatabaseError::Migration(error.to_string()))?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("failed to get database connection: {0}")]
    Pool(#[from] PoolError),
    #[error("database query failed: {0}")]
    Query(#[from] diesel::result::Error),
    #[error("database migration failed: {0}")]
    Migration(String),
}
