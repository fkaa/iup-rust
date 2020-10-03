use iup_sys;
use std::ptr;

use Element;

/// See the [IUP Frame Documentation][1].
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/elem/iupframe.html
pub struct FlatFrame(*mut iup_sys::Ihandle);

impl FlatFrame {
    /// Creates a frame with a child element.
    pub fn new<E: Element>(child: E) -> FlatFrame {
        unsafe { FlatFrame::from_raw(iup_sys::IupFlatFrame(child.raw())) }
    }

    /// Creates a frame with no elements.
    pub fn new_empty() -> FlatFrame {
        unsafe { FlatFrame::from_raw(iup_sys::IupFlatFrame(ptr::null_mut())) }
    }
}

impl_widget_container!(FlatFrame, "flatframe");
impl ::callback::MapCb for FlatFrame {}
impl ::callback::UnmapCb for FlatFrame {}
