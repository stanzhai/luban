use sea_orm::Statement;
use sea_schema::migration::{prelude::*, sea_orm::ConnectionTrait};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220418_000001_create_project_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"-- Table Definition
CREATE TABLE IF NOT EXISTS "project" (
    "id" bpchar(21) NOT NULL,
    "name" varchar(64) NOT NULL,
    "owner" varchar(64) NOT NULL,
    "comment" text,
    "ctime" timestamp NOT NULL DEFAULT now(),
    "is_del" bool NOT NULL DEFAULT false,
    PRIMARY KEY ("id")
);"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE project";
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
