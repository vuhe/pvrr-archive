use crate::token::ItemRef;
use std::ops::RangeInclusive;

pub(crate) struct Episode {
    season: Option<u16>,
    season_range: Option<RangeInclusive<u16>>,
    episode: Option<u16>,
    episode_range: Option<RangeInclusive<u16>>,
}

impl Episode {
    /// 存在 episode 但不存在 season 时为默认 season 存在
    fn use_default_season(&self) -> bool {
        self.season.is_none()
            && self.season_range.is_none()
            && (self.episode.is_some() || self.episode_range.is_some())
    }

    fn season_include(&self, season: u16) -> bool {
        // 默认 season == 1
        if self.use_default_season() && season == 1 {
            return true;
        }
        self.season_range
            .as_ref()
            .map(|it| it.contains(&season))
            .or_else(|| self.season.map(|it| it == season))
            .unwrap_or(false)
    }

    fn set_season(&mut self, season: u16) {
        match self.season {
            None => self.season = Some(season),
            Some(old) if season < old => self.season = Some(season),
            _ => {}
        }
    }

    fn set_season_range(&mut self, range: RangeInclusive<u16>) {
        if self.season_range.is_none() {
            self.season_range = Some(range);
        }
    }

    fn episode_include(&self, episode: u16) -> bool {
        self.episode_range
            .as_ref()
            .map(|it| it.contains(&episode))
            .or_else(|| self.episode.map(|it| it == episode))
            .unwrap_or(false)
    }

    fn set_episode(&mut self, episode: u16) {
        match self.episode {
            None => self.episode = Some(episode),
            Some(old) if episode < old => self.episode = Some(episode),
            _ => {}
        }
    }

    fn set_episode_range(&mut self, range: RangeInclusive<u16>) {
        if self.episode_range.is_none() {
            self.episode_range = Some(range);
        }
    }
}

impl ItemRef<'_, '_> {}
