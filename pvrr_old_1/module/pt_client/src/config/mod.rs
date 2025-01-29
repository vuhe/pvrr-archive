mod download;
mod login;
mod search;
mod torrents;
mod userinfo;

use anyhow::{anyhow, bail, ensure, Context, Result};
use download::DownloadConfig;
use login::LoginConfig;
pub(crate) use login::LoginMethod;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::Url;
use search::SearchConfig;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use serde_yaml::Value;
use torrents::TorrentConfig;
use userinfo::UserinfoConfig;

/// 网站返回值类型
#[derive(Copy, Clone)]
pub(crate) enum RespBodyType {
    HTML,
    JSON,
    XML,
}

impl RespBodyType {
    pub(crate) fn from(value: &Value) -> Result<Self> {
        match value {
            Value::String(it) if it.eq_ignore_ascii_case("html_old") => Ok(Self::HTML),
            Value::String(it) if it.eq_ignore_ascii_case("json_old") => Ok(Self::JSON),
            Value::String(it) if it.eq_ignore_ascii_case("xml_old") => Ok(Self::XML),
            Value::String(it) => bail!("resp_type 不支持 {it}, 请在 html_old, json_old, xml_old 中选择"),
            _ => bail!("resp_type 应为 string 类型"),
        }
    }
}

/// 网站类型
pub(crate) enum SiteType {
    /// 无需登录
    Public,
    /// 需要登录，但可以公开访问
    SemiPrivate,
    /// 需要登录，无凭证无法访问
    Private,
}

impl SiteType {
    fn from_str(s: &str) -> Result<Self> {
        match s {
            _ if s.eq_ignore_ascii_case("public") => Ok(Self::Public),
            _ if s.eq_ignore_ascii_case("semi-private") => Ok(Self::SemiPrivate),
            _ if s.eq_ignore_ascii_case("SemiPrivate") => Ok(Self::SemiPrivate),
            _ if s.eq_ignore_ascii_case("private") => Ok(Self::Private),
            _ => bail!("不支持 {s}, 请在 public, semi-private, private 中选择"),
        }
    }
}

/// 站点配置
pub(crate) struct SiteConfig {
    /// 索引器的内部名称
    pub(crate) id: String,
    /// 显示名称
    pub(crate) name: String,
    /// 索引器类型
    pub(crate) site_type: SiteType,
    /// 网站页面编码
    pub(crate) encoding: String,
    /// 网站域名
    pub(crate) domain: String,
    /// 网站域名 url 格式
    pub(crate) domain_url: Url,
    /// 网站登录信息
    pub(crate) login: Option<LoginConfig>,
    /// 站点用户信息
    pub(crate) userinfo: Option<UserinfoConfig>,
    /// 网站搜索设置
    pub(crate) search: SearchConfig,
    /// torrent 下载配置
    pub(crate) download: DownloadConfig,
    /// torrent 解析配置
    pub(crate) torrents: TorrentConfig,
}

static URL_CHECK: Lazy<Regex> = Lazy::new(|| Regex::new("^https?://.*/$").unwrap());

impl SiteConfig {
    fn try_from(value: &Value) -> Result<Self> {
        let id = value.get("id").context("id 为必须配置")?;
        let id = id.as_str().context("id 应为 string 类型")?.to_owned();

        let name = value.get("name").context("name 为必须配置")?;
        let name = name.as_str().context("name 应为 string 类型")?.to_owned();

        let site_type = value.get("type").context("type 为必须配置")?;
        let site_type = site_type.as_str().context("type 应为 string 类型")?;
        let site_type = SiteType::from_str(site_type).map_err(|e| anyhow!("type {e}"))?;

        let encoding = value.get("encoding").context("encoding 为必须配置")?;
        let encoding = encoding.as_str().context("name 应为 string 类型")?.to_owned();

        let domain = value.get("domain").context("domain 为必须配置")?;
        let domain = domain.as_str().context("name 应为 string 类型")?.to_owned();
        ensure!(URL_CHECK.is_match(domain.as_str()), "domain 应该以 http(s):// 开头, 以 / 结尾");
        let domain_url = domain.parse().context("domain 解析为 url 错误")?;

        let login = match &site_type {
            SiteType::Public => None,
            SiteType::SemiPrivate => {
                value.get("login").and_then(|it| LoginConfig::try_from(it, &domain).ok())
            },
            SiteType::Private => {
                let login = value.get("login").context("login 为必须配置")?;
                Some(LoginConfig::try_from(login, &domain).map_err(|e| anyhow!("login.{e}"))?)
            },
        };

        let userinfo = match &site_type {
            SiteType::Public => None,
            SiteType::SemiPrivate => {
                value.get("userinfo").and_then(|it| UserinfoConfig::try_from(it, &domain).ok())
            },
            SiteType::Private => {
                let userinfo = value.get("userinfo").context("userinfo 为必须配置")?;
                Some(
                    UserinfoConfig::try_from(userinfo, &domain)
                        .map_err(|e| anyhow!("userinfo.{e}"))?,
                )
            },
        };

        let search = value.get("search").context("search 为必须配置")?;
        let search = SearchConfig::try_from(search, &domain).map_err(|e| anyhow!("search.{e}"))?;

        let download = value.get("download");
        let download = match download {
            None => DownloadConfig::default(),
            Some(it) => DownloadConfig::try_from(it).map_err(|e| anyhow!("download.{e}"))?,
        };

        let torrents = value.get("torrents").context("torrents 为必须配置")?;
        let torrents = TorrentConfig::try_from(torrents).map_err(|e| anyhow!("torrents.{e}"))?;

        Ok(Self {
            id,
            name,
            site_type,
            encoding,
            domain,
            domain_url,
            login,
            userinfo,
            search,
            download,
            torrents,
        })
    }
}

impl<'de> Deserialize<'de> for SiteConfig {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_yaml::Value::deserialize(deserializer).unwrap();
        Self::try_from(&value).map_err(D::Error::custom)
    }
}
