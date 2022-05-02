use poem_openapi::payload::Json;
use serde::Serialize;
use serde_json::{json, Value};

use crate::error::Error;

pub mod code {
    pub const SUCCESS: u16 = 0;
    pub const LOGIN_FAILED: u16 = 1;
    pub const API_INTERNAL_ERROR: u16 = 2;
}

pub type Response = Result<Json<Resp>, Error>;

#[derive(serde::Serialize)]
pub struct Resp {
    pub code: u16,
    pub message: Option<String>,
    pub success: bool,
    pub payload: Value,
}

impl Resp {
    #[inline]
    pub const fn new(code: u16, payload: Value, msg: Option<String>) -> Self {
        Self {
            code,
            message: msg,
            success: code == code::SUCCESS,
            payload,
        }
    }

    #[inline]
    pub const fn empty() -> Self {
        Resp::new(code::SUCCESS, Value::Null, None)
    }

    #[inline]
    pub fn ok(data: impl Serialize) -> Self {
        let payload = json!(data);
        Self::new(code::SUCCESS, payload, None)
    }

    #[inline]
    pub fn error(code: u16, msg: &str) -> Self {
        Self::new(code, Value::Null, Some(msg.to_string()))
    }
}

impl From<Resp> for Response {
    fn from(val: Resp) -> Self {
        Ok(Json(val))
    }
}

impl poem_openapi::types::Type for Resp {
    const IS_REQUIRED: bool = true;
    type RawValueType = Self;
    type RawElementValueType = Self;

    fn name() -> std::borrow::Cow<'static, str> {
        "Resp".into()
    }

    fn schema_ref() -> poem_openapi::registry::MetaSchemaRef {
        poem_openapi::registry::MetaSchemaRef::Inline(Box::new(poem_openapi::registry::MetaSchema {
            title: Some("Resp".to_owned()),
            default: Some(serde_json::json!(Self::empty())),
            description: Some("general response of web api"),
            ..poem_openapi::registry::MetaSchema::new("object")
        }))
    }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(self)
    }

    fn raw_element_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::RawElementValueType> + 'a> {
        Box::new(self.as_raw_value().into_iter())
    }
}

impl poem_openapi::types::ToJSON for Resp {
    fn to_json(&self) -> Option<Value> {
        Some(serde_json::json!(self))
    }
}
