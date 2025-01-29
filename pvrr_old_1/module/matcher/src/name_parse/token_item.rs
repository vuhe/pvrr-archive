use super::token_text::TokenText;
use std::fmt::{Debug, Formatter};
use std::{cell::RefCell, mem::replace, ops::Add, rc::Rc};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Category {
    /// 开括号
    BracketOpen,
    /// 闭括号
    BracketClosed,
    /// 分隔符
    Delimiter,
    /// 未识别
    Unknown,
    /// 已识别
    Identifier,
    /// 已处理（失效）
    Invalid,
}

struct Item<'t> {
    /// tokens 类型
    category: Category,
    /// tokens 内容
    text: TokenText<'t>,
    /// 是否在括号内
    enclosed: bool,
}

#[derive(Clone)]
pub(crate) struct TokenRef<'t>(Rc<RefCell<Item<'t>>>);

impl<'t> TokenRef<'t> {
    pub(crate) fn bracket_open(text: TokenText<'t>, enclosed: bool) -> Self {
        let item = Item { category: Category::BracketOpen, text, enclosed };
        Self(Rc::new(RefCell::new(item)))
    }

    pub(crate) fn bracket_closed(text: TokenText<'t>, enclosed: bool) -> Self {
        let item = Item { category: Category::BracketClosed, text, enclosed };
        Self(Rc::new(RefCell::new(item)))
    }

    pub(crate) fn delimiter(text: TokenText<'t>, enclosed: bool) -> Self {
        let item = Item { category: Category::Delimiter, text, enclosed };
        Self(Rc::new(RefCell::new(item)))
    }

    pub(crate) fn unknown(text: TokenText<'t>, enclosed: bool) -> Self {
        let item = Item { category: Category::Unknown, text, enclosed };
        Self(Rc::new(RefCell::new(item)))
    }

    pub(crate) fn is_unknown(&self) -> bool {
        self.0.borrow().category == Category::Unknown
    }

    pub(crate) fn is_open_bracket(&self) -> bool {
        self.0.borrow().category == Category::BracketOpen
    }

    pub(crate) fn is_closed_bracket(&self) -> bool {
        self.0.borrow().category == Category::BracketClosed
    }

    pub(crate) fn is_delimiter(&self) -> bool {
        self.0.borrow().category == Category::Delimiter
    }

    pub(crate) fn is_identifier(&self) -> bool {
        self.0.borrow().category == Category::Identifier
    }

    pub(crate) fn is_valid(&self) -> bool {
        self.0.borrow().category != Category::Invalid
    }

    pub(crate) fn enclosed(&self) -> bool {
        self.0.borrow().enclosed
    }

    pub(crate) fn set_unknown(&mut self) {
        self.0.borrow_mut().category = Category::Unknown;
    }

    pub(crate) fn set_identifier(&mut self) {
        self.0.borrow_mut().category = Category::Identifier;
    }

    pub(crate) fn to_text(&self) -> TokenText<'t> {
        self.0.borrow().text.clone()
    }
}

impl PartialEq for TokenRef<'_> {
    fn eq(&self, other: &Self) -> bool {
        (&*self.0.borrow() as *const Item) == (&*other.0.borrow() as *const Item)
    }
}

impl<'t> Add for TokenRef<'t> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut rhs = rhs.0.borrow_mut();
        rhs.category = Category::Invalid;
        let rhs = replace(&mut rhs.text, TokenText::default());
        self.0.borrow_mut().text += rhs;
        self
    }
}

impl Debug for TokenRef<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let item = self.0.borrow();
        if item.enclosed {
            write!(f, "[{:?}({})]", item.category, item.text)
        } else {
            write!(f, "{:?}({})", item.category, item.text)
        }
    }
}
