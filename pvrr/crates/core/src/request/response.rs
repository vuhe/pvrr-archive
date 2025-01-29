use anyhow::{bail, Result};
use bytes::Bytes;
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct SolverResp {
    url: String,
    status: u16,
    headers: HashMap<String, String>,
    response: String,
}

pub enum Resp {
    Default(Response),
    FlareSolver(SolverResp),
}

impl Resp {
    pub(super) async fn new(resp: Response, flare_solver: bool) -> Result<Self> {
        if flare_solver {
            Ok(Self::FlareSolver(resp.json().await?))
        } else {
            Ok(Self::Default(resp))
        }
    }

    pub fn status(&self) -> StatusCode {
        match self {
            Resp::Default(it) => it.status(),
            Resp::FlareSolver(it) => {
                StatusCode::from_u16(it.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    pub fn header(&self, name: &str) -> Option<&str> {
        match self {
            Resp::Default(it) => it.headers().get(name).and_then(|it| it.to_str().ok()),
            Resp::FlareSolver(it) => it.headers.get(name).map(String::as_str),
        }
    }

    pub async fn bytes(self) -> Result<Bytes> {
        match self {
            Resp::Default(it) => Ok(it.bytes().await?),
            Resp::FlareSolver(it) => Ok(Bytes::from(it.response)),
        }
    }

    pub async fn text(self) -> Result<String> {
        match self {
            Resp::Default(it) => Ok(it.text().await?),
            Resp::FlareSolver(it) => Ok(it.response),
        }
    }

    pub async fn json<T: DeserializeOwned>(self) -> Result<T> {
        match self {
            Resp::Default(it) => Ok(it.json().await?),
            Resp::FlareSolver(it) => Ok(serde_json::from_str(&it.response)?),
        }
    }

    pub fn error_for_status(self) -> Result<Self> {
        match self {
            Resp::Default(it) => Ok(Self::Default(it.error_for_status()?)),
            Resp::FlareSolver(it) => {
                let status = StatusCode::from_u16(it.status)?;
                if status.is_client_error() || status.is_server_error() {
                    bail!("HTTP status error ({status}) for url ({})", it.url)
                } else {
                    Ok(Self::FlareSolver(it))
                }
            }
        }
    }
}
