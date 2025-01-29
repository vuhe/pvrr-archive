#![cfg_attr(debug_assertions, allow(dead_code))]
mod query_type;
mod request;
mod body_type;
mod response;

pub use request::Req;
pub use response::Resp;
