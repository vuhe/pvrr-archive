use std::sync::Arc;

pub struct SiteCategory {
    inner: Arc<SiteCategoryRef>,
}

impl SiteCategory {
    pub(super) fn new<'a>(type_name: &'a str, tag: &'a str, desc: &'a str) -> Self {
        SiteCategory {
            inner: Arc::new(SiteCategoryRef {
                type_name: type_name.to_owned(),
                tag: tag.to_owned(),
                desc: desc.to_owned(),
            })
        }
    }

    pub fn type_name(&self) -> &str { self.inner.type_name.as_str() }
    pub fn tag(&self) -> &str { self.inner.tag.as_str() }
    pub fn desc(&self) -> &str { self.inner.desc.as_str() }
}

impl Clone for SiteCategory {
    fn clone(&self) -> Self { SiteCategory { inner: self.inner.clone() } }
}

struct SiteCategoryRef {
    type_name: String,
    tag: String,
    desc: String,
}
