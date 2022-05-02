use std::env;

use once_cell::sync::OnceCell;

/// 系统全局配置
#[derive(Debug)]
pub struct Config {
    /// PostgreSQL 数据库连接地址
    pub db_url: String,
    /// Web服务器端口
    pub http_port: String,
    /// 对外开放的域名+端口
    pub domain: String,
    /// LDAP服务器地址
    pub ldap_server: String,
    /// LDAP认证base dn
    pub ldap_base: String,
}
static INSTANCE: OnceCell<Config> = OnceCell::new();

impl Config {
    pub fn global() -> &'static Config {
        INSTANCE.get().expect("config is not initialized")
    }

    pub fn init_from_dotenv() {
        // get env vars
        dotenv::dotenv().ok();
        let db_url = env::var("DB_URL").expect("DB_URL is not set in .env file");
        let http_port = env::var("HTTP_PORT").expect("HTTP_PORT is not set in .env file");
        let domain = env::var("DOMAIN").expect("DOMAIN is not set in .env file");
        let ldap_server = env::var("LDAP_SERVER").expect("LDAP_SERVER is not set in .env file");
        let ldap_base = env::var("LDAP_BASE").expect("LDAP_BASE is not set in .env file");
        INSTANCE
            .set(Config {
                db_url,
                http_port,
                domain,
                ldap_server,
                ldap_base,
            })
            .unwrap()
    }
}
