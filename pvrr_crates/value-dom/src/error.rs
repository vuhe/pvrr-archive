use cssparser::{ParseError, ParseErrorKind};
use selectors::parser::SelectorParseErrorKind;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("html 部分解析错误")]
    HtmlParseError(Vec<String>),
    #[error("xml 解析错误, {0}")]
    XmlParseError(#[from] quick_xml::Error),
    #[error("json 解析错误, {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("CSS 选择器解析错误, {0}")]
    CssSelectorParseError(String),
}

impl From<ParseError<'_, SelectorParseErrorKind<'_>>> for Error {
    fn from(value: ParseError<'_, SelectorParseErrorKind<'_>>) -> Self {
        let hint = match value.kind {
            ParseErrorKind::Basic(it) => format!("{it:?}, location: {:?}", value.location),
            ParseErrorKind::Custom(it) => format!("{it:?}, location: {:?}", value.location),
        };
        Self::CssSelectorParseError(hint)
    }
}
