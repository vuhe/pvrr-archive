use anyhow::{anyhow, bail};
use chrono::{Duration, Local, NaiveDateTime, TimeZone};
use regex::Regex;
use serde_yaml::Sequence;

pub(super) trait FilterFn {
    fn invoke(&self, value: String) -> Option<String>;
}

pub(super) type StringFilter = Box<dyn FilterFn>;

pub(super) fn build_filter(name: &str, args: &Sequence) -> anyhow::Result<StringFilter> {
    match name {
        "strip_prefix" => Ok(strip_prefix(args)?),
        "strip_suffix" => Ok(strip_suffix(args)?),
        "replace" => Ok(replace(args)?),
        "append" => Ok(append(args)),
        "prepend" => Ok(prepend(args)),
        "to_lower" => Ok(to_lower()),
        "to_upper" => Ok(to_upper()),
        "split" => Ok(split(args)?),
        "re_search" => Ok(re_search(args)?),
        "date_parse" => Ok(date_parse(args)?),
        "date_parse_elapsed_cn" => Ok(date_parse_elapsed_cn()),
        "date_parse_elapsed_en" => Ok(date_parse_elapsed_en()),
        _ => bail!("不支持 {} filter", name)
    }
}

fn strip_prefix(args: &Sequence) -> anyhow::Result<StringFilter> {
    let prefix = args.get(0)
        .and_then(|it| it.as_str())
        .map(|it| it.to_owned())
        .ok_or(anyhow!("strip_prefix 参数[0]错误"))?;

    struct StripPrefixFn {
        prefix: String,
    }

    impl FilterFn for StripPrefixFn {
        fn invoke(&self, value: String) -> Option<String> {
            value.strip_prefix(&self.prefix).map(|it| it.to_owned())
        }
    }

    Ok(Box::new(StripPrefixFn { prefix }))
}

fn strip_suffix(args: &Sequence) -> anyhow::Result<StringFilter> {
    let suffix = args.get(0)
        .and_then(|it| it.as_str())
        .map(|it| it.to_owned())
        .ok_or(anyhow!("strip_suffix 参数[0]错误"))?;

    struct StripSuffixFn {
        suffix: String,
    }

    impl FilterFn for StripSuffixFn {
        fn invoke(&self, value: String) -> Option<String> {
            value.strip_suffix(&self.suffix).map(|it| it.to_owned())
        }
    }
    Ok(Box::new(StripSuffixFn { suffix }))
}

fn replace(args: &Sequence) -> anyhow::Result<StringFilter> {
    let old = args.get(0)
        .and_then(|it| it.as_str())
        .map(|it| it.to_owned())
        .ok_or(anyhow!("replace 参数[0]错误"))?;

    let new = args.get(1)
        .and_then(|it| it.as_str())
        .map(|it| it.to_owned())
        .ok_or(anyhow!("replace 参数[1]错误"))?;

    struct ReplaceFn {
        old: String,
        new: String,
    }

    impl FilterFn for ReplaceFn {
        fn invoke(&self, value: String) -> Option<String> {
            Some(value.replace(self.old.as_str(), self.new.as_str()))
        }
    }

    Ok(Box::new(ReplaceFn { old, new }))
}

fn append(args: &Sequence) -> StringFilter {
    let append_str = args.iter().map(|it| it.as_str().unwrap_or(""))
        .fold(String::default(), |acc, it| acc + it);

    struct AppendFn {
        append_str: String,
    }

    impl FilterFn for AppendFn {
        fn invoke(&self, value: String) -> Option<String> {
            Some(value + &self.append_str)
        }
    }

    Box::new(AppendFn { append_str })
}

fn prepend(args: &Sequence) -> StringFilter {
    let prepend_str = args.iter().map(|it| it.as_str().unwrap_or(""))
        .fold(String::default(), |acc, it| acc + it);

    struct PrependFn {
        prepend_str: String,
    }

    impl FilterFn for PrependFn {
        fn invoke(&self, value: String) -> Option<String> {
            Some(self.prepend_str.clone() + &value)
        }
    }

    Box::new(PrependFn { prepend_str })
}

fn to_lower() -> StringFilter {
    struct ToLowerFn;
    impl FilterFn for ToLowerFn {
        fn invoke(&self, value: String) -> Option<String> {
            Some(value.to_lowercase())
        }
    }
    Box::new(ToLowerFn)
}

fn to_upper() -> StringFilter {
    struct ToUpperFn;
    impl FilterFn for ToUpperFn {
        fn invoke(&self, value: String) -> Option<String> {
            Some(value.to_uppercase())
        }
    }
    Box::new(ToUpperFn)
}

fn split(args: &Sequence) -> anyhow::Result<StringFilter> {
    let pat = args.get(0)
        .and_then(|it| it.as_str())
        .map(|it| it.to_owned())
        .ok_or(anyhow!("split 参数[0]错误"))?;
    let index = args.get(1)
        .and_then(|it| it.as_u64())
        .map(|it| it as usize)
        .ok_or(anyhow!("split 参数[1]错误"))?;

    struct SplitFn {
        pat: String,
        index: usize,
    }

    impl FilterFn for SplitFn {
        fn invoke(&self, value: String) -> Option<String> {
            value.split(self.pat.as_str())
                .skip(self.index).next()
                .map(|it| it.to_owned())
        }
    }

    Ok(Box::new(SplitFn { pat, index }))
}

fn re_search(args: &Sequence) -> anyhow::Result<StringFilter> {
    let regex = args.get(0)
        .and_then(|it| it.as_str())
        .ok_or(anyhow!("re_search 参数[0]错误"))?;
    let regex = Regex::new(regex).map_err(|_| anyhow!("错误的正则表达式"))?;

    struct ReSearch {
        regex: Regex,
    }

    impl FilterFn for ReSearch {
        fn invoke(&self, value: String) -> Option<String> {
            self.regex.captures(value.as_str())
                .and_then(|it| it.get(0))
                .map(|it| it.as_str().to_owned())
        }
    }

    Ok(Box::new(ReSearch { regex }))
}

fn date_parse(args: &Sequence) -> anyhow::Result<StringFilter> {
    let format_str = args.get(0)
        .and_then(|it| it.as_str())
        .map(|it| it.to_owned())
        .ok_or(anyhow!("date_parse 参数[0]错误"))?;

    struct DateParseFn {
        format_str: String,
    }

    impl FilterFn for DateParseFn {
        fn invoke(&self, value: String) -> Option<String> {
            let local = Local::now().timezone();
            NaiveDateTime::parse_from_str(value.as_str(), self.format_str.as_str()).ok()
                .and_then(|it| local.from_local_datetime(&it).earliest())
                .map(|it| format!("{:?}", it))
        }
    }

    Ok(Box::new(DateParseFn { format_str }))
}

fn date_parse_elapsed_cn() -> StringFilter {
    struct DateParseElapsedCnFn;
    impl FilterFn for DateParseElapsedCnFn {
        fn invoke(&self, value: String) -> Option<String> {
            let regex = Regex::new(r"((\d+)日)?((\d+)[時时])?((\d+)分)?")
                .ok().and_then(|it| it.captures(&value));
            let regex_group = match regex {
                None => { return None; }
                Some(regex_group) => regex_group
            };

            let days = regex_group.get(1)
                .and_then(|it| it.as_str().parse::<i64>().ok())
                .map(|it| Duration::days(it));
            let hours = regex_group.get(2)
                .and_then(|it| it.as_str().parse::<i64>().ok())
                .map(|it| Duration::days(it));
            let minutes = regex_group.get(3)
                .and_then(|it| it.as_str().parse::<i64>().ok())
                .map(|it| Duration::days(it));

            let mut duration = Duration::zero();
            if let Some(days) = days { duration = duration + days }
            if let Some(hours) = hours { duration = duration + hours }
            if let Some(minutes) = minutes { duration = duration + minutes }

            Some(format!("{:?}", Local::now() - duration))
        }
    }
    Box::new(DateParseElapsedCnFn)
}

fn date_parse_elapsed_en() -> StringFilter {
    struct DateParseElapsedEnFn;
    impl FilterFn for DateParseElapsedEnFn {
        fn invoke(&self, value: String) -> Option<String> {
            let regex = Regex::new(r"([.\d]+)\s(seconds|minutes|hours|days|weeks|years)\sago")
                .ok().and_then(|it| it.captures(&value));
            let regex_group = match regex {
                None => { return None; }
                Some(regex_group) => regex_group
            };

            let num = regex_group.get(1)
                .and_then(|it| it.as_str().parse::<i64>().ok()).unwrap_or_default();
            let unit = regex_group.get(2)
                .map(|it| it.as_str()).unwrap_or("");

            let duration = match unit {
                "seconds" => Duration::seconds(num),
                "minutes" => Duration::minutes(num),
                "hours" => Duration::hours(num),
                "days" => Duration::days(num),
                "weeks" => Duration::weeks(num),
                "years" => Duration::days(num * 365),
                _ => Duration::zero()
            };
            Some(format!("{:?}", Local::now() - duration))
        }
    }
    Box::new(DateParseElapsedEnFn)
}
