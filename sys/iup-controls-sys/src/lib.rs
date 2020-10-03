#![allow(dead_code, non_camel_case_types)]

use iup_sys::Ihandle;
use libc::{c_int, c_char};

// Force linking with cd_sys
#[allow(unused_imports)]
use cd_sys::CD_VERSION;

extern "C" {
    pub fn IupControlsOpen() -> c_int;

    pub fn IupCells() -> *mut Ihandle;
    pub fn IupMatrix(action: *const c_char) -> *mut Ihandle;
    pub fn IupMatrixList() -> *mut Ihandle;
    pub fn IupMatrixEx() -> *mut Ihandle;
}
