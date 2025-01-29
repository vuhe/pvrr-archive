use poem::error::NotFoundError;
use poem::http::StatusCode;
use poem::web::Json;
use poem::{Body, IntoResponse, Response};
use serde::Serialize;

pub(super) async fn handle_api_404(_e: NotFoundError) -> Response {
    let json: JsonResp<()> = JsonResp { status: 404, message: "not found".to_owned(), data: None };
    let body = Body::from_json(json).unwrap();
    Response::builder().status(StatusCode::OK).body(body)
}

#[derive(Serialize)]
pub(super) struct JsonResp<T> {
    status: u16,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T> JsonResp<T> {
    pub(super) fn ok() -> JsonResp<()> {
        JsonResp { status: 200, message: "ok".to_string(), data: None }
    }

    pub(super) fn err() -> JsonResp<()> {
        JsonResp { status: 500, message: "服务器异常".to_string(), data: None }
    }

    pub(super) fn err_with(message: &str) -> JsonResp<T> {
        JsonResp { status: 500, message: message.to_string(), data: None }
    }
}

impl<T: Serialize + Send> JsonResp<T> {
    pub(super) fn ok_with(data: T) -> JsonResp<T> {
        JsonResp { status: 200, message: "ok".to_string(), data: Some(data) }
    }
}

impl<T: Serialize + Send> IntoResponse for JsonResp<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
