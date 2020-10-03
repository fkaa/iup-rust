use iup_sys;

use std::ptr;
use std::ffi::CString;

use Handle;
use Element;

pub struct Menu(*mut iup_sys::Ihandle);

impl_widget_container!(Menu, "menu");

impl Menu {
    pub fn new<A>(elems: A) -> Menu where A: AsRef<[Handle]>  {
        let mut carray = slice_to_ih_array!(elems.as_ref());
        unsafe { Menu::from_raw(iup_sys::IupMenuv(carray.as_mut_ptr())) }
    }
}

pub struct Submenu(*mut iup_sys::Ihandle);

impl_widget!(Submenu, "submenu");

impl Submenu {
    pub fn new<E: Element>(title: &str, child: E) -> Self {
        let ctitle = CString::new(title).unwrap();
        unsafe { Submenu::from_raw(iup_sys::IupSubmenu(ctitle.as_ptr(), child.raw())) }
    }
}

pub struct Item(*mut iup_sys::Ihandle);

impl_widget!(Item, "item");

impl Item {
    pub fn with_title(title: &str) -> Self {
        let ctitle = CString::new(title).unwrap();
        unsafe { Item::from_raw(iup_sys::IupItem(ctitle.as_ptr(), ptr::null_mut())) }
    }
}

impl ::callback::Action for Item {}

pub struct Separator(*mut iup_sys::Ihandle);

impl_widget!(Separator, "separator");

impl Separator {
    pub fn new() -> Self {
        unsafe { Separator::from_raw(iup_sys::IupSeparator()) }
    }
}
