mod cookies;
mod request;
mod response;

pub(crate) use cookies::cookies_change;
use cookies::COOKIES;
use once_cell::sync::Lazy;
use reqwest::Proxy;
use std::sync::Mutex;
use thiserror::Error;

pub use request::Req;
pub use reqwest::{Method, StatusCode};
pub use response::Resp;

/// 默认 client
static DEFAULT_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .no_proxy()
        .cookie_provider(COOKIES.clone())
        .build()
        .unwrap()
});

/// 代理 client
static PROXY_CLIENT: Lazy<Mutex<reqwest::Client>> = Lazy::new(|| {
    Mutex::new(
        reqwest::Client::builder()
            .cookie_provider(COOKIES.clone())
            .build()
            .unwrap(),
    )
});

/// 代理设置变化
pub(crate) fn proxy_change(url: &str, username: Option<&str>, password: Option<&str>) {
    let mut proxy = match Proxy::all(url) {
        Ok(it) => it,
        Err(_) => return,
    };
    if let (Some(username), Some(password)) = (username, password) {
        proxy = proxy.basic_auth(username, password);
    }
    let client = reqwest::Client::builder()
        .cookie_provider(COOKIES.clone())
        .proxy(proxy)
        .build()
        .unwrap();
    let mut mutex = PROXY_CLIENT.lock().unwrap();
    *mutex = client;
}

/// 默认 client
pub fn direct() -> Client {
    Client {
        client: DEFAULT_CLIENT.clone(),
        flare_solver: false,
    }
}

/// 使用 proxy 的 client，
/// 如果没有设置则默认使用系统代理
pub fn proxy() -> Client {
    Client {
        client: PROXY_CLIENT.lock().unwrap().clone(),
        flare_solver: false,
    }
}

/// flare solver 代理的 client
pub fn flare_solver() -> Client {
    Client {
        client: DEFAULT_CLIENT.clone(),
        flare_solver: true,
    }
}

pub struct Client {
    client: reqwest::Client,
    flare_solver: bool,
}

impl Client {
    pub fn get(&self, url: impl AsRef<str>) -> Req {
        self.request(Method::GET, url)
    }

    pub fn post(&self, url: impl AsRef<str>) -> Req {
        self.request(Method::POST, url)
    }

    pub fn request(&self, method: Method, url: impl AsRef<str>) -> Req {
        if self.flare_solver {
            Req::flare_solver(self.client.clone(), method, url)
        } else {
            Req::default(self.client.clone(), method, url)
        }
    }
}

// only support for build Request, make error cloneable
#[derive(Error, Debug, Clone)]
enum RequestError {
    #[error("FlareSolverr unsupported {0} method request.")]
    FlareSolverUnsupportedMethod(Method),
    #[error("FlareSolverr unsupported {0}.")]
    FlareSolverUnsupportedFunc(String),
    #[error("Url encode fail.")]
    UrlEncodeError(#[from] serde_urlencoded::ser::Error),
}
