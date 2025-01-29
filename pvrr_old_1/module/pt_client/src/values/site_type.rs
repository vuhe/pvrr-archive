use serde::Deserialize;
use std::borrow::Cow;

/// 网站类型
#[derive(Deserialize)]
#[serde(try_from = "Cow<'_, str>")]
pub(crate) enum SiteType {
    /// 无需登录
    Public,
    /// 需要登录，但可以公开访问
    SemiPrivate,
    /// 需要登录，无凭证无法访问
    Private,
}

impl TryFrom<Cow<'_, str>> for SiteType {
    type Error = String;

    fn try_from(value: Cow<'_, str>) -> Result<Self, Self::Error> {
        match &value {
            _ if value.eq_ignore_ascii_case("public") => Ok(Self::Public),
            _ if value.eq_ignore_ascii_case("semi-private") => Ok(Self::SemiPrivate),
            _ if value.eq_ignore_ascii_case("SemiPrivate") => Ok(Self::SemiPrivate),
            _ if value.eq_ignore_ascii_case("private") => Ok(Self::Private),
            _ => Err(format!("不支持 {value}, 请在 public, semi-private, private 中选择")),
        }
    }
}
