use super::PtClient;
use std::sync::Arc;
use anyhow::Result;
use http_client::Req;

impl PtClient {
    pub(super) async fn build_req(&self) -> Result<Req> {
        todo!()
    }

    pub(super) async fn set_proxy(&self, builder: ClientBuilder) -> ClientBuilder {
        // todo search proxy config from database
        builder.no_proxy()
    }

    pub(super) async fn set_cookie(&self, builder: ClientBuilder) -> ClientBuilder {
        // todo search cookie from database
        let cookie = String::new();
        let cookie_store = Arc::new(Jar::default());
        cookie_store.add_cookie_str(&cookie, &self.0.domain_url);
        builder.cookie_store(true).cookie_provider(cookie_store)
    }

    pub(super) async fn update_cookie(&self, resp: &Response) {
        let cookie = resp
            .cookies()
            .map(|it| format!("{}={}", it.name(), it.value()))
            .reduce(|acc, it| acc + "; " + &it);
        if let Some(_cookie) = cookie {
            // todo save cookie to database
        }
    }
}
