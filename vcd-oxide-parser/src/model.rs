use core::cell::RefCell;
use std::rc::{Rc, Weak};

type VcdScopeNode = Rc<RefCell<ValueChangeDumpScope>>;

#[derive(Debug, Default)]
pub struct ValueChangeDump {
    pub date: String,
    pub version: String,
    pub timescale: String,
    // TODO: Validate this but there's only one root scope
    // This scope can have multiple child scopes
    pub root_scope: VcdScopeNode,
}

#[derive(Debug, Default)]
pub struct ValueChangeDumpScope {
    pub name: String,
    pub kind: String,
    pub scopes: Vec<Rc<RefCell<ValueChangeDumpScope>>>,
    pub parent: Option<Weak<RefCell<ValueChangeDumpScope>>>,
    pub signals: Vec<ValueChangeDumpSignal>,
}

#[derive(Debug, Default)]
pub struct ValueChangeDumpSignal {
    pub kind: String,
    pub identifier: String,
    pub reference: String,
    pub size: usize,
}
