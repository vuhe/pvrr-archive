use super::Token;
use ego_tree::NodeId;
use Category::*;

#[derive(Debug, Copy, Clone)]
enum Category {
    /// 开括号
    BracketOpen,
    /// 闭括号
    BracketClosed,
    /// 分隔符
    Delimiter,
    /// 未识别
    Unknown,
    /// 不可分割
    Fixed,
    /// 已识别
    Identifier,
}

type Text<'t> = std::borrow::Cow<'t, str>;

pub(crate) struct Item<'t> {
    /// tokens 类型
    category: Category,
    /// tokens 内容
    text: Text<'t>,
    /// 是否在括号内
    enclosed: bool,
}

impl<'t> Item<'t> {
    /// 仅在创建 tokens 初始化时使用
    pub(super) fn new(text: &'t str) -> Self {
        Item {
            category: Unknown,
            text: Text::Borrowed(text),
            enclosed: false,
        }
    }

    /// Safety: 整个生命周期内所有的 item 均不会被 drop，
    /// 且所有引用均来自于 token，可以保证此处引用有效
    fn text(&self) -> &'t str {
        unsafe { &*(self.text.as_ref() as *const _) as &'t str }
    }
}

type NodeRef<'a, 't> = ego_tree::NodeRef<'a, Item<'t>>;

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct ItemRef<'a, 't: 'a>(NodeRef<'a, 't>);

impl<'a, 't: 'a> ItemRef<'a, 't> {
    pub(super) fn wrap(node: NodeRef<'a, 't>) -> Self {
        Self(node)
    }

    pub(crate) fn id(&self) -> NodeId {
        self.0.id()
    }

    pub(crate) fn text(&self) -> &'t str {
        self.0.value().text()
    }

    pub(crate) fn enclosed(&self) -> bool {
        self.0.value().enclosed
    }

    pub(crate) fn is_open_bracket(&self) -> bool {
        matches!(self.0.value().category, BracketOpen)
    }

    pub(crate) fn is_closed_bracket(&self) -> bool {
        matches!(self.0.value().category, BracketClosed)
    }

    pub(crate) fn is_bracket(&self) -> bool {
        matches!(self.0.value().category, BracketOpen | BracketClosed)
    }

    pub(crate) fn is_delimiter(&self) -> bool {
        matches!(self.0.value().category, Delimiter)
    }

    pub(crate) fn is_unknown(&self) -> bool {
        matches!(self.0.value().category, Unknown | Fixed)
    }

    pub(crate) fn is_identifier(&self) -> bool {
        matches!(self.0.value().category, Identifier)
    }

    #[rustfmt::skip]
    pub(crate) fn prev_find<P: Fn(&Self) -> bool>(&self, p: P) -> Option<Self> {
        self.0.prev_siblings().map(Self::wrap).find(p)
    }

    #[rustfmt::skip]
    pub(crate) fn next_find<P: Fn(&Self) -> bool>(&self, p: P) -> Option<Self> {
        self.0.next_siblings().map(Self::wrap).find(p)
    }
}

pub(crate) struct ItemMut<'a, 't: 'a> {
    pub(super) id: NodeId,
    pub(super) token: &'a mut Token<'t>,
}

impl<'a, 't: 'a> ItemMut<'a, 't> {
    pub(super) fn new(id: NodeId, token: &'a mut Token<'t>) -> Self {
        Self { id, token }
    }

    fn get_ref(&self) -> ego_tree::NodeRef<'_, Item<'t>> {
        unsafe { self.token.linked_list.get_unchecked(self.id) }
    }

    fn get_mut(&mut self) -> ego_tree::NodeMut<'_, Item<'t>> {
        unsafe { self.token.linked_list.get_unchecked_mut(self.id) }
    }

    pub(crate) fn text(&self) -> &'t str {
        self.get_ref().value().text()
    }

    pub(super) fn enclosed(&self) -> bool {
        self.get_ref().value().enclosed
    }

    pub(super) fn can_split(&self) -> bool {
        matches!(self.get_ref().value().category, Unknown)
    }

    pub(crate) fn is_delimiter(&self) -> bool {
        matches!(self.get_ref().value().category, Delimiter)
    }

    fn insert(&mut self, category: Category, text: Text<'t>, enclosed: bool) -> ItemMut<'_, 't> {
        #[rustfmt::skip]
        let item = Item { category, text, enclosed };
        let id = self.get_mut().insert_before(item).id();
        unsafe { self.token.get_mut(id) }
    }

    pub(super) fn insert_open_bracket(&mut self) {
        self.insert(BracketOpen, Text::Borrowed("["), true);
    }

    pub(super) fn insert_closed_bracket(&mut self) {
        self.insert(BracketClosed, Text::Borrowed("]"), true);
    }

    pub(super) fn insert_delimiter(&mut self, text: Text<'t>, enclosed: bool) {
        self.insert(Delimiter, text, enclosed);
    }

    pub(super) fn insert_unknown(&mut self, text: Text<'t>, enclosed: bool) {
        self.insert(Unknown, text, enclosed);
    }

    pub(super) fn insert_fixed(&mut self, text: Text<'t>, enclosed: bool) {
        self.insert(Fixed, text, enclosed);
    }

    /// 如果 node 为 Unknown 或 Fixed，标记为 Identifier
    pub(crate) fn tag_identifier(&mut self) {
        if matches!(self.get_ref().value().category, Unknown | Fixed) {
            self.get_mut().value().category = Identifier
        }
    }

    pub(super) fn detach(&mut self) {
        self.get_mut().detach();
    }
}
