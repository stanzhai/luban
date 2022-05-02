use poem::{error::ResponseError, http::StatusCode, IntoResponse, Response};
use poem_openapi::{
    registry::{MetaResponses, Registry},
    ApiResponse,
};

#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Error(String),
    #[error("{1}")]
    ApiError(u16, String),
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    LdapError(#[from] ldap3::LdapError),
}

impl Error {
    pub fn new(msg: String) -> Self {
        Error::Error(msg)
    }

    pub fn for_api(code: u16, msg: impl Into<String>) -> Self {
        Error::ApiError(code, msg.into())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn to_json(&self) -> serde_json::Value {
        let code = match self {
            Self::ApiError(code, _) => *code,
            _ => crate::server::response::code::API_INTERNAL_ERROR,
        };
        serde_json::json!({
            "code": code,
            "success": false,
            "message": self.to_string(),
        })
    }
}

impl ResponseError for Error {
    fn status(&self) -> StatusCode {
        self.status_code()
    }

    fn as_response(&self) -> Response {
        let mut resp = poem::web::Json(self.to_json()).into_response();
        resp.set_status(self.status());
        resp
    }
}

impl ApiResponse for Error {
    fn meta() -> MetaResponses {
        MetaResponses { responses: Vec::new() }
    }

    fn register(_registry: &mut Registry) {}
}
