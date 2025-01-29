use super::{RequestError, Resp, COOKIES};
use crate::entity::system_config::Model as SystemConfig;
use anyhow::Result;
use reqwest::cookie::CookieStore;
use reqwest::{Client, Method, RequestBuilder, Response, Url};
use serde::Serialize;
use std::fmt::Display;

#[derive(Serialize, Clone)]
struct FlareSolverCookie {
    name: String,
    value: String,
}

impl FlareSolverCookie {
    /// 值应为 "name=value"
    fn new(cookie: &str) -> Option<Self> {
        cookie.trim().split_once("=").map(|(name, value)| Self {
            name: name.to_owned(),
            value: value.to_owned(),
        })
    }
}

#[derive(Serialize, Clone)]
struct FlareSolverParam {
    cmd: &'static str,
    url: String,
    #[serde(skip)]
    query: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    cookies: Vec<FlareSolverCookie>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "postData")]
    post_data: Option<String>,
}

#[derive(Clone)]
pub struct SolverReq {
    client: Client,
    param: Result<FlareSolverParam, RequestError>,
}

impl SolverReq {
    fn unsupported(client: Client, method: Method) -> Self {
        Self {
            client,
            param: Err(RequestError::FlareSolverUnsupportedMethod(method)),
        }
    }

    fn get(client: Client, url: impl AsRef<str>) -> Self {
        Self {
            client,
            param: Ok(FlareSolverParam {
                cmd: "request.get",
                url: url.as_ref().to_owned(),
                query: None,
                cookies: vec![],
                post_data: None,
            }),
        }
    }

    fn post(client: Client, url: impl AsRef<str>) -> Self {
        Self {
            client,
            param: Ok(FlareSolverParam {
                cmd: "request.post",
                url: url.as_ref().to_owned(),
                query: None,
                cookies: vec![],
                post_data: Some(String::default()),
            }),
        }
    }

    fn query<T: Serialize + ?Sized>(mut self, data: &T) -> Self {
        if let Ok(ref mut param) = self.param {
            match serde_urlencoded::to_string(data) {
                Ok(data) => match param.query.as_mut() {
                    None => param.query = Some(data),
                    Some(it) => {
                        it.push_str("&");
                        it.push_str(&data);
                    }
                },
                Err(e) => self.param = Err(e.into()),
            }
        }
        self
    }

    fn post_data<T: Serialize + ?Sized>(mut self, data: &T) -> Self {
        if let Ok(ref mut param) = self.param {
            if param.cmd == "request.post" {
                match serde_urlencoded::to_string(data) {
                    Ok(data) => param.post_data = Some(data),
                    Err(e) => self.param = Err(e.into()),
                }
            }
        }
        self
    }

    fn err(mut self, func: &str) -> Self {
        self.param = Err(RequestError::FlareSolverUnsupportedFunc(func.to_owned()));
        self
    }

    async fn send(self) -> Result<Response> {
        let mut param = self.param?;

        // 尝试设置 cookies
        let url = Url::parse(&param.url)?;
        let cookies = COOKIES.cookies(&url);
        let cookies = cookies.and_then(|it| it.to_str().map(|it| it.to_owned()).ok());
        if let Some(cookies) = cookies {
            param.cookies = cookies
                .split(";")
                .filter_map(FlareSolverCookie::new)
                .collect();
        }

        // 设置 url query
        if let Some(query) = param.query.as_ref() {
            param.url.push_str("?");
            param.url.push_str(query);
        }

        // todo update url from database
        let req = self.client.post("").json(&param);
        Ok(req.send().await?)
    }
}

pub enum Req {
    Default(RequestBuilder),
    FlareSolver(SolverReq),
}

impl Req {
    pub(super) fn default(client: Client, method: Method, url: impl AsRef<str>) -> Self {
        let builder = client.request(method, url.as_ref());
        Self::Default(builder)
    }

    pub(super) fn flare_solver(client: Client, method: Method, url: impl AsRef<str>) -> Self {
        match method {
            Method::GET => Self::FlareSolver(SolverReq::get(client, url)),
            Method::POST => Self::FlareSolver(SolverReq::post(client, url)),
            _ => Self::FlareSolver(SolverReq::unsupported(client, method)),
        }
    }

    pub fn header(self, key: &'static str, value: impl AsRef<str>) -> Self {
        match self {
            Req::Default(it) => Req::Default(it.header(key, value.as_ref())),
            Req::FlareSolver(it) => Req::FlareSolver(it.err("set header")),
        }
    }

    pub fn basic_auth<U: Display, P: Display>(self, username: U, password: Option<P>) -> Self {
        match self {
            Req::Default(it) => Req::Default(it.basic_auth(username, password)),
            Req::FlareSolver(it) => Req::FlareSolver(it.err("set basic auth")),
        }
    }

    pub fn query<T: Serialize + ?Sized>(self, query: &T) -> Self {
        match self {
            Req::Default(it) => Req::Default(it.query(query)),
            Req::FlareSolver(it) => Req::FlareSolver(it.query(query)),
        }
    }

    pub fn form<T: Serialize + ?Sized>(self, form: &T) -> Self {
        match self {
            Req::Default(it) => Req::Default(it.form(form)),
            Req::FlareSolver(it) => Req::FlareSolver(it.post_data(form)),
        }
    }

    pub fn json<T: Serialize + ?Sized>(self, json: &T) -> Self {
        match self {
            Req::Default(it) => Req::Default(it.json(&json)),
            Req::FlareSolver(it) => Req::FlareSolver(it.err("json body")),
        }
    }

    pub async fn send(self) -> Result<Resp> {
        let flare_solver = matches!(self, Req::FlareSolver(_));
        let resp = match self {
            Req::Default(it) => it.send().await?,
            Req::FlareSolver(it) => it.send().await?,
        };
        Resp::new(resp, flare_solver).await
    }
}

impl Clone for Req {
    fn clone(&self) -> Self {
        match self {
            Req::Default(it) => Req::Default(it.try_clone().unwrap()),
            Req::FlareSolver(it) => Req::FlareSolver(it.clone()),
        }
    }
}
