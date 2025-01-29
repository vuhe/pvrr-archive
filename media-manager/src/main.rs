mod test;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use crate::test::RssTag;

#[tokio::main]
async fn main() {
    // // 加载环境变量
    // dotenv::dotenv().ok();
    // let data_path_str = env::var("DATA_PATH")
    //     .expect("Can't load environment variable DATA_PATH!");
    // let data_path = Path::new(&data_path_str);
    //
    // // 加载系统配置
    // env_config::load_config(data_path);
    //
    // let num: f64 = "inf".parse().unwrap();
    // println!("{},{}", num, num.is_infinite());
    let mut test = File::open("./test/torznab_animetosho.xml").unwrap();
    let mut s = String::new();
    test.read_to_string(&mut s).unwrap();
    let rss: RssTag = quick_xml::de::from_str(s.as_str()).unwrap();
    println!("{:#?}", rss);
}
