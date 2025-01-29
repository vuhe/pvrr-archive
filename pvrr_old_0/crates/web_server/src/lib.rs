#![cfg_attr(debug_assertions, allow(dead_code))]
mod web_api;

use crate::web_api::web_api_route;
use base_tool::error::{AnyContext, AnyResult};
use poem::listener::TcpListener;
use poem::{Route, Server};

pub async fn load() -> AnyResult {
    let app = Route::new().nest("/api", web_api_route());

    Server::new(TcpListener::bind("127.0.0.1:7810")).run(app).await.context("服务器启动失败")
}
