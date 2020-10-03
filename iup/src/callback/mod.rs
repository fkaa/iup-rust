//! Event-driven communication.

use iup_sys;
use libc::{c_char, c_double, c_int, c_void};
use std::path::PathBuf;
use std::char;

#[macro_use]
mod macros;
pub mod callbacks;
pub mod key;
pub use self::callbacks::*;

pub mod button;

// This is called right when a IUP element is being destroyed and it should free up all data
// associated with callbacks. Just use the `drop_callback!` macro for each callback implemented.
#[doc(hidden)]
pub unsafe fn drop_callbacks(ih: *mut iup_sys::Ihandle) {
    // Prehaps this isn't the best way to get on it as we might forget to add things...

    // button.rs
    drop_callback!(ih, "BUTTON_CB");
    drop_callback!(ih, "MOTION_CB");

    // callbacks.rs
    drop_callback!(ih, "ACTION");
    drop_callback!(ih, "ACTION_CB");
    drop_callback!(ih, "LDESTROY_CB");
    drop_callback!(ih, "MAP_CB");
    drop_callback!(ih, "UNMAP_CB");
    drop_callback!(ih, "GETFOCUS_CB");
    drop_callback!(ih, "KILLFOCUS_CB");
    drop_callback!(ih, "ENTERWINDOW_CB");
    drop_callback!(ih, "LEAVEWINDOW_CB");
    drop_callback!(ih, "HELP_CB");
    drop_callback!(ih, "CARET_CB");
    drop_callback!(ih, "SPIN_CB");
    drop_callback!(ih, "VALUECHANGED_CB");
    drop_callback!(ih, "DROPFILES_CB");
    drop_callback!(ih, "CLOSE_CB");
    drop_callback!(ih, "MOVE_CB");
    drop_callback!(ih, "RESIZE_CB");

    // dialog.rs
    drop_callback!(ih, "COPYDATA_CB");
    drop_callback!(ih, "MDIACTIVATE_CB");
    drop_callback!(ih, "SHOW_CB");
    drop_callback!(ih, "TRAYCLICK_CB");
}




/// Return this from a callback to tell the framework a non-default action to be performed.
///
/// Not all callbacks accepts `Close`, `Ignore` or `Continue`, check their respective docs.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CallbackReturn {
    /// The default `CallbackReturn`, does nothing when returned.
    Default,
    /// If this is returned from a callback, then when the callback returns the dialog containing
    /// the element on which the callback was invoked will be closed.
    Close,
    /// Callback specific, check the callback documentation to see if it accepts this return value
    /// and it's effect.
    Ignore,
    /// Callback specific, check the callback documentation to see if it accepts this return value
    /// and it's effect.
    Continue,
    /// Callback specific, check the callback documentation to see if it accepts this return value
    /// and it's effect.
    Char(char),
}

impl From<CallbackReturn> for c_int {
    fn from(cb: CallbackReturn) -> c_int {
        use self::CallbackReturn::*;
        match cb {
            Close => iup_sys::IUP_CLOSE,
            Default => iup_sys::IUP_DEFAULT,
            Ignore => iup_sys::IUP_IGNORE,
            Continue => iup_sys::IUP_CONTINUE,
            Char(c) => c as c_int,
        }
    }
}

// This allows returning '()' from a callback instead of CallbackReturn.
impl From<()> for CallbackReturn {
    fn from(_: ()) -> CallbackReturn {
        CallbackReturn::Default
    }
}

pub trait Callback<Args, Return> : 'static {
    fn on_callback(&mut self, args: Args) -> Return; 
}

impl<Args, Return, Out: Into<Return>, F: 'static> Callback<Args, Return> for F where F: FnMut(Args) -> Out {
    /// Because of the `impl From<()> for CallbackReturn`, closures that return `()` can be
    /// accepted by this impl.
    fn on_callback(&mut self, args: Args) -> Return {
        let r = self(args).into();
        r
    }
}

/// This is a internal trait used to convert IUP C types into IUP Rust types in callbacks.
///
/// For instance BUTTON_CB has a `char* status` parameter that must be abstracted into another
/// type (e.g. `KeyStatus`).
///
/// This trait method `into_rust` is called from the `impl_callback!` macro.
#[doc(hidden)]
pub trait IntoRust<T> {
    fn into_rust(self) -> T;
}

impl IntoRust<i32> for c_int {
    fn into_rust(self) -> i32 {
        self as i32
    }
}

impl IntoRust<u32> for c_int {
    fn into_rust(self) -> u32 {
        self as u32
    }
}

impl IntoRust<usize> for c_int {
    fn into_rust(self) -> usize {
        self as usize
    }
}

impl IntoRust<f64> for c_double {
    fn into_rust(self) -> f64 {
        self as f64
    }
}

impl IntoRust<bool> for c_int {
    fn into_rust(self) -> bool {
        self != 0
    }
}

impl IntoRust<PathBuf> for *const c_char {
    fn into_rust(self) -> PathBuf {
        PathBuf::from(string_from_cstr!(self))
    }
}

impl IntoRust<String> for *const c_char {
    fn into_rust(self) -> String {
        string_from_cstr!(self)
    }
}

impl IntoRust<*mut ()> for *mut () {
    fn into_rust(self) -> *mut () {
        self
    }
}

impl IntoRust<*mut c_void> for *mut c_void {
    fn into_rust(self) -> *mut c_void {
        self
    }
}

impl IntoRust<*mut c_int> for *mut c_int {
    fn into_rust(self) -> *mut c_int {
        self
    }
}

impl IntoRust<Option<char>> for c_int {
    fn into_rust(self) -> Option<char> {
        if self == 0 { None } else { Some(char::from_u32(self as u32).unwrap()) }
    }
}
