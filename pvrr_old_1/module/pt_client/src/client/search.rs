use super::PtClient;
use anyhow::{anyhow, Context as Ctx, Result};
use rand::Rng;
use reqwest::RequestBuilder;
use std::collections::HashMap;
use tera::Context;
use tokio::time::{sleep, Duration};
use torrent::Torrent;

impl PtClient {
    pub async fn search(&self, key_word: Option<&str>) -> Vec<Torrent> {
        let client = self.client().await;
        let mut ctx = Context::new();
        if let Some(key_word) = key_word {
            ctx.insert("key_word", key_word);
        }
        let reqs = self.0.search.build_reqs(&client, ctx);

        let mut torrents = vec![];
        for req in reqs {
            match self.search_one(req).await {
                Ok(mut it) => torrents.append(&mut it),
                Err(e) => log::warn!("{} {e}, 跳过", self.config_name()),
            }
        }
        torrents
    }

    async fn search_one(&self, req: RequestBuilder) -> Result<Vec<Torrent>> {
        // 多页面搜索随机延迟
        let random_time = rand::thread_rng().gen_range(2..6);
        let duration = Duration::from_secs(random_time);
        sleep(duration).await;

        let resp = req.send().await.map_err(|e| anyhow!("search 请求失败 {e}"))?;
        self.update_cookie(&resp).await;
        let resp = resp.text().await.context("search 结果解析 text 失败")?;
        let rows = self.parse_element(&resp)?;

        let mut torrents = Vec::with_capacity(rows.len());
        for result in rows {
            let content = self.download_torrent(result["download"].as_str()).await?;
            match parse_torrent(result, content) {
                Ok(it) => torrents.push(it),
                Err(e) => log::warn!("{} torrent 解析失败: {e}", self.config_name()),
            };
        }

        Ok(torrents)
    }

    fn parse_element(&self, text: &str) -> Result<Vec<HashMap<&str, String>>> {
        let resp_type = self.0.torrents.resp_type;
        let element = ElementRoot::from_str(text, resp_type).map_err(|e| anyhow!("search {e}"))?;
        let selector = &self.0.torrents.selector;
        let root = element.root();

        let mut vec = vec![];
        for row in element.get_all(selector) {
            let map = self.0.torrents.fields.parse(root, row);
            let map = map.map_err(|e| anyhow!("torrent 解析失败, {e}"))?;
            vec.push(map);
        }
        Ok(vec)
    }
}

fn parse_torrent(map: HashMap<&str, String>, content: Vec<u8>) -> Result<Torrent> {
    let mut builder = Torrent::builder().set_name(&map["title"]);
    builder = builder.set_normal_date(&map["pub_date"]);
    if let Ok(peers) = map["peers"].parse() {
        builder = builder.set_peers(peers);
    }
    if let Ok(seeds) = map["seeds"].parse() {
        builder = builder.set_seeds(seeds);
    }
    if let Ok(download_volume_factor) = map["download_volume_factor"].parse() {
        builder = builder.set_download_volume_factor(download_volume_factor)
    }
    if let Ok(upload_volume_factor) = map["upload_volume_factor"].parse() {
        builder = builder.set_upload_volume_factor(upload_volume_factor)
    }
    builder = builder.set_content(content)?;
    Ok(builder.build())
}
