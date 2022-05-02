use std::collections::HashMap;

use ldap3::{result::Result, LdapConnAsync, Scope, SearchEntry};
use crate::config;

/// 验证ldap用户登录信息，如果有效，返回用户详情
pub async fn check_user(username: &str, password: &str) -> Result<HashMap<String, Vec<String>>> {
    let config = config::Config::global();
    let (conn, mut ldap) = LdapConnAsync::new(config.ldap_server.as_str()).await?;
    ldap3::drive!(conn);

    let bind_dn = config.ldap_base.to_owned().replace("*", username);
    ldap.simple_bind(bind_dn.as_str(), password).await?.success()?;

    let (mut rs, _res) = ldap
        .search(bind_dn.as_str(), Scope::Base, "(objectClass=*)", vec!["*"])
        .await?
        .success()?;

    ldap.unbind().await?;

    let entry = rs.pop().unwrap();
    let entry = SearchEntry::construct(entry);

    Ok(entry.attrs)
}
