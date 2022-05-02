use poem_openapi::OpenApi;

use crate::api::api_tags::ApiTags;

pub struct Project;

#[OpenApi(prefix_path = "/prototype", tag = "ApiTags::Prototype")]
impl Project {
    #[oai(path = "/", method = "get")]
    async fn test(&self) {}
}
