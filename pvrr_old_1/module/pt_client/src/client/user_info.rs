use super::PtClient;
use crate::helper::ElementRoot;
use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;

pub struct UserInfo {
    uid: u64,
}

impl UserInfo {
    fn from(_map: HashMap<&'_ str, String>) -> Result<Self> {
        todo!()
    }
}

impl PtClient {
    pub async fn user_info(&self) -> Result<UserInfo> {
        let config = self.0.userinfo.as_ref().context("配置文件未配置 userinfo")?;
        let req = self.client().await.get(config.url.as_str());
        let resp = req.send().await.context("userinfo 请求失败")?;
        self.update_cookie(&resp).await;
        let resp = resp.text().await.context("userinfo 结果解析 text 失败")?;

        let element = ElementRoot::from_str(&resp, config.resp_type);
        let element = element.map_err(|e| anyhow!("userinfo {e}"))?;
        let root = element.root();
        let row = element.get(&config.selector);
        let result = config.fields.parse(root, row);
        let result = result.map_err(|e| anyhow!("userinfo 结果解析失败, {e}"))?;
        UserInfo::from(result)
    }
}
