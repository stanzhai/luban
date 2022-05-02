use std::env;

use luban::{config, server};
use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    color_eyre::install().unwrap();
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    config::Config::init_from_dotenv();

    let config = config::Config::global();
    // create tables if not exists
    let conn = sea_orm::Database::connect(&config.db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    server::start(conn).await
}
