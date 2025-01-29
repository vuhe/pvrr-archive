use super::PtClient;
use crate::config::LoginMethod;
use crate::helper::ElementRoot;
use anyhow::{anyhow, ensure, Context, Result};

impl PtClient {
    pub async fn login_test(&self) -> Result<()> {
        let config = self.0.login.as_ref().context("配置文件未配置 login")?;
        let client = self.client().await;
        // todo add more info about login
        let req = match config.method {
            LoginMethod::Cookie | LoginMethod::Get => client.get(config.url.as_str()),
            LoginMethod::Post | LoginMethod::From => client.post(config.url.as_str()),
        };

        let resp = req.send().await.map_err(|e| anyhow!("login_test 请求失败, {e}"))?;
        self.update_cookie(&resp).await;
        let resp = resp.text().await.context("login_test 结果解析 text 失败")?;
        let element = ElementRoot::from_str(&resp, config.resp_type);
        let element = element.map_err(|e| anyhow!("search {e}"))?;
        ensure!(element.get(&config.selector).is_some(), "未找到指定元素，登录失败");
        Ok(())
    }
}
