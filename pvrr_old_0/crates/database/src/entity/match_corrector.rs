use crate::table::match_corrector::*;
use crate::DATABASE;
use base_tool::error::{AnyContext, AnyResult};
use base_tool::text::Text;
use sea_orm::{ActiveModelTrait, EntityTrait, NotSet, Set};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchCorrector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    id: Option<u32>,
    regex: Text,
    imdb: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    season: Option<u16>,
}

impl MatchCorrector {
    pub fn regex(&self) -> Text {
        self.regex.clone()
    }

    pub fn imdb(&self) -> Text {
        self.imdb.clone()
    }

    pub fn season(&self) -> Option<u16> {
        self.season.clone()
    }
}

impl MatchCorrector {
    pub async fn all() -> AnyResult<Vec<MatchCorrector>> {
        let vec = Entity::find()
            .all(DATABASE.get().unwrap())
            .await
            .context("获取 match_corrector 错误")?;
        Ok(vec.into_iter().map(|it| Self::from_model(it)).collect())
    }

    pub async fn insert(self) -> AnyResult {
        self.into_model()
            .insert(DATABASE.get().unwrap())
            .await
            .context("添加 match_corrector 错误")
            .map(|_| ())
    }
}

impl MatchCorrector {
    fn from_model(value: Model) -> Self {
        Self {
            id: Some(value.id),
            regex: Text::from(value.regex),
            imdb: Text::from(value.imdb),
            season: value.season,
        }
    }

    fn into_model(self) -> ActiveModel {
        ActiveModel {
            id: self.id.map(|it| Set(it)).unwrap_or(NotSet),
            regex: Set(self.regex.to_string()),
            imdb: Set(self.imdb.to_string()),
            season: Set(self.season),
        }
    }
}
