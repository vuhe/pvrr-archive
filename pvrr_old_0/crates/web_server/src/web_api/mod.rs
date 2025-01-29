mod json_resp;

use json_resp::handle_api_404;
use json_resp::JsonResp;
use poem::{get, handler, web::Path, EndpointExt, IntoEndpoint, Route};

pub(crate) fn web_api_route() -> impl IntoEndpoint {
    Route::new().at("/hello/:name", get(hello)).catch_error(handle_api_404)
}

#[handler]
fn hello(Path(name): Path<String>) -> JsonResp<String> {
    JsonResp::ok_with(format!("hello: {name}"))
}
