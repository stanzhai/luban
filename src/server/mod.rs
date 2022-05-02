use poem::{
    listener::TcpListener,
    middleware,
    session::{CookieConfig, MemoryStorage, ServerSession},
    EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;
use sea_orm::{entity::*, DatabaseConnection};
use state::State;

use crate::{api::*, config, server::endpoint::domain::Domain};

pub mod endpoint;
pub mod response;
pub mod state;

fn routes(state: State, server_url: &str) -> impl poem::Endpoint {
    let api_service = OpenApiService::new((auth::AuthApi, project::Project), "LuBan Api", "1.0")
        .server(format!("http://{}/api", server_url));
    let ui = api_service.swagger_ui();

    Route::new()
        .nest("/api", api_service)
        .nest("/docs", ui)
        .with(ServerSession::new(
            CookieConfig::new().secure(false).name("sid"),
            MemoryStorage::new(),
        ))
        .with(middleware::Tracing)
        .data(state)
}

pub async fn start(conn: DatabaseConnection) -> Result<(), std::io::Error> {
    let config = config::Config::global();
    let server_url = format!("{}:{}", config.host, config.port);
    let state = State { conn };

    println!("Starting server at {}", server_url);

    let http_host = &config.host;
    let route = routes(state, http_host.as_str());
    let domain_endpoint = Domain::new(http_host.to_owned(), route);
    let server = Server::new(TcpListener::bind(server_url));
    server.run(domain_endpoint).await
}
