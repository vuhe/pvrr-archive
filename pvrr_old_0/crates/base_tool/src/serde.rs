pub mod json {
    use crate::error::{AnyContext, AnyResult};
    use serde::{de::DeserializeOwned, Deserialize, Serialize};
    pub use serde_json::{json as build_json, Value as JsonVal};
    use std::fmt::Debug;

    pub fn from_str<'a, T: Deserialize<'a>>(s: &'a str) -> AnyResult<T> {
        serde_json::from_str(s)
            .with_context(|| format!("json value: {}", s))
            .context("Json 反序列化错误")
    }

    pub fn from_value<T: DeserializeOwned>(s: JsonVal) -> AnyResult<T> {
        serde_json::from_value(s).context("Json 反序列化错误")
    }

    pub fn to_string<T: ?Sized + Serialize + Debug>(value: &T) -> AnyResult<String> {
        serde_json::to_string(value)
            .with_context(|| format!("object: {:?}", value))
            .context("Json 序列化错误")
    }
}

pub mod xml {
    use crate::error::{AnyContext, AnyResult};
    use serde::Deserialize;

    pub fn from_str<'a, T: Deserialize<'a>>(s: &'a str) -> AnyResult<T> {
        quick_xml::de::from_str(s)
            .with_context(|| format!("xml value: {}", s))
            .context("XML 反序列化错误")
    }
}

pub mod yaml {
    use crate::error::{AnyContext, AnyResult};
    use serde::de::DeserializeOwned;
    pub use serde_yaml::Value as YamlVal;
    use std::fs::File;
    use std::path::Path;

    pub fn from_path<T: DeserializeOwned>(path: &Path) -> AnyResult<T> {
        let file = File::open(path)
            .with_context(|| format!("file path: {:?}", path))
            .context("读取文件错误")?;
        serde_yaml::from_reader(file)
            .with_context(|| format!("yaml file path: {:?}", path))
            .context("yaml 反序列化错误")
    }
}

pub mod bencode {
    use crate::error::{AnyContext, AnyResult};
    pub use bt_bencode::Value as BencodeVal;
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use std::fmt::Debug;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    pub fn from_path<T: DeserializeOwned>(path: &Path) -> AnyResult<T> {
        let file = File::open(path)
            .with_context(|| format!("file path: {:?}", path))
            .context("读取文件错误")?;
        let reader = BufReader::new(file);
        bt_bencode::from_reader(reader)
            .with_context(|| format!("torrent file path: {:?}", path))
            .context("bencode 反序列化错误")
    }

    pub fn to_bytes<T: ?Sized + Serialize + Debug>(value: &T) -> AnyResult<Vec<u8>> {
        bt_bencode::to_vec(value)
            .with_context(|| format!("object: {:?}", value))
            .context("bencode 序列化错误")
    }
}
