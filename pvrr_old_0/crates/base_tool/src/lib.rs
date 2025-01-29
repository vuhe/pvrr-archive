pub mod datetime;
pub mod encode;
pub mod regex;
pub mod serde;
pub mod text;

pub mod env {
    use crate::error::AnyContext;
    use crate::once_cell::Lazy;
    use std::{env, path::PathBuf};

    pub static DATA_PATH: Lazy<PathBuf> = Lazy::new(|| {
        let value = env::var("DATA_PATH").context("缺失环境变量 DATA_PATH");
        value.map(|it| PathBuf::from(it)).unwrap()
    });

    pub static SQLX_LOG_ENABLE: Lazy<bool> = Lazy::new(|| {
        let value = env::var("SQLX_LOG_ENABLE");
        value.map(|it| it.parse().unwrap_or(false)).unwrap_or(false)
    });
}

pub mod error {
    pub use anyhow::Context as AnyContext;
    pub type AnyResult<T = ()> = Result<T, AnyError>;
    pub use anyhow::Error as AnyError;
}

pub mod once_cell {
    pub use once_cell::sync::Lazy;
    pub use once_cell::sync::OnceCell;
}
