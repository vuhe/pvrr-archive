#![cfg_attr(debug_assertions, allow(dead_code))]
mod client;
mod config;
mod fields;
mod filter;
mod values;

pub use client::PtClient;
use config::SiteConfig;
use once_cell::sync::Lazy;
use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

static CONFIGS: Lazy<Vec<Arc<SiteConfig>>> = Lazy::new(|| {
    let data_path = match env::var("DATA_PATH") {
        Ok(it) => it,
        Err(_) => {
            log::warn!("缺失环境变量 DATA_PATH，跳过站点配置加载");
            return vec![];
        },
    };
    let data_path = PathBuf::from(data_path);
    let site_config_path = data_path.join("site");
    let dir = match site_config_path.read_dir() {
        Ok(it) => it,
        Err(e) => {
            log::debug!("读取站点配置文件夹错误: {:?}", e);
            log::warn!("读取站点配置文件夹错误，跳过站点配置加载");
            return vec![];
        },
    };
    let mut configs = vec![];
    for file in dir {
        let path = match file {
            Ok(it) => it,
            Err(e) => {
                log::debug!("读取站点配置文件错误: {:?}", e);
                log::warn!("读取站点配置文件错误，跳过加载");
                continue;
            },
        };
        let filename = path.file_name();
        let path = path.path();
        if path.is_file() {
            let file = match File::open(path) {
                Ok(it) => it,
                Err(e) => {
                    log::debug!("读取站点配置文件 {:?} 错误: {:?}", filename, e);
                    log::warn!("读取站点配置文件 {:?} 错误，跳过加载", filename);
                    continue;
                },
            };
            match serde_yaml::from_reader(file) {
                Ok(it) => configs.push(Arc::new(it)),
                Err(e) => {
                    log::warn!("读取站点配置文件 {:?} 错误，跳过加载，原因：{e}", filename);
                    continue;
                },
            };
        }
    }
    configs
});

pub fn load() {
    Lazy::force(&CONFIGS);
}
