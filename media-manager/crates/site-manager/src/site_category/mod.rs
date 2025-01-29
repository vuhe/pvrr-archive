mod category_map;
mod category_info;
mod category_parser;

pub use category_info::SiteCategory;
pub(crate) use category_map::CategoryMap;
// pub(crate) use category_parser::CategoryParser;

pub const MOVIE: &str = "MOVIE";
pub const TV: &str = "TV";
pub const DOCUMENTARY: &str = "DOCUMENTARY";
pub const ANIME: &str = "ANIME";
pub const MUSIC: &str = "MUSIC";
pub const GAME: &str = "GAME";
pub const AV: &str = "AV";
pub const OTHER: &str = "OTHER";
