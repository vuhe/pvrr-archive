mod cn_duration_parse;
mod en_duration_parse;
mod query_string;
mod replace;

use crate::filter::cn_duration_parse::CnDurationParse;
use crate::filter::en_duration_parse::EnDurationParse;
use crate::filter::query_string::QueryString;
use crate::filter::replace::RegexReplace;
use crate::values::ValArgs;
use serde::Deserialize;
use std::borrow::Cow;

trait FilterTrait: Send + Sync + Sized {
    fn invoke(&self, input: Option<String>) -> Option<String>;
}

#[derive(Deserialize)]
struct FilterBuilder<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    args: Option<ValArgs>,
}

#[derive(Deserialize)]
#[serde(try_from = "FilterBuilder<'_>")]
pub(crate) struct Filter(Box<dyn FilterTrait>);

impl Filter {
    pub(crate) fn invoke(&self, input: Option<String>) -> Option<String> {
        self.0.invoke(input)
    }
}

impl TryFrom<FilterBuilder<'_>> for Filter {
    type Error = String;

    fn try_from(value: FilterBuilder<'_>) -> Result<Self, Self::Error> {
        match &*value.name {
            "query_string" => {
                let args = match value.args {
                    None => return Err("缺少 args 参数".to_string()),
                    Some(it) => it,
                };
                QueryString::new(args).map_err(|it| it.to_string()).map(|it| Filter(Box::new(it)))
            },
            "replace" => {
                let args = match value.args {
                    None => return Err("缺少 args 参数".to_string()),
                    Some(it) => it,
                };
                RegexReplace::new(args).map_err(|it| it.to_string()).map(|it| Filter(Box::new(it)))
            },
            "en_duration_parse" => Ok(Filter(Box::new(EnDurationParse))),
            "cn_duration_parse" => Ok(Filter(Box::new(CnDurationParse))),
            _ => Err(format!("不支持 {name} 过滤器")),
        }
    }
}
