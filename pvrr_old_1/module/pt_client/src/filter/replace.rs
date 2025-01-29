use super::FilterTrait;
use crate::values::ValArgs;
use regex::Regex;

pub(super) struct RegexReplace {
    regex: Regex,
    pat: String,
}

impl RegexReplace {
    pub(super) fn new(args: ValArgs) -> Result<Self, &'static str> {
        let regex = match args.get(0).and_then(|it| it.as_str()) {
            Some(it) => it,
            None => return Err("args[0] 应为 string"),
        };
        let regex = match Regex::new(regex) {
            Ok(it) => it,
            Err(_) => return Err("正则表达式解析错误"),
        };
        let pat = match args.get(1).and_then(|it| it.as_str()) {
            Some(it) => it,
            None => return Err("args[1] 应为 string"),
        };
        Ok(Self { regex, pat: pat.to_owned() })
    }
}

impl FilterTrait for RegexReplace {
    fn invoke(&self, input: Option<String>) -> Option<String> {
        Some(regex.replace(input.as_str(), pat.as_str()).to_string())
    }
}
