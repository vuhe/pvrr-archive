use super::FilterTrait;
use once_cell::sync::Lazy;
use regex::Regex;

static CN_DURATION: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"((?P<day>\d+)日)?((?P<hour>\d+)[時时])?((?P<minutes>\d+)分)?").unwrap()
});

pub(super) struct CnDurationParse;

impl FilterTrait for CnDurationParse {
    fn invoke(&self, input: Option<String>) -> Option<String> {
        let group = match CN_DURATION.captures(input.as_str()) {
            None => return None,
            Some(it) => it,
        };
        let mut num: u64 = 0;
        if let Some(day) = group.name("day") {
            num += day.as_str().parse().unwrap_or(0_u64) * 60 * 60 * 24;
        }
        if let Some(hour) = group.name("hour") {
            num += hour.as_str().parse().unwrap_or(0_u64) * 60 * 60;
        }
        if let Some(minutes) = group.name("minutes") {
            num += minutes.as_str().parse().unwrap_or(0_u64) * 60;
        }
        Some(num.to_string())
    }
}
