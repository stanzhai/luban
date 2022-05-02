use sea_orm::Statement;
use sea_schema::migration::{prelude::*, sea_orm::ConnectionTrait};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220419_000001_create_prototype_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"-- Table Definition
CREATE TABLE IF NOT EXISTS "prototype" (
    "id" bpchar(21) NOT NULL,
    "project_id" bpchar(21) NOT NULL,
    "name" varchar(128) NOT NULL,
    "owner" varchar(64) NOT NULL,
    "comment" text,
    "ctime" timestamp NOT NULL DEFAULT now(),
    "version" int4 NOT NULL,
    "is_del" bool NOT NULL,
    PRIMARY KEY ("id")
);"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE prototype";
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
