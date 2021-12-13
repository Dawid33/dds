use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use indextree::{Arena};

pub enum ElementKind {
    Div,
    None
}

pub struct Element {
    pub kind : ElementKind,
    pub inner : String,
}

impl Element{
    pub fn new() -> Element {
        Element {
            kind : ElementKind::None,
            inner : String::new(),
        }
    }
}