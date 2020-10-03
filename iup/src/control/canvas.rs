use iup_sys;
use std::ptr;

use Element;

pub struct Canvas(*mut iup_sys::Ihandle);

impl Canvas {
    /// Creates a label with no predefined text, image or separator.
    pub fn new() -> Self {
        unsafe { Canvas::from_raw(iup_sys::IupCanvas(ptr::null_mut())) }
    }
}

impl_widget!(Canvas, "canvas");
impl ::callback::ActionCb for Canvas {}
impl ::callback::MapCb for Canvas {}
impl ::callback::UnmapCb for Canvas {}
impl ::callback::EnterWindowCb for Canvas {}
impl ::callback::LeaveWindowCb for Canvas {}
