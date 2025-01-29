use html5ever::tendril::StrTendril;
use std::borrow::Cow;
use std::ops::Deref;

#[derive(Debug)]
pub(crate) struct StrValue(pub(super) StrTendril);

impl From<Cow<'_, str>> for TagValue {
    fn from(value: Cow<'_, str>) -> Self {
        Self(StrTendril::from(&*value))
    }
}

impl From<StrTendril> for TagValue {
    fn from(value: StrTendril) -> Self {
        Self(value)
    }
}

impl Deref for StrValue {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
