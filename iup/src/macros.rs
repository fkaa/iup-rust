
/// Converts a Rust string literal into a C null terminated string literal typed `c_char`.
#[macro_export]
macro_rules! cstr {
    ($str_lit:expr) => {{    // must be a literal!!!
        use libc::c_char;
        concat!($str_lit, '\0').as_ptr() as *const c_char
    }};
}

/// Converts a `*const c_char` pointer into a owned `String`.
macro_rules! string_from_cstr {
    ($c_str:expr) => {{
        use std::ffi::CStr;
        let cstr = $c_str;
        unsafe { String::from_utf8_lossy(CStr::from_ptr(cstr).to_bytes()).to_string() }
    }};
}

macro_rules! slice_to_ih_array {
    ($v:expr) => {{
        use std::ptr;
        let mut v = Vec::with_capacity($v.len() + 1);
        v.extend($v.iter().map(|elem| elem.raw()));
        v.push(ptr::null_mut());
        v
    }};    
}
