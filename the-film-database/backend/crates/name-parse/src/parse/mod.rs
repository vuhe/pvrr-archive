mod file_source;
mod film_title;
mod film_year;
mod release_group;
mod streaming_service;
mod tv_episode;
mod video_codec;

use crate::token::ItemRef;

impl ItemRef<'_, '_> {
    /// 单括号 token, e.g. (2000)
    fn is_token_isolated(&self) -> bool {
        // 前一个非分隔符
        let prev = self.prev_find(|it| !it.is_delimiter());
        // 后一个非分隔符
        let next = self.next_find(|it| !it.is_delimiter());
        self.enclosed()
            && prev.map(|it| it.is_open_bracket()).unwrap_or(false)
            && next.map(|it| it.is_closed_bracket()).unwrap_or(false)
    }
}
