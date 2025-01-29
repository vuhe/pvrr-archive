use regex::Regex;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{AddAssign, Deref};
use std::rc::Rc;

#[derive(Clone, Eq, PartialEq)]
enum TextRef<'t> {
    Slice(&'t str),
    String(Rc<String>),
}

#[derive(Clone)]
pub(crate) struct TokenText<'t> {
    inner: TextRef<'t>,
    start: usize,
    end: usize,
}

impl<'t> TokenText<'t> {
    pub(crate) fn split_once(&self, regex: &Regex) -> Option<(Self, Self, Self)> {
        let (start, end) = match regex.find(self) {
            None => return None,
            Some(it) => (it.start(), it.end()),
        };
        let start = self.start + start;
        let end = self.start + end;
        let left = Self { inner: self.inner.clone(), start: self.start, end: start };
        let sp = Self { inner: self.inner.clone(), start, end };
        let right = Self { inner: self.inner.clone(), start: end, end: self.end };
        Some((left, sp, right))
    }

    pub(crate) fn capture_at(&self, regex: &Regex, i: usize) -> Option<Self> {
        regex.captures(self).and_then(|it| it.get(i)).map(|it| Self {
            inner: self.inner.clone(),
            start: self.start + it.start(),
            end: self.start + it.end(),
        })
    }
}

impl<'t> From<&'t str> for TokenText<'t> {
    fn from(value: &'t str) -> Self {
        let size = value.len();
        let inner = TextRef::Slice(value);
        Self { inner, start: 0, end: size }
    }
}

impl Deref for TokenText<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        let str = match self.inner {
            TextRef::Slice(it) => it,
            TextRef::String(ref it) => it.as_str(),
        };
        &str[self.start..self.end]
    }
}

impl PartialEq<&str> for TokenText<'_> {
    fn eq(&self, other: &&str) -> bool {
        &**self == *other
    }
}

impl AddAssign<TokenText<'_>> for TokenText<'_> {
    fn add_assign(&mut self, rhs: TokenText<'_>) {
        if self.inner == rhs.inner && self.end == rhs.start {
            self.end = rhs.end;
        } else {
            let inner = Rc::new((&**self).to_owned() + rhs.as_ref());
            self.start = 0;
            self.end = inner.len();
            self.inner = TextRef::String(inner);
        }
    }
}

impl Default for TokenText<'_> {
    fn default() -> Self {
        TokenText::from("")
    }
}

impl Debug for TokenText<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &**self)
    }
}

impl Display for TokenText<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &**self)
    }
}
