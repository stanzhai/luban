use poem::{
    endpoint::StaticFilesEndpoint,
    http::{header, StatusCode},
    Endpoint, IntoResponse, Request, Response,
};

pub struct Domain<T> {
    /// 主网站域名
    domain: String,
    inner: T,
}

impl<E: Endpoint> Domain<E> {
    pub fn new(domain: String, ep: E) -> Self {
        Domain { domain, inner: ep }
    }
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for Domain<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> poem::error::Result<Self::Output> {
        if let Some(host) = req.header(header::HOST) {
            if host.eq(self.domain.as_str()) {
                Ok(self.inner.call(req).await?.into_response())
            } else if host.contains(self.domain.as_str()) {
                // TODO session校验
                let v: Vec<&str> = host.split('.').collect();
                let folder = v.first().unwrap();
                let static_endpoint = StaticFilesEndpoint::new(folder).index_file("index.html");
                Ok(static_endpoint.call(req).await?.into_response())
            } else {
                let res = Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(format!("Request host must be: {}", self.domain));
                Ok(res)
            }
        } else {
            let res = StatusCode::NOT_ACCEPTABLE.into_response();
            Ok(res)
        }
    }
}
