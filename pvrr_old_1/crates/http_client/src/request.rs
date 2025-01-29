use crate::response::Resp;
use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use reqwest::{Client, Proxy};

static DEFAULT_CLIENT: Lazy<Client> = Lazy::new(|| Client::default());

#[derive(Copy, Clone)]
pub enum ReqMethod {
    Get,
    Post,
}

pub struct Req {
    url: String,
    method: ReqMethod,
    cookie: Option<String>,
    proxy: Option<String>,
}

impl Req {
    pub fn new(url: &str) -> Self {
        Self { url: url.to_string(), method: ReqMethod::Get, cookie: None, proxy: None }
    }

    pub fn proxy(mut self, proxy: String) -> Self {
        self.proxy = Some(proxy);
        self
    }

    pub fn cookie(mut self, cookie: String) -> Self {
        self.cookie = Some(cookie);
        self
    }

    pub async fn send(self) -> Result<Resp> {
        let client = if self.proxy.is_some() || self.cookie.is_some() {
            let mut client = Client::builder();
            if let Some(proxy) = self.proxy {
                let proxy = Proxy::all(proxy).context("")?;
                client = client.proxy(proxy);
            }
            if let Some(_cookie) = self.cookie {
                todo!()
            }
            client.build().context("")?
        } else {
            DEFAULT_CLIENT.clone()
        };

        let req = match self.method {
            ReqMethod::Get => client.get(self.url),
            ReqMethod::Post => client.post(self.url),
        };

        req.send().await.map(|it| Resp(it)).context("")
    }
}
