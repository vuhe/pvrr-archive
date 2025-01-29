use base_tool::once_cell::Lazy;
use base_tool::text::Text;
use database::entity::SystemConfig;
use reqwest::{Client, RequestBuilder};

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

pub(crate) struct HttpClient;

impl HttpClient {
    async fn fanart_url(&self) -> Text {
        // todo log print waring with db err
        SystemConfig::get("fanart_url")
            .await
            .ok()
            .filter(|it| it.is_not_empty())
            .unwrap_or(Text::from("https://webservice.fanart.tv"))
    }

    async fn tmdb_url(&self) -> Text {
        // todo log print waring with db err
        SystemConfig::get("tmdb_url")
            .await
            .ok()
            .filter(|it| it.is_not_empty())
            .unwrap_or(Text::from("https://api.themoviedb.org/3"))
    }

    async fn fanart_api_key(&self) -> Option<Text> {
        SystemConfig::get("fanart_api_key").await.ok()
    }

    async fn tmdb_api_key(&self) -> Option<Text> {
        SystemConfig::get("tmdb_api_key").await.ok()
    }

    async fn douban_cookie(&self) -> Option<Text> {
        SystemConfig::get("douban_cookie").await.ok()
    }
}

impl HttpClient {
    pub(crate) async fn fanart_query(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.fanart_url().await, path);
        let mut req = HTTP_CLIENT.get(url);
        if let Some(api_key) = self.fanart_api_key().await {
            req = req.query(&[("api_key", &*api_key)]);
        }
        return req;
    }

    pub(crate) async fn tmdb_query(&self, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.tmdb_url().await, path);
        let mut req = HTTP_CLIENT.get(url);
        if let Some(api_key) = self.tmdb_api_key().await {
            req = req.query(&[("api_key", &*api_key)]);
        }
        return req;
    }
}
