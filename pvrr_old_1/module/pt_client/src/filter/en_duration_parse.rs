use super::FilterTrait;
use once_cell::sync::Lazy;
use regex::Regex;

static EN_DURATION: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<num>[\d.]+)\s(?P<unit>seconds|minutes|hours|days|weeks|years)\sago").unwrap()
});

pub(super) struct EnDurationParse;

impl FilterTrait for EnDurationParse {
    fn invoke(&self, input: Option<String>) -> Option<String> {
        let group = match EN_DURATION.captures(input.as_str()) {
            None => return None,
            Some(it) => it,
        };
        let num = group.name("num").unwrap().as_str().parse().unwrap_or(0.0);
        let num = match group.name("unit").unwrap().as_str() {
            "seconds" => num,
            "minutes" => num * 60.0,
            "hours" => num * 60.0 * 60.0,
            "days" => num * 60.0 * 60.0 * 24.0,
            "weeks" => num * 60.0 * 60.0 * 24.0 * 7.0,
            "years" => num * 60.0 * 60.0 * 24.0 * 365.0,
            _ => 0.0,
        };
        Some(num.to_string())
    }
}
