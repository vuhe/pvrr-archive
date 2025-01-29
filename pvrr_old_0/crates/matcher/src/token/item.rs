use super::Category;
use base_tool::text::Text;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

struct Item {
    /// token 类型
    category: Category,
    /// token 内容
    text: Text,
    /// 是否在括号内
    enclosed: bool,
}

pub(super) struct TokenRef(Rc<RefCell<Item>>);

impl TokenRef {
    pub(super) fn new(category: Category, text: Text, enclosed: bool) -> Self {
        let item = Item { category, text, enclosed };
        Self(Rc::new(RefCell::new(item)))
    }

    pub(super) fn category(&self) -> Category {
        self.0.borrow().category
    }

    pub(super) fn text(&self) -> Text {
        self.0.borrow().text.clone()
    }

    pub(super) fn enclosed(&self) -> bool {
        self.0.borrow().enclosed
    }

    pub(super) fn set_category(&mut self, category: Category) {
        self.0.borrow_mut().category = category
    }
}

impl<'text> PartialEq for TokenRef {
    fn eq(&self, other: &Self) -> bool {
        (&*self.0.borrow() as *const Item) == (&*other.0.borrow() as *const Item)
    }
}

impl Clone for TokenRef {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Debug for TokenRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let item = self.0.borrow();
        if item.enclosed {
            write!(f, "[{:?}({})]", item.category, item.text)
        } else {
            write!(f, "{:?}({})", item.category, item.text)
        }
    }
}
