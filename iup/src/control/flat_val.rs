use iup_sys;
use std::ptr;

use Element;

pub struct FlatVal(*mut iup_sys::Ihandle);

impl FlatVal {
    /// Creates a label with no predefined text, image or separator.
    pub fn new() -> Self {
        unsafe { FlatVal::from_raw(iup_sys::IupFlatVal(ptr::null_mut())) }
    }
}

impl_widget!(FlatVal, "flatval");
impl ::callback::ValueChangedCb for FlatVal {}
impl ::callback::MapCb for FlatVal {}
impl ::callback::UnmapCb for FlatVal {}
impl ::callback::EnterWindowCb for FlatVal {}
impl ::callback::LeaveWindowCb for FlatVal {}
