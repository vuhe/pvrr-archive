mod custom;
mod elements;
mod matched_item;
mod matched_tag;
mod parser;
mod token;

use base_tool::text::Text;
use custom::CustomMatch;
pub use matched_item::MatchedItem;
pub use matched_tag::*;
use parser::FilmNameParser;

pub struct Matcher;

impl Matcher {
    pub async fn match_title(&self, title: Text) -> MatchedItem {
        let custom = CustomMatch.try_custom_match(title.clone()).await;
        if let Some(it) = custom {
            return it;
        }
        static_match_title(title)
    }
}

pub fn static_match_title(title: Text) -> MatchedItem {
    let elements = FilmNameParser::parse(title);
    MatchedItem::from(elements)
}
