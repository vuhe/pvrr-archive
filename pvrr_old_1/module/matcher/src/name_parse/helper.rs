use super::token_item::TokenRef;
use super::token_text::TokenText;
use once_cell::sync::OnceCell;
use regex::Regex;
use std::ops::Deref;

pub(crate) struct LazyRegex {
    re: &'static str,
    cell: OnceCell<Regex>,
}

impl LazyRegex {
    pub(crate) const fn new(re: &'static str) -> Self {
        Self { re, cell: OnceCell::new() }
    }
}

impl Deref for LazyRegex {
    type Target = Regex;

    fn deref(&self) -> &Self::Target {
        self.cell.get_or_init(|| Regex::new(self.re).unwrap())
    }
}

static CN_U16_NUM: LazyRegex = LazyRegex::new("(?P<u1>一?千)?零?(?P<u2>(?P<n2>[一二三四五六七八九])?百)?零?(?P<u3>(?P<n3>[一二三四五六七八九])?十)?(?P<n4>[一二三四五六七八九])?");

pub(crate) trait StrExtra {
    fn is_not_empty(&self) -> bool;
    fn is_ascii_digit(&self) -> bool;
    /// 解析文字中的中文数字或者阿拉伯数字，
    /// 此模块下的所有数字均小于 [u16::MAX]，因此如果解析失败则返回 [u16::MAX]
    fn auto_to_u16(&self) -> u16;
}

impl<T: AsRef<str>> StrExtra for T {
    fn is_not_empty(&self) -> bool {
        !self.as_ref().is_empty()
    }

    fn is_ascii_digit(&self) -> bool {
        self.is_not_empty() && self.as_ref().chars().all(|it| it.is_ascii_digit())
    }

    fn auto_to_u16(&self) -> u16 {
        let num = self.as_ref();
        if let Ok(it) = num.parse::<u16>() {
            return it;
        }
        if num == "零" {
            return 0;
        }
        fn number_map(str: &str) -> Option<u16> {
            match str {
                "一" => Some(1),
                "二" => Some(2),
                "三" => Some(3),
                "四" => Some(4),
                "五" => Some(5),
                "六" => Some(6),
                "七" => Some(7),
                "八" => Some(8),
                "九" => Some(9),
                _ => None,
            }
        }
        let group = match CN_U16_NUM.captures(num) {
            None => return u16::MAX,
            Some(it) => it,
        };
        let mut number: u16 = 0;
        if group.name("u1").is_some() {
            number = number + 1000;
        }
        if group.name("u2").is_some() {
            let n = group.name("n2").and_then(|it| number_map(it.as_str())).unwrap_or(1);
            number = number + 100 * n;
        }
        if group.name("u3").is_some() {
            let n = group.name("n3").and_then(|it| number_map(it.as_str())).unwrap_or(1);
            number = number + 10 * n;
        }
        if let Some(n) = group.name("n4") {
            let n = number_map(n.as_str()).unwrap_or(0);
            number = number + n;
        }
        return number;
    }
}

impl StrExtra for TokenText<'_> {
    fn is_not_empty(&self) -> bool {
        (&**self).is_not_empty()
    }

    fn is_ascii_digit(&self) -> bool {
        (&**self).is_ascii_digit()
    }

    fn auto_to_u16(&self) -> u16 {
        (&**self).auto_to_u16()
    }
}

impl StrExtra for TokenRef<'_> {
    fn is_not_empty(&self) -> bool {
        self.to_text().is_not_empty()
    }

    fn is_ascii_digit(&self) -> bool {
        self.to_text().is_ascii_digit()
    }

    fn auto_to_u16(&self) -> u16 {
        self.to_text().auto_to_u16()
    }
}
