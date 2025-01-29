use html5ever::{ExpandedName, LocalName, Namespace, QualName};
use quick_xml::name::QName;

#[derive(Eq, PartialEq, Hash, Debug)]
pub(crate) struct TagName(pub(super) QualName);

impl TagName {
    pub(crate) fn ns(&self) -> &Namespace {
        &self.0.ns
    }

    pub(crate) fn local(&self) -> &LocalName {
        &self.0.local
    }
}

impl From<usize> for TagName {
    fn from(value: usize) -> Self {
        Self::from(value.to_string())
    }
}

impl From<String> for TagName {
    fn from(value: String) -> Self {
        Self(QualName::new(None, Default::default(), value.into()))
    }
}

impl From<&[u8]> for TagName {
    fn from(value: &[u8]) -> Self {
        let local = String::from_utf8_lossy(value);
        Self(QualName::new(None, Default::default(), local.into()))
    }
}

impl From<QName<'_>> for TagName {
    fn from(value: QName<'_>) -> Self {
        let prefix = value.prefix();
        let prefix = prefix.map(|it| String::from_utf8_lossy(it.as_ref()).into());
        let local = value.local_name();
        let local = String::from_utf8_lossy(local.as_ref()).into();
        Self(QualName::new(prefix, Default::default(), local))
    }
}

impl From<QualName> for TagName {
    fn from(value: QualName) -> Self {
        Self(value)
    }
}

impl<T: AsRef<str>> PartialEq<T> for TagName {
    fn eq(&self, other: &T) -> bool {
        match other.as_ref().split_once(":") {
            None => self.0.local.as_ref() == other.as_ref(),
            Some((prefix, local)) => {
                self.0.prefix.as_ref().map(|it| it.as_ref()) == Some(prefix)
                    && self.0.local.as_ref() == local
            }
        }
    }
}
