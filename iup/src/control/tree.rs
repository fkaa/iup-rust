use iup_sys;

use std::ffi::CString;
use std::cell::RefCell;
use callback::callbacks::DestroyCb;

use Element;

pub struct Tree(*mut iup_sys::Ihandle);

impl Tree {
    pub fn new() -> Self {
        let tree = unsafe { Tree(iup_sys::IupTree()) };
        tree.set_userdata_vec();

        let destroy_tree = tree.clone();
        tree.set_destroy_cb(move |_this| {
            destroy_tree.remove_userdata_vec();
        });

        tree
    }

    fn set_userdata_vec(&self) {
        let vec = Box::new(RefCell::new(Vec::new()));
        let ptr = Box::into_raw(vec);
        self.set_attrib_data("_IUPRUST_TREE_UD", ptr as *mut RefCell<Vec<CString>> as _);
    }

    fn get_userdata_vec(&self) -> *mut RefCell<Vec<CString>> {
        self.attrib_data("_IUPRUST_TREE_UD") as *mut RefCell<Vec<CString>>
    }

    fn remove_userdata_vec(&self) {
        unsafe { Box::from_raw(self.get_userdata_vec()); }
    }

    pub fn set_userdata<S: Into<String>>(&self, idx: i32, value: S) -> Self {
        let cvalue = CString::new(value.into()).unwrap();
        let cvalue_ptr = cvalue.as_ptr();

        println!("Setting UD({})={:?}", idx, cvalue);

        let ud_vec = self.get_userdata_vec();
        unsafe { (*ud_vec).borrow_mut().push(cvalue); }

        let val = unsafe { iup_sys::IupTreeSetUserId(self.0, idx, cvalue_ptr as _) };
        dbg!(val);
        self.clone()
    }

    pub fn userdata(&self, idx: i32) -> Option<String> {
        match unsafe { iup_sys::IupTreeGetUserId(self.raw(), idx) } {
            cvalue if cvalue.is_null() => None,
            cvalue => Some(string_from_cstr!(cvalue as _)),
        }
    }
}

impl_widget!(Tree, "tree");

impl ::callback::MapCb for Tree {}
impl ::callback::UnmapCb for Tree {}
impl ::callback::GetFocusCb for Tree {}
impl ::callback::KillFocusCb for Tree {}
impl ::callback::EnterWindowCb for Tree {}
impl ::callback::LeaveWindowCb for Tree {}
impl ::callback::HelpCb for Tree {}

impl self::SelectionCb for Tree {}
impl_callback! {
    pub trait SelectionCb where Self: Element {
        let name = "SELECTION_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle, c: c_int, s: c_int) -> CallbackReturn;
        fn set_selection_cb<F: Callback(Self, i32, bool)>(&mut self, cb: F) -> Self;
        fn remove_action(&mut self) -> Option<Box<_>>;
    }
}
