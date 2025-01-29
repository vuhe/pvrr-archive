use crate::text::Text;
use cached::proc_macro::cached;
use regex::{Captures, Regex as RegexRef};
use std::borrow::Cow;

#[cached]
fn static_regex(str: &'static str) -> RegexRef {
    RegexRef::new(str).unwrap()
}

pub struct Regex(Option<RegexRef>);

pub(crate) struct RegexGroup<'a>(Captures<'a>);

pub trait IntoRegex {
    fn into_regex(self) -> Regex;
}

impl Regex {
    pub(crate) fn is_match(&self, str: &str) -> bool {
        match self.0 {
            None => false,
            Some(ref regex) => regex.is_match(str),
        }
    }

    pub(crate) fn split(&self, str: &str) -> Vec<(usize, usize)> {
        match self.0 {
            None => vec![(0, str.len())],
            Some(ref regex) => regex.find_iter(str).map(|it| (it.start(), it.end())).collect(),
        }
    }

    pub(crate) fn split_once(&self, str: &str) -> Option<(usize, usize)> {
        self.0.as_ref().and_then(|it| it.find(str)).map(|it| (it.start(), it.end()))
    }

    pub(crate) fn captures<'t>(&self, str: &'t str) -> Option<RegexGroup<'t>> {
        self.0.as_ref().and_then(|it| it.captures(str)).map(|it| RegexGroup(it))
    }
}

impl<'a> RegexGroup<'a> {
    pub(crate) fn get(&self, i: usize) -> Option<(usize, usize)> {
        self.0.get(i).map(|it| (it.start(), it.end()))
    }

    pub(crate) fn name(&self, name: &str) -> Option<(usize, usize)> {
        self.0.name(name).map(|it| (it.start(), it.end()))
    }
}

/// &'static str 的 regex 为程序内部 regex 通常它不应该出现错误，
/// 并且为了提高性能和重用，需要缓存
impl IntoRegex for &'static str {
    fn into_regex(self) -> Regex {
        Regex(Some(static_regex(self)))
    }
}

/// 一般的 &str 生成的 regex 可以使用 [Cow::Borrowed] 包裹创建
impl IntoRegex for Cow<'_, str> {
    fn into_regex(self) -> Regex {
        Regex(RegexRef::new(self.as_ref()).ok())
    }
}

/// 默认为用户输入，需要检查
impl IntoRegex for String {
    fn into_regex(self) -> Regex {
        Regex(RegexRef::new(self.as_str()).ok())
    }
}

/// 默认为用户输入，需要检查
impl IntoRegex for Text {
    fn into_regex(self) -> Regex {
        Regex(RegexRef::new(&*self).ok())
    }
}
