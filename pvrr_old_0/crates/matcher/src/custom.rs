use crate::MatchedItem;
use base_tool::text::Text;
use database::entity::MatchCorrector;

pub(super) struct CustomMatch;

impl CustomMatch {
    pub(super) async fn try_custom_match(&self, title: Text) -> Option<MatchedItem> {
        let all = match MatchCorrector::all().await {
            Ok(it) => it,
            Err(_) => return None,
        };

        for matcher in all {
            if let Some(_group) = title.captures(matcher.regex()) {
                // todo group.ep, imdb, season to MatchedItem
            }
        }

        return None;
    }
}
