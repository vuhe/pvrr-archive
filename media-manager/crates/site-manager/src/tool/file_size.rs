use std::str::FromStr;
use anyhow::anyhow;
use regex::Regex;

pub(crate) struct FileSize {
    byte_num: u32,
}

impl FromStr for FileSize {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.replace(",", "");
        let regex = Regex::new(r"([.\d]*)\s*(KB|KiB|MB|MiB|GB|GiB|TB|TiB|PB|PiB)")?
            .captures(&value).ok_or(anyhow!("无法匹配: {}", s))?;

        let num = regex.get(1).ok_or(anyhow!("无法解析: {}", s))?.as_str();
        let unit = regex.get(2).ok_or(anyhow!("无法解析: {}", s))?.as_str();
        let num: f64 = num.parse().map_err(|_| anyhow!("无法解析: {}", s))?;

        // 保留小数点后两位
        let num = (num * 100.0).round() as u32;
        let byte_num = match unit {
            "KB" | "KiB" => num * 1024,
            "MB" | "MiB" => num * 1024 * 1024,
            "GB" | "GiB" => num * 1024 * 1024 * 1024,
            "TB" | "TiB" => num * 1024 * 1024 * 1024 * 1024,
            "PB" | "PiB" => num * 1024 * 1024 * 1024 * 1024 * 1024,
            _ => num
        };
        let byte_num = byte_num / 100;

        Ok(FileSize { byte_num })
    }
}

#[test]
fn file_size_convert_test() {
    let num_str = "1.2 KB";
    let file_size: FileSize = num_str.parse().unwrap();
    assert_eq!(file_size.byte_num, 1228);
}
