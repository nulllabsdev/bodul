use poem::{Route, Server, get, handler, listener::TcpListener};

const DEFAULT_ADDR: &str = "127.0.0.1:3000";

fn app() -> Route {
    Route::new()
        .at("/", get(index))
        .at("/healthz", get(healthz))
}

#[handler]
fn index() -> &'static str {
    "bodul-mvp is running"
}

#[handler]
fn healthz() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr = std::env::var("BODUL_SERVER_ADDR").unwrap_or_else(|_| DEFAULT_ADDR.to_string());
    println!("server listening on http://{addr}");

    Server::new(TcpListener::bind(addr)).run(app()).await
}
