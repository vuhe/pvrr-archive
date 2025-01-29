#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub(super) enum Category {
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
