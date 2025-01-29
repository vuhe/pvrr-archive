use super::RespBodyType;
use crate::helper::Selector;
use anyhow::{anyhow, bail, Context, Result};
use serde_yaml::Value;

/// 登录方法类型
pub(crate) enum LoginMethod {
    Cookie,
    Post,
    Get,
    From,
}

impl LoginMethod {
    fn from(value: &Value) -> Result<Self> {
        // todo add more method
        match value {
            Value::String(it) if it.eq_ignore_ascii_case("cookie") => Ok(Self::Cookie),
            Value::String(_) => bail!("method 目前仅支持 cookie"),
            _ => bail!("method 应为 string 类型"),
        }
    }
}

/// 登录配置
pub(crate) struct LoginConfig {
    /// 登录方法，默认使用 cookie
    pub(crate) method: LoginMethod,
    /// 测试访问路径
    pub(crate) url: String,
    /// 测试访问选择器
    pub(crate) selector: Selector,
    /// 测试访问返回值类型，默认使用 html_old
    pub(crate) resp_type: RespBodyType,
}

impl LoginConfig {
    pub(super) fn try_from(value: &Value, domain: &String) -> Result<Self> {
        let method = match value.get("method") {
            None => LoginMethod::Cookie,
            Some(it) => LoginMethod::from(it)?,
        };

        let path = value.get("path").context("path 为必须配置")?;
        let path = path.as_str().context("path 应为 string 类型")?;
        let url = domain.clone() + path;

        let resp_type = match value.get("resp_type") {
            None => RespBodyType::HTML,
            Some(it) => RespBodyType::from(it)?,
        };

        let selector = value.get("selector").context("selector 为必须配置")?;
        let selector = selector.as_str().context("selector 应为 string 类型")?;
        let selector = Selector::try_from(selector, resp_type);
        let selector = selector.map_err(|e| anyhow!("selector {e}"))?;

        Ok(Self { method, url, selector, resp_type })
    }
}
