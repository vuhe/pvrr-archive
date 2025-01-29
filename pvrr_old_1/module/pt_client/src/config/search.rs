use anyhow::{bail, ensure, Context as Ctx, Result};
use reqwest::header::REFERER;
use reqwest::{Client, Method, RequestBuilder};
use serde_json::Value as JsonVal;
use serde_yaml::Value;
use tera::{Context, Tera};

struct ReqConfig {
    /// 查询网页路径
    url: String,
    /// 搜索方法
    method: Method,
    /// 用于网页查询类型
    categories: JsonVal,
}

impl ReqConfig {
    fn try_from(value: &Value, domain: &String) -> Result<Self> {
        let path = value.get("path").context("paths.path 为必须配置")?;
        let path = path.as_str().context("paths.path 应为 string 类型")?;
        let url = domain.clone() + path;

        let method = match value.get("method") {
            None => Method::GET,
            Some(it) => match it {
                Value::String(it) if it.eq_ignore_ascii_case("get") => Method::GET,
                Value::String(it) if it.eq_ignore_ascii_case("post") => Method::POST,
                Value::String(it) => bail!("paths.method 不支持 {it}, 请在 get, post 中选择"),
                _ => bail!("paths.method 应为 string 类型"),
            },
        };

        let categories = match value.get("categories") {
            None => JsonVal::Array(vec![]),
            Some(it) => {
                let it = it.as_sequence().context("paths.categories 应为 list 类型")?;
                it.into_iter().map(|it| serde_json::to_value(it).unwrap()).collect()
            },
        };

        Ok(Self { url, method, categories })
    }
}

pub(crate) struct SearchConfig {
    reqs: Vec<ReqConfig>,
    query: Tera,
}

impl SearchConfig {
    pub(super) fn try_from(value: &Value, domain: &String) -> Result<Self> {
        let paths = value.get("paths").context("paths 为必须配置")?;
        let paths = paths.as_sequence().context("paths 应为 list 类型")?;
        let mut reqs = vec![];
        for path in paths {
            reqs.push(ReqConfig::try_from(path, domain)?);
        }
        ensure!(!reqs.is_empty(), "paths 配置应该至少有一项");

        let mut query = Tera::default();
        if let Some(q) = value.get("query") {
            let mut vec = vec![];
            for (key, value) in q.as_mapping().context("query 应为 mapping 类型")? {
                let key = key.as_str().unwrap();
                let value = serde_yaml::to_string(value).unwrap();
                vec.push((key, value));
            }
            query.add_raw_templates(vec).context("query 模版解析失败")?;
        }

        Ok(Self { reqs, query })
    }

    /// 构建多个搜索
    pub(crate) fn build_reqs(&self, client: &Client, ctx: Context) -> Vec<RequestBuilder> {
        let mut reqs = Vec::with_capacity(self.reqs.len());
        for config in self.reqs.iter() {
            let mut ctx = ctx.clone();
            ctx.insert("categories", &config.categories);
            let args: Vec<(&str, String)> = self
                .query
                .get_template_names()
                .map(|it| (it, self.query.render(it, &ctx)))
                .filter(|(_, it)| it.is_ok())
                .map(|it| (it.0, it.1.unwrap()))
                .collect();
            let req = client
                .request(config.method.clone(), &config.url)
                .header(REFERER, &config.url)
                .query(&args);
            reqs.push(req);
        }
        reqs
    }
}
