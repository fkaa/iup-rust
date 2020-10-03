use iup_sys;
use std::ptr;

use Element;

pub struct Val(*mut iup_sys::Ihandle);

impl Val {
    /// Creates a label with no predefined text, image or separator.
    pub fn new() -> Self {
        unsafe { Val::from_raw(iup_sys::IupVal(ptr::null_mut())) }
    }
}

impl_widget!(Val, "val");
impl ::callback::MapCb for Val {}
impl ::callback::UnmapCb for Val {}
impl ::callback::EnterWindowCb for Val {}
impl ::callback::LeaveWindowCb for Val {}
