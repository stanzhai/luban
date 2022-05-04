use poem::{
    listener::TcpListener,
    middleware,
    session::{CookieConfig, MemoryStorage, ServerSession},
    EndpointExt, Route, Server,
};
use poem::endpoint::{EmbeddedFileEndpoint, EmbeddedFilesEndpoint};
use poem_openapi::OpenApiService;
use sea_orm::{entity::*, DatabaseConnection};
use state::State;
use rust_embed::RustEmbed;

use crate::{api::*, config, server::endpoint::domain::Domain};

pub mod endpoint;
pub mod response;
pub mod state;

#[derive(RustEmbed)]
#[folder = "web/dist"]
pub struct Files;

fn routes(state: State, domain: &str) -> impl poem::Endpoint {
    let api_service = OpenApiService::new((auth::AuthApi, project::Project), "LuBan Api", "1.0")
        .server(format!("http://{}/api", domain));
    let ui = api_service.swagger_ui();

    Route::new()
        .nest("/api", api_service)
        .nest("/docs", ui)
        .nest("/", EmbeddedFilesEndpoint::<Files>::new())
        .with(ServerSession::new(
            CookieConfig::new().secure(false).http_only(false).name("sid"),
            MemoryStorage::new(),
        ))
        .with(middleware::Tracing)
        .data(state)
}

pub async fn start(conn: DatabaseConnection) -> Result<(), std::io::Error> {
    let config = config::Config::global();
    let server_url = format!("0.0.0.0:{}", config.http_port);
    let state = State { conn };

    println!("Starting server at {}", server_url);

    let domain = &config.domain;
    let route = routes(state, domain.as_str());
    let domain_endpoint = Domain::new(domain.to_owned(), route);
    let server = Server::new(TcpListener::bind(server_url));
    server.run(domain_endpoint).await
}
