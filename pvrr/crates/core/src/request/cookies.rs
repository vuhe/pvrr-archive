use bytes::Bytes;
use cookie::Cookie;
use cookie_store::CookieStore;
use once_cell::sync::Lazy;
use reqwest::header::HeaderValue;
use reqwest::Url;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// 全局 cookies
pub(super) static COOKIES: Lazy<Arc<Jar>> = Lazy::new(|| Arc::new(Jar::default()));

type CustomCookie = (Box<str>, Box<str>);
type CustomCookieList = Arc<[CustomCookie]>;

/// 转换单条 cookie
fn covert_to_cookie(cookie: &str) -> Option<CustomCookie> {
    cookie
        .split_once("=")
        .map(|(key, value)| (key.into(), value.into()))
}

/// 转换全部 cookies
fn covert_cookies(cookies: &str) -> CustomCookieList {
    cookies
        .split(";")
        .map(|it| it.trim())
        .filter_map(covert_to_cookie)
        .collect()
}

/// cookies 设置变化
pub(crate) fn cookies_change(url: &str, cookies: &str) {
    let url = match Url::parse(url) {
        Ok(it) => it,
        Err(_) => todo!("print error log"),
    };
    let domain = match url.domain() {
        None => todo!("print error log"),
        Some(it) => it,
    };
    let cookies = covert_cookies(cookies);
    let mut store = COOKIES.0.write().unwrap();
    store.custom.insert(domain.into(), cookies);
}

#[derive(Default)]
struct MixCookieStore {
    store: CookieStore,
    custom: HashMap<Box<str>, CustomCookieList>,
}

#[derive(Default)]
pub(super) struct Jar(RwLock<MixCookieStore>);

impl Jar {
    fn custom_cookies(&self, domain: &str) -> Option<CustomCookieList> {
        let lock = self.0.read().unwrap();
        let cookies = lock.custom.get(domain);
        cookies.map(|it| it.clone())
    }
}

fn build_resp_cookie(value: &HeaderValue) -> Option<Cookie<'static>> {
    std::str::from_utf8(value.as_bytes())
        .ok()
        .and_then(|it| Cookie::parse(it).ok())
        .map(Cookie::into_owned)
}

impl reqwest::cookie::CookieStore for Jar {
    #[rustfmt::skip]
    fn set_cookies(&self, cookies: &mut dyn Iterator<Item = &HeaderValue>, url: &Url) {
        let iter = cookies.filter_map(build_resp_cookie);
        self.0.write().unwrap().store.store_response_cookies(iter, url);
    }

    fn cookies(&self, url: &Url) -> Option<HeaderValue> {
        let mut map = HashMap::new();

        // 设置中自定义的 cookies，优先级低于原生 cookies
        let custom = url.domain().and_then(|it| self.custom_cookies(it));
        if let Some(cookies) = custom.as_ref() {
            for (key, value) in cookies.as_ref() {
                map.insert(key.as_ref(), value.as_ref());
            }
        }

        // 通过客户端请求获得的 cookies，会覆盖自定义 cookies
        let cookies = {
            let store = self.0.read().unwrap();
            for (key, value) in store.store.get_request_values(url) {
                map.insert(key, value);
            }
            map.into_iter()
                .map(|(key, value)| format!("{key}={value}"))
                .collect::<Vec<_>>()
                .join("; ")
        };

        if cookies.is_empty() {
            return None;
        }

        HeaderValue::from_maybe_shared(Bytes::from(cookies)).ok()
    }
}
