use piot_http::middleware::Middleware;

pub fn middleware() -> Vec<Box<dyn Middleware>> {
    vec![]
}