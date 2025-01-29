use anyhow::{anyhow, bail};
use scraper::{Element, ElementRef};
use serde_yaml::{Value, Mapping};
use super::{build_selector, string_filter::StringFilter, string_filter::build_filter};

/// 选择器定位参数检查
fn select_check(yml: &Value) -> bool {
    let selector = yml.get("selector");
    let case = yml.get("case");
    let has_noe = selector.xor(case).is_some();
    has_noe
}

/// 选择器取值参数检查
fn param_check(yml: &Value) -> bool {
    let attribute = yml.get("attribute");
    let method = yml.get("method");
    let remove = yml.get("remove");
    let contents = yml.get("contents");
    let all_null = attribute.is_none() && method.is_none()
        && remove.is_none() && contents.is_none();
    let has_one = attribute.xor(method)
        .xor(remove).xor(contents).is_some();
    all_null || has_one
}

/// 将此节点下的所有 text 连接为 string
fn ele_text_value(ele: &ElementRef) -> String {
    ele.text().fold(String::default(), |acc, it| acc + it)
}

pub(crate) struct FieldParser {
    // 选择器定位参数
    selector: Option<String>,
    index: Option<usize>,
    case: Option<Mapping>,
    // 选择器取值参数
    attribute: Option<String>,
    method: Option<String>,
    remove: Option<String>,
    contents: Option<usize>,
    // 过滤处理
    filters: Vec<StringFilter>,
    default_value: Option<String>,
}

impl FieldParser {
    pub(crate) fn from(yml: &Value) -> anyhow::Result<Self> {
        if !select_check(yml) { bail!("参数 selector, case 设置错误") }
        if !param_check(yml) { bail!("参数 attribute, method, remove, contents 设置错误") }

        let selector = yml.get("selector")
            .and_then(|it| it.as_str())
            .map(|it| it.to_owned());
        let index = yml.get("index")
            .and_then(|it| it.as_u64())
            .map(|it| it as usize);
        let case = yml.get("case")
            .and_then(|it| it.as_mapping())
            .map(|it| it.clone());

        let attribute = yml.get("attribute")
            .and_then(|it| it.as_str())
            .map(|it| it.to_owned());
        let method = yml.get("method")
            .and_then(|it| it.as_str())
            .map(|it| it.to_owned());
        let remove = yml.get("remove")
            .and_then(|it| it.as_str())
            .map(|it| it.to_owned());
        let contents = yml.get("contents")
            .and_then(|it| it.as_u64())
            .map(|it| it as usize);

        let default_value = yml.get("default_value")
            .and_then(|it| it.as_str())
            .map(|it| it.to_owned());

        let mut filters = Vec::new();
        if let Some(filter_it) = yml.get("filters").and_then(|it| it.as_sequence()) {
            for f in filter_it.iter() {
                let name = f.get("name").and_then(|it| it.as_str())
                    .ok_or(anyhow!("filter 参数 name 设置错误"))?;
                let args = f.get("args").and_then(|it| it.as_sequence())
                    .ok_or(anyhow!("filter 参数 args 设置错误"))?;
                let filter = build_filter(name, args)
                    .map_err(|e| anyhow!("filter 设置错误: {}", e))?;
                filters.push(filter)
            }
        }

        Ok(FieldParser {
            selector,
            index,
            case,
            attribute,
            method,
            remove,
            contents,
            filters,
            default_value,
        })
    }
}

impl FieldParser {
    pub(crate) fn parse(&self, dom: &ElementRef) -> anyhow::Result<String> {
        // get html value
        let value = if self.selector.is_some() {
            self.select(dom).and_then(|it| self.find_value(&it))
        } else if self.case.is_some() {
            self.case(dom)
        } else { bail!("参数 selector, case 设置错误") };

        // filter and map value
        let mut value = Some(value?);
        for filter in &self.filters {
            value = value.and_then(|it| filter.invoke(it))
        }
        if self.default_value.is_some() {
            value = value.or(self.default_value.clone())
        }

        Ok(value.unwrap())
    }

    /// case select value
    fn case(&self, dom: &ElementRef) -> anyhow::Result<String> {
        let mut it = self.case.as_ref().unwrap().iter();
        it.find(|&it| {
            let selector = it.0.as_str();
            if let Some(selector) = selector {
                match selector {
                    "*" => true,
                    _ => build_selector(selector).map(|it|
                        dom.select(&it).next().is_some()
                    ).unwrap_or(false)
                }
            } else { false }
        }).and_then(|it| it.1.as_str())
            .map(|it| it.to_owned())
            .ok_or(anyhow!("case 未匹配"))
    }

    /// find css element
    fn select<'a>(&'a self, dom: &'a ElementRef) -> anyhow::Result<ElementRef<'a>> {
        let selector = self.selector.as_ref().unwrap();
        let selector = build_selector(selector)?;
        let index = self.index.unwrap_or(0);
        dom.select(&selector).skip(index).next()
            .ok_or(anyhow!("select 未匹配"))
    }

    /// selected value find
    fn find_value(&self, ele: &ElementRef) -> anyhow::Result<String> {
        let value = if let Some(attr) = self.attribute.as_ref() {
            ele.value().attr(attr).map(|it| it.to_owned())
        } else if let Some(method) = self.method.as_ref() {
            match method.as_str() {
                "next_sibling" => ele.next_sibling_element()
                    .map(|it| ele_text_value(&it)),
                _ => None
            }
        } else if let Some(_remove) = self.remove.as_ref() {
            todo!("")
        } else if let Some(index) = self.contents.as_ref() {
            ele.children()
                .filter(|it| it.value().is_element())
                .skip(*index).next()
                .and_then(|it| ElementRef::wrap(it))
                .map(|it| ele_text_value(&it))
        } else {
            Some(ele_text_value(ele))
        };

        value.map(|it| it.trim().to_owned())
            .filter(|it| !it.is_empty())
            .ok_or(anyhow!("select 选择值为空"))
    }
}
