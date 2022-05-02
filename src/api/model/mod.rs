//! Api接口定义用到的数据模型
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod auth;

#[derive(Deserialize, Serialize)]
pub struct UserInfo {
    pub username: String,
    pub email: String,
    pub display_name: String,
}

impl UserInfo {
    pub fn from_ldap_user(user_info: HashMap<String, Vec<String>>) -> Result<UserInfo, &'static str> {
        let username = user_info
            .get("uid")
            .ok_or_else(|| "ldap用户信息中缺少uid字段")?
            .first()
            .unwrap()
            .to_owned();
        let email = user_info
            .get("mail")
            .ok_or_else(|| "ldap用户信息中缺少mail字段")?
            .first()
            .unwrap()
            .to_owned();
        let display_name = user_info
            .get("displayName")
            .ok_or_else(|| "ldap用户信息中缺少displayName字段")?
            .first()
            .unwrap()
            .to_owned();
        Ok(UserInfo {
            username,
            email,
            display_name,
        })
    }
}
