pub mod download_client;
pub mod index_client;
pub mod system_config;

use sea_orm::ActiveModelTrait;
use serde::{de::Error, Deserialize, Deserializer};

// 自动实现以支持 ActiveModel 直接反序列化
// ActiveModel::from_json 方法中会检测 json 是否含有某个键
// 例如：当 json 中不含有 id 字段时，反序列化到 ActiveModel 会设置为 NotSet
macro_rules! deserialize_active_model {
    ($m:ident) => {
        impl<'de> Deserialize<'de> for $m::ActiveModel {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
                Self::from_json(value).map_err(D::Error::custom)
            }
        }
    };
}

deserialize_active_model!(download_client);
deserialize_active_model!(index_client);
