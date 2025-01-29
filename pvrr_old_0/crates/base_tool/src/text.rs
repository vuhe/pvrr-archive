use crate::regex::{IntoRegex, RegexGroup};
use serde::de::{Error as DeErr, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};
use std::ops::{Add, Deref};
use std::rc::Rc;

enum TextRef {
    Slice(&'static str),
    String(Rc<String>),
}

pub struct Text {
    text: TextRef,
    start: usize,
    end: usize,
}

pub struct TextGroup<'a> {
    group: RegexGroup<'a>,
    text: Text,
}

impl Text {
    fn sub_str(&self, start: usize, end: usize) -> Self {
        if end < start {
            return Self::default();
        }
        let text = match self.text {
            TextRef::Slice(it) => TextRef::Slice(it),
            TextRef::String(ref it) => TextRef::String(it.clone()),
        };
        Self { text, start: self.start + start, end: self.start + end }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn is_ascii_digit(&self) -> bool {
        self.is_not_empty() && self.chars().all(|it| it.is_ascii_digit())
    }

    pub fn is_match<P: IntoRegex>(&self, pat: P) -> bool {
        pat.into_regex().is_match(self)
    }

    pub fn has_chinese_char(&self) -> bool {
        self.chars().any(|it| ('\u{4E00}'..='\u{9FBB}').contains(&it))
    }

    /// 阿拉伯数字和中文数字
    pub fn has_number(&self) -> bool {
        self.is_match(r"[\d一二三四五六七八九十百千零]+")
    }

    pub fn captures<P: IntoRegex>(&self, pat: P) -> Option<TextGroup<'_>> {
        pat.into_regex().captures(self).map(|group| TextGroup { group, text: self.clone() })
    }

    /// regex 全分割，方法会忽略分隔符
    pub fn split<P: IntoRegex>(&self, pat: P) -> Vec<Self> {
        let mut result = Vec::new();
        let mut last: usize = 0;
        pat.into_regex().split(self).into_iter().for_each(|(start, end)| {
            result.push(self.sub_str(last, start));
            last = end;
        });
        if last != self.len() {
            result.push(self.sub_str(last, self.len()));
        }
        result
    }

    pub fn split_at(&self, mid: usize) -> (Self, Self) {
        (self.sub_str(0, mid), self.sub_str(mid, self.len()))
    }

    pub fn split_once<P: IntoRegex>(&self, pat: P) -> Option<(Self, Self, Self)> {
        pat.into_regex().split_once(self).map(|(start, end)| {
            (self.sub_str(0, start), self.sub_str(start, end), self.sub_str(end, self.len()))
        })
    }

    pub fn trim_start_matches(&self, pat: &[char]) -> Self {
        let start = self.len() - (&**self).trim_start_matches(pat).len();
        self.sub_str(start, self.len())
    }

    pub fn trim_matches(&self, pat: &[char]) -> Self {
        let start = self.len() - (&**self).trim_start_matches(pat).len();
        let end = (&**self).trim_end_matches(pat).len();
        self.sub_str(start, end)
    }

    pub fn to_u16(&self) -> Option<u16> {
        self.parse().ok()
    }
}

impl<'a> TextGroup<'a> {
    pub fn get(&self, i: usize) -> Option<Text> {
        self.group.get(i).map(|(start, end)| self.text.sub_str(start, end))
    }

    pub fn name(&self, name: &str) -> Option<Text> {
        self.group.name(name).map(|(start, end)| self.text.sub_str(start, end))
    }
}

impl From<&'static str> for Text {
    fn from(value: &'static str) -> Self {
        Self { text: TextRef::Slice(value), start: 0, end: value.len() }
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        let len = value.len();
        Self { text: TextRef::String(Rc::new(value)), start: 0, end: len }
    }
}

impl Default for Text {
    fn default() -> Self {
        Text::from("")
    }
}

impl Deref for Text {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self.text {
            TextRef::Slice(it) => &it[self.start..self.end],
            TextRef::String(ref it) => &it.as_str()[self.start..self.end],
        }
    }
}

impl Add for Text {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from((&*self).to_owned() + &*rhs)
    }
}

impl PartialEq<str> for Text {
    fn eq(&self, other: &str) -> bool {
        &**self == other
    }
}

impl PartialEq<&str> for Text {
    fn eq(&self, other: &&str) -> bool {
        &**self == *other
    }
}

impl PartialEq for Text {
    fn eq(&self, other: &Self) -> bool {
        &**self == &**other
    }
}

impl Eq for Text {}

impl PartialOrd for Text {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (&**self).partial_cmp(&**other)
    }
}

impl Ord for Text {
    fn cmp(&self, other: &Self) -> Ordering {
        (&**self).cmp(&**other)
    }
}

impl Hash for Text {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (&**self).hash(state)
    }
}

impl Clone for Text {
    fn clone(&self) -> Self {
        let text = match self.text {
            TextRef::Slice(it) => TextRef::Slice(it),
            TextRef::String(ref it) => TextRef::String(it.clone()),
        };
        Self { text, start: self.start, end: self.end }
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &**self)
    }
}

impl Debug for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &**self)
    }
}

// ============================= serde =============================

impl Serialize for Text {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&**self)
    }
}

impl<'de> Deserialize<'de> for Text {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_string(TextVisitor)
    }
}

struct TextVisitor;

impl<'a> Visitor<'a> for TextVisitor {
    type Value = Text;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a Text")
    }

    fn visit_str<E: DeErr>(self, v: &str) -> Result<Self::Value, E> {
        Ok(Text::from(v.to_owned()))
    }

    fn visit_string<E: DeErr>(self, v: String) -> Result<Self::Value, E> {
        Ok(Text::from(v))
    }

    fn visit_bytes<E: DeErr>(self, v: &[u8]) -> Result<Self::Value, E> {
        match std::str::from_utf8(v) {
            Ok(s) => Ok(Text::from(s.to_owned())),
            Err(_) => Err(DeErr::invalid_value(Unexpected::Bytes(v), &self)),
        }
    }

    fn visit_byte_buf<E: DeErr>(self, v: Vec<u8>) -> Result<Self::Value, E> {
        match String::from_utf8(v) {
            Ok(s) => Ok(Text::from(s)),
            Err(e) => Err(DeErr::invalid_value(Unexpected::Bytes(&e.into_bytes()), &self)),
        }
    }
}
