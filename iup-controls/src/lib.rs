#[macro_use]
extern crate iup;

use std::ptr;

use iup_sys;
use libc::{c_char, c_int};

use iup::Element;

pub fn open() -> Result<(), iup::InitError> {
    use iup::InitError;

    match unsafe { iup_controls_sys::IupControlsOpen() } {
        iup_sys::IUP_NOERROR => {}
        iup_sys::IUP_OPENED => return Err(InitError::AlreadyOpen),
        iup_sys::IUP_ERROR => return Err(InitError::Error),
        _ => unreachable!(),
    };

    Ok(())
}

pub struct Matrix(*mut iup_sys::Ihandle);

impl Matrix {
    pub fn new() -> Self {
        unsafe { Matrix(iup_controls_sys::IupMatrix(ptr::null_mut())) }
    }
}

impl_widget!(Matrix, "matrix");

impl iup::callback::MapCb for Matrix {}
impl iup::callback::UnmapCb for Matrix {}
impl iup::callback::GetFocusCb for Matrix {}
impl iup::callback::ResizeCb for Matrix {}
impl iup::callback::KillFocusCb for Matrix {}
impl iup::callback::EnterWindowCb for Matrix {}
impl iup::callback::LeaveWindowCb for Matrix {}
impl iup::callback::HelpCb for Matrix {}


impl self::BgColorCb for Matrix {}
impl_callback! {
    pub trait BgColorCb where Self: Element {
        let name = "BGCOLOR_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle, lin: c_int, col: c_int, red: *mut c_int, green: *mut c_int, blue: *mut c_int) -> CallbackReturn;
        fn set_bg_color_cb<F: Callback(Self, i32, i32, *mut c_int, *mut c_int, *mut c_int)>(&mut self, cb: F) -> Self;
        fn remove_bg_color_cb(&mut self) -> Option<Box<_>>;
    }
}

impl self::FgColorCb for Matrix {}
impl_callback! {
    pub trait FgColorCb where Self: Element {
        let name = "FGCOLOR_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle, lin: c_int, col: c_int, red: *mut c_int, green: *mut c_int, blue: *mut c_int) -> CallbackReturn;
        fn set_fg_color_cb<F: Callback(Self, i32, i32, *mut c_int, *mut c_int, *mut c_int)>(&mut self, cb: F) -> Self;
        fn remove_fg_color_cb(&mut self) -> Option<Box<_>>;
    }
}

impl self::ClickCb for Matrix {}
impl_callback! {
    pub trait ClickCb where Self: Element {
        let name = "CLICK_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle, lin: c_int, col: c_int, status: *const c_char) -> CallbackReturn;
        fn set_click_cb<F: Callback(Self, i32, i32, String)>(&mut self, cb: F) -> Self;
        fn remove_click_cb(&mut self) -> Option<Box<_>>;
    }
}

impl self::ValueCb for Matrix {}
impl_callback! {
    pub trait ValueCb where Self: Element {
        let name = "VALUE_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle, lin: c_int, col: c_int) -> *mut c_char;
        fn set_value_cb<F: Callback(Self, i32, i32) -> *mut c_char>(&mut self, cb: F) -> Self;
        fn remove_value_cb(&mut self) -> Option<Box<_>>;
    }
}

impl self::MarkCb for Matrix {}
impl_callback! {
    pub trait MarkCb where Self: Element {
        let name = "MARK_CB";
        extern fn listener(ih: *mut iup_sys::Ihandle, lin: c_int, col: c_int) -> c_int;
        fn set_mark_cb<F: Callback(Self, i32, i32) -> c_int>(&mut self, cb: F) -> Self;
        fn remove_mark_cb(&mut self) -> Option<Box<_>>;
    }
}
