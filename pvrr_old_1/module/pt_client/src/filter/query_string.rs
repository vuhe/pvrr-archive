use super::FilterTrait;
use crate::values::ValArgs;
use std::borrow::Cow;

type QueryPair<'a> = Vec<(Cow<'a, str>, Cow<'a, str>)>;

pub(super) struct QueryString {
    key: String,
}

impl QueryString {
    pub(super) fn new(args: ValArgs) -> Result<Self, &'static str> {
        let key = match args.get(0).map(|it| it.to_string()) {
            Some(it) => it,
            None => return Err("args 应为 string"),
        };
        Ok(Self { key })
    }
}

impl FilterTrait for QueryString {
    fn invoke(&self, input: Option<String>) -> Option<String> {
        let input = match input {
            None => return None,
            Some(it) => it,
        };
        input
            .split_once("?")
            .map(|it| it.1)
            .and_then(|it| serde_urlencoded::from_str::<'_, QueryPair<'_>>(it).ok())
            .unwrap_or_default()
            .into_iter()
            .find(|it| it.0.as_ref() == self.key.as_str())
            .map(|it| it.1.to_string())
    }
}
