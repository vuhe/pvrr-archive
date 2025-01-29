#![cfg_attr(debug_assertions, allow(dead_code))]
mod web_api;

use anyhow::{Context, Result};
use poem::listener::TcpListener;
use poem::{Route, Server};
use web_api::web_api_route;

#[rustfmt::skip]
pub async fn load() -> Result<()> {
    let app = Route::new()
        .nest("/api", web_api_route());

    Server::new(TcpListener::bind("127.0.0.1:7810"))
        .run(app).await
        .context("服务器启动失败")
}
