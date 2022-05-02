use poem_openapi::Object;

#[derive(Object)]
pub struct Auth {
    pub username: String,
    pub password: String,
}
