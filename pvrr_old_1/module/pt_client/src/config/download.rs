use anyhow::{bail, Context as Ctx, Result};
use once_cell::sync::Lazy;
use reqwest::header::CONTENT_TYPE;
use reqwest::{Method, RequestBuilder};
use serde_yaml::Value;
use tera::{Context, Tera};

static EMPTY_CONTEXT: Lazy<Context> = Lazy::new(|| Context::new());

pub(crate) struct DownloadConfig {
    pub(crate) method: Method,
    content_type: Option<String>,
    args: Option<Tera>,
}

impl DownloadConfig {
    pub(super) fn try_from(value: &Value) -> Result<Self> {
        let method = match value.get("method") {
            None => Method::GET,
            Some(it) => match it {
                Value::String(it) if it.eq_ignore_ascii_case("get") => Method::GET,
                Value::String(it) if it.eq_ignore_ascii_case("post") => Method::POST,
                Value::String(it) => bail!("method 不支持 {it}, 请在 get, post 中选择"),
                _ => bail!("method 应为 string 类型"),
            },
        };

        let content_type = match value.get("content_type") {
            None => None,
            Some(Value::String(it)) => Some(it.clone()),
            _ => bail!("content_type 应为 string 类型"),
        };

        let args = match value.get("args") {
            None => None,
            Some(Value::Sequence(it)) => {
                let mut vec = vec![];
                for v in it.iter() {
                    let name = v.get("name").context("args.name 为必须配置")?;
                    let name = name.as_str().context("args.name 应为 string 类型")?;
                    let val = v.get("value").context("args.value 为必须配置")?;
                    let val = match val {
                        Value::String(it) => it.clone(),
                        Value::Bool(it) => it.to_string(),
                        Value::Number(it) => it.to_string(),
                        _ => bail!("args.value 应为 string 或 bool 或 number 类型"),
                    };
                    vec.push((name, val));
                }
                let mut tera = Tera::default();
                tera.add_raw_templates(vec).context("args 模版解析失败")?;
                Some(tera)
            },
            _ => bail!("args 应为 list 类型"),
        };

        Ok(Self { method, content_type, args })
    }

    pub(crate) fn build_reqs(&self, mut req: RequestBuilder) -> RequestBuilder {
        if let Some(ref content_type) = self.content_type {
            req = req.header(CONTENT_TYPE, content_type);
        }
        if let Some(ref args) = self.args {
            if self.method == Method::POST {
                let args: Vec<(&str, String)> = args
                    .get_template_names()
                    .map(|it| (it, args.render(it, &EMPTY_CONTEXT)))
                    .filter(|(_, it)| it.is_ok())
                    .map(|it| (it.0, it.1.unwrap()))
                    .collect();
                req = req.query(&args);
            }
        }
        req
    }
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self { method: Method::GET, content_type: None, args: None }
    }
}
