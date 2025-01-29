mod item;
mod tokenize;

use ego_tree::{NodeId, Tree};
pub(crate) use item::ItemRef;
use item::{Item, ItemMut};

type NeedSplit<'a, 't: 'a> = impl Iterator<Item = ItemMut<'a, 't>>;
type Unknowns<'a, 't: 'a> = impl DoubleEndedIterator<Item = ItemRef<'a, 't>>;
type Subs<'a, 't: 'a> = impl Iterator<Item = ItemMut<'a, 't>>;

pub(crate) struct Token<'t> {
    linked_list: Tree<Item<'t>>,
}

impl<'t> Token<'t> {
    pub(crate) fn new(text: &'t str) -> Self {
        let mut linked_list = Tree::new(Item::new(""));
        linked_list.root_mut().append(Item::new(text));
        let mut token = Self { linked_list };
        tokenize::split_token(&mut token);
        token
    }

    // /// 获取一个 item 引用，不会进行边界检查
    // pub(crate) unsafe fn get(&self, idx: NodeId) -> ItemRef<'_, 't> {
    //     self.linked_list.get_unchecked(idx)
    // }
    //
    /// 获取一个 item 可变引用，不会进行边界检查
    pub(crate) unsafe fn get_mut(&mut self, id: NodeId) -> ItemMut<'_, 't> {
        ItemMut::new(id, self)
    }

    /// 首个 token
    pub(crate) fn first(&self) -> Option<ItemRef<'_, 't>> {
        self.linked_list.root().first_child().map(ItemRef::wrap)
    }

    /// 最后一个 token
    pub(crate) fn last(&self) -> Option<ItemRef<'_, 't>> {
        self.linked_list.root().last_child().map(ItemRef::wrap)
    }

    // /// 首个开括号
    // pub(crate) fn first_open_bracket(&self) -> Option<ItemRef<'_, 't>> {
    //     self.linked_list
    //         .root()
    //         .children()
    //         .find(|it| it.is_open_bracket())
    // }
    //
    /// 首个未识别 token
    pub(crate) fn first_unknown(&self) -> Option<ItemRef<'_, 't>> {
        self.unknown_tokens().next()
    }

    /// 需要切分 && 未识别的 token
    fn need_split_tokens(&mut self) -> NeedSplit<'_, 't> {
        let start = self.linked_list.root().first_child().map(|it| it.id());
        let end = None;
        let token = self;
        NextMutTokens { token, start, end }.filter(ItemMut::can_split)
    }

    /// tokens 的未识别 token 切片
    pub(crate) fn unknown_tokens(&self) -> Unknowns<'_, 't> {
        let root = self.linked_list.root();
        let start = root.first_child().map(ItemRef::wrap);
        let end = root.last_child().map(ItemRef::wrap);
        NextRefTokens { start, end }.filter(ItemRef::is_unknown)
    }

    /// tokens 的 [start, end) 切片，不检查 start 和 end 的合法性
    pub(crate) unsafe fn sub_tokens(&mut self, start: NodeId, end: Option<NodeId>) -> Subs<'_, 't> {
        let start = Some(start);
        let token = self;
        NextMutTokens { token, start, end }
    }
}

struct NextMutTokens<'a, 't: 'a> {
    token: &'a mut Token<'t>,
    start: Option<NodeId>,
    end: Option<NodeId>,
}

impl<'a, 't: 'a> Iterator for NextMutTokens<'a, 't> {
    type Item = ItemMut<'a, 't>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end || self.start == None {
            None
        } else {
            let node = self.start.take();
            let next = node
                .and_then(|id| self.token.linked_list.get(id))
                .and_then(|it| it.next_sibling())
                .map(|it| it.id());
            self.start = next;
            // Safety: 此处扩展了 token 的生命周期，如果不及时 drop item 可能会造成借用冲突
            // 通常此迭代器仅用于循环中，不会出现借用冲突的情况
            node.map(|id| unsafe {
                (&mut *(self.token as *mut _) as &'a mut Token<'t>).get_mut(id)
            })
        }
    }
}

struct NextRefTokens<'a, 't: 'a> {
    start: Option<ItemRef<'a, 't>>,
    end: Option<ItemRef<'a, 't>>,
}

impl<'a, 't: 'a> Iterator for NextRefTokens<'a, 't> {
    type Item = ItemRef<'a, 't>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            let node = self.start.take();
            self.end = None;
            node
        } else {
            let node = self.start.take();
            // 获取后一个 node，不进行任何过滤
            self.start = node.as_ref().and_then(|it| it.next_find(|_| true));
            node
        }
    }
}

impl<'a, 't> DoubleEndedIterator for NextRefTokens<'a, 't> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end == self.start {
            let node = self.end.take();
            self.start = None;
            node
        } else {
            let node = self.end.take();
            // 获取前一个 node，不进行任何过滤
            self.end = node.as_ref().and_then(|it| it.prev_find(|_| true));
            node
        }
    }
}
