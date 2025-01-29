use std::sync::Arc;
use anyhow::anyhow;
use cached::{cached_control, TimedCache};
use scraper::Selector;

cached_control! {
    CACHE: TimedCache<String, Arc<Selector>> = TimedCache::with_lifespan(120);
    Key = input.to_owned();
    PostGet(cached_val) = return Ok(cached_val.clone());
    PostExec(body_result) = {
        match body_result {
            Ok(v) => v,
            Err(e) => return Err(e),
        }
    };
    Set(set_value) = set_value.clone();
    Return(return_value) = Ok(return_value);
    fn selector_cache(input: &str) -> anyhow::Result<Arc<Selector>> = {
        let selector = Selector::parse(input)
            .map_err(|_| anyhow!("非法 selector"))?;
        Ok(Arc::new(selector))
    }
}

/// 创建 [Selector] 并缓存
pub(crate) fn build_selector(val: &str) -> anyhow::Result<Arc<Selector>> {
    selector_cache(val)
}
