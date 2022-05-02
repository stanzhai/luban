use poem::session::Session;
use poem_openapi::{payload::Json, OpenApi};

use crate::{
    api::{
        api_tags::ApiTags,
        model::{auth::Auth, UserInfo},
    },
    error::Error,
    server::response::{code, Resp, Response},
    util::ldap,
};

pub struct AuthApi;

#[OpenApi(prefix_path = "/auth", tag = "ApiTags::Auth")]
impl AuthApi {
    #[oai(path = "/login", method = "post")]
    async fn login(&self, auth: Json<Auth>, session: &Session) -> Response {
        let auth = auth.0;
        let check_res = ldap::check_user(auth.username.as_str(), auth.password.as_str()).await;

        let user_info = check_res
            .map_err(|e| format!("登录失败：{:?}", e))
            .and_then(|ldap_user| UserInfo::from_ldap_user(ldap_user).map_err(|e| e.to_owned()))
            .map_err(|e| Error::for_api(code::LOGIN_FAILED, e))?;

        session.set("username", &user_info);
        Resp::ok(user_info).into()
    }

    #[oai(path = "/logout", method = "get")]
    async fn logout(&self, session: &Session) -> Response {
        session.clear();
        Resp::empty().into()
    }
}
