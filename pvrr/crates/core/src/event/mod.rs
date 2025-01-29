use std::any::Any;

/// 内部标识，禁止其他模块 impl
pub(super) mod private {
    pub trait Sealed {}
}

/// 事件，用于传递信息
pub trait Event: Any + Clone + private::Sealed {}

/// 事件总线
pub struct EventBus;
