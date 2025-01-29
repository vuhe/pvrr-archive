use std::collections::HashMap;
use crate::site_category::SiteCategory;

pub(crate) struct CategoryMap {
    map: HashMap<String, SiteCategory>,
}

impl CategoryMap {
    pub(super) fn new() -> Self { CategoryMap { map: HashMap::new() } }

    pub(crate) fn get(&self, id: &str) -> Option<SiteCategory> {
        self.map.get(id).map(|it| it.clone())
    }

    pub(crate) fn search_ids(&self, type_name: &str) -> Vec<String> {
        self.map.iter()
            .filter(|&it| it.1.type_name() == type_name)
            .map(|it| it.0.clone())
            .collect()
    }

    pub(super) fn set(&mut self, id: &str, value: SiteCategory) {
        self.map.insert(id.to_owned(), value);
    }
}
