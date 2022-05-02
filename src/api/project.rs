use poem::session::Session;
use poem_openapi::OpenApi;

use crate::api::api_tags::ApiTags;

pub struct Project;

#[OpenApi(prefix_path = "/project", tag = "ApiTags::Project")]
impl Project {
    #[oai(path = "/", method = "get")]
    async fn list(&self, _session: &Session) {}
}
