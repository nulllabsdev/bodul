use std::error::Error;

use mvp::assembly::io::{AppState, start_mulac};
use mvp::database;
use poem::http::StatusCode;
use poem::middleware::AddData;
use poem::web::Data;
use poem::{EndpointExt, Response, Route, Server, get, handler, listener::TcpListener};

const DEFAULT_ADDR: &str = "127.0.0.1:3000";

fn app(state: AppState) -> impl poem::Endpoint {
    Route::new()
        .at("/", get(index))
        .at("/healthz", get(healthz))
        .at("/readyz", get(readyz))
        .with(AddData::new(state))
}

#[handler]
fn index() -> &'static str {
    "bodul-mvp is running"
}

#[handler]
fn healthz() -> &'static str {
    "ok"
}

#[handler]
async fn readyz(Data(state): Data<&AppState>) -> Response {
    let pool = state.pool.clone();
    let result = tokio::task::spawn_blocking(move || database::health_check(&pool)).await;

    match result {
        Ok(Ok(())) => Response::builder().status(StatusCode::OK).body("ok"),
        Ok(Err(error)) => Response::builder()
            .status(StatusCode::SERVICE_UNAVAILABLE)
            .body(format!("database unavailable: {error}")),
        Err(error) => Response::builder()
            .status(StatusCode::SERVICE_UNAVAILABLE)
            .body(format!("database readiness task failed: {error}")),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let _guard = mvp::logging::init();
    let addr = std::env::var("BODUL_SERVER_ADDR").unwrap_or_else(|_| DEFAULT_ADDR.to_string());
    let database_config = database::DatabaseConfig::from_env();
    let pool = database::connect(&database_config)?;
    database::run_migrations(&pool)?;
    let kernel = start_mulac(pool.clone(), 0)?;
    let state = AppState::new(pool, kernel.state());

    println!("server listening on http://{addr}");

    Server::new(TcpListener::bind(addr)).run(app(state)).await?;
    Ok(())
}
