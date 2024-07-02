use piot_http::http::HttpContext;
use piot_http::routing::Route;

struct TestRoute {}

impl Route for TestRoute {
    fn is_match(&self, http_context: &HttpContext) -> bool {
        todo!()
    }

    fn handler(&self, http_context: HttpContext) -> Result<HttpContext, (HttpContext, &'static str)> {
        todo!()
    }
}