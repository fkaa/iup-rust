/// Turns a string literal into a boxed closure attribute.
#[macro_export]
macro_rules! fbox_c_str {
    ($cb_name:expr) => {
        // It's important to use the prefix '_IUP*', it's reserved by IUP for internal use and bindings.
        // So we use '_IUPRUST_*' prefix to refer to data reserved for the Rust binding.
        cstr!(concat!("_IUPRUST_FBOX_", $cb_name))
    }
}

/// Sets a closure as a callback to IUP.
///
/// Note: `$ih` can be a `ptr::null_mut` to set a callback in the global enviroment.
#[macro_export]
macro_rules! set_fbox_callback {
    ($ih:expr, $cb_name:expr, $clistener:expr, $rcb:expr, Callback<$($rargs:ty),*>, $cb_return:ty) => {{

        use $crate::iup_sys;

        clear_fbox_callback!($ih, $cb_name, Callback<$($rargs),*>, $cb_return);

        let ih = $ih;
        let fb: Box<Box<dyn $crate::callback::Callback<$($rargs),*, $cb_return>>> = Box::new(Box::new($rcb));
        iup_sys::IupSetAttribute(ih, fbox_c_str!($cb_name), Box::into_raw(fb) as *const _);
        if ih.is_null() {
            iup_sys::IupSetFunction(cstr!($cb_name), Some(transmute($clistener as *const ())));
        } else {
            iup_sys::IupSetCallback(ih, cstr!($cb_name), Some(transmute($clistener as *const ())));
        }

    }}
}

/// Clears a closure as a callback to IUP.
///
/// Returns a Option<Box<_>> with the previosly set closure.
///
/// Note: `$ih` can be a `ptr::null_mut` to set a callback in the global enviroment.
#[macro_export]
macro_rules! clear_fbox_callback {
    ($ih:expr, $cb_name:expr, Callback<$($rargs:ty),*>, $cb_return:ty) => {{
        use $crate::iup_sys;
        use std::ptr;

        let ih = $ih;
        let capsule_box = iup_sys::IupGetAttribute(ih, fbox_c_str!($cb_name))
                                                as *mut Box<dyn $crate::callback::Callback<$($rargs),*, $cb_return>>;
        if capsule_box.is_null() {
            None
        } else {


            // TODO when Box::from_raw gets stable use it instead of transmute here.
            let inner_box: Box<Box<dyn $crate::callback::Callback<$($rargs),*, $cb_return>>> = Box::from_raw(capsule_box);

            iup_sys::IupSetAttribute(ih, fbox_c_str!($cb_name), ptr::null());

            if ih.is_null() {
                iup_sys::IupSetFunction(cstr!($cb_name), None);
            } else {
                iup_sys::IupSetCallback(ih, cstr!($cb_name), None);
            }

            Some(*inner_box)
            // inner_box itself gets freed now
        }
    }}
}

#[macro_export]
macro_rules! get_fbox_callback {
    ($ih:expr, $cb_name:expr, Callback<$($rargs:ty),*>, $cb_return:ty) => {{
        let fbox_ptr = unsafe {
            iup_sys::IupGetAttribute($ih, fbox_c_str!($cb_name))
                as *mut Box<dyn $crate::callback::Callback<$($rargs),*, $cb_return>>
        };
        assert!(fbox_ptr.is_null() == false);
        let fbox: &mut Box<_> = unsafe { &mut (*(fbox_ptr)) };
        fbox
    }}
}

/// Implements a callback binding between C IUP and Rust which accepts closures.
///
/// After this macro is executed the trait `$trait_name` is implemented with the following
/// default methods:
///
///   + `$set_method` to set associate a closure with the callback `$cb_name`.
///     The `F` (macro captured) constraint defines the type of high-level callback.
///   + `$remove_method` to remove a previosly associated callback `$cb_name`.
///   + `listener` is **not** defined. It is the native C callback signature (macro captured).
///   + `resolve_args` is optional should have a code body, and is also not defined.
///      It is responsible for translating the C arguments into Rust arguments. By default it just
///      calls the `IntoRust` trait for each argument.
///
/// **Note**: Don't forget to add a dropper for the event in `drop_callbacks` after using this
/// macro. You **must** do so to free allocations associated with closures.
///
#[macro_export]
macro_rules! impl_callback {

    // Used for element callbacks.
    // (no resolve_args version)
    (
        $(#[$trait_attr:meta])* // allow doc comments here
        pub trait $trait_name:ident where Self: Element {
            let name = $cb_name:expr;
            extern fn listener(ih: *mut iup_sys::Ihandle $(, $ls_arg:ident: $ls_arg_ty:ty)*) -> CallbackReturn;

            fn $set_method:ident<F: Callback(Self $(, $fn_arg_ty:ty)*)>(&mut self, cb: F) -> Self;
            fn $remove_method:ident(&mut self) -> Option<Box<_>>;
        }

    ) => {
        impl_callback! {
            $(#[$trait_attr])*
            pub trait $trait_name where Self: Element {
                let name = $cb_name;
                extern fn listener(ih: *mut iup_sys::Ihandle $(, $ls_arg: $ls_arg_ty)*) -> $crate::callback::CallbackReturn;

                fn $set_method<F: Callback(Self $(, $fn_arg_ty)*) -> c_int>(&mut self, cb: F) -> Self;
                fn $remove_method(&mut self) -> Option<Box<_>>;

                fn resolve_args(elem: Self, $($ls_arg: $ls_arg_ty),*) -> (Self, $($fn_arg_ty),*) {
                    (elem, $($ls_arg.into_rust()),*)
                }
            }
        }
    };



    // non-resolve raw ret
    (
        $(#[$trait_attr:meta])* // allow doc comments here
        pub trait $trait_name:ident where Self: Element {
            let name = $cb_name:expr;
            extern fn listener(ih: *mut iup_sys::Ihandle $(, $ls_arg:ident: $ls_arg_ty:ty)*) -> $cb_return:ty;

            fn $set_method:ident<F: Callback(Self $(, $fn_arg_ty:ty)*) -> $raw_ret:ty>(&mut self, cb: F) -> Self;
            fn $remove_method:ident(&mut self) -> Option<Box<_>>;
        }

    ) => {
        impl_callback! {
            $(#[$trait_attr])*
            pub trait $trait_name where Self: Element {
                let name = $cb_name;
                extern fn listener(ih: *mut iup_sys::Ihandle $(, $ls_arg: $ls_arg_ty)*) -> $cb_return;

                fn $set_method<F: Callback(Self $(, $fn_arg_ty)*) -> $raw_ret>(&mut self, cb: F) -> Self;
                fn $remove_method(&mut self) -> Option<Box<_>>;

                fn resolve_args(elem: Self, $($ls_arg: $ls_arg_ty),*) -> (Self, $($fn_arg_ty),*) {
                    (elem, $($ls_arg.into_rust()),*)
                }
            }
        }
    };

    // Used for element callbacks.
    // (resolve args version)
    (
        $(#[$trait_attr:meta])* // allow doc comments here
        pub trait $trait_name:ident where Self: Element {
            let name = $cb_name:expr;
            extern fn listener(ih: *mut iup_sys::Ihandle $(, $ls_arg:ident: $ls_arg_ty:ty)*) -> $cb_return:ty;

            fn $set_method:ident<F: Callback(Self $(, $fn_arg_ty:ty)*) -> $raw_ret:ty>(&mut self, cb: F) -> Self;
            fn $remove_method:ident(&mut self) -> Option<Box<_>>;

            fn resolve_args($aa_argself:ident: Self, $($aa_arg:ident: $aa_arg_ty:ty),*)
                            -> (Self, $($aa_ret_ty:ty),*)
            $resolve_args:expr
        }

    ) => {
        $(#[$trait_attr])*
        pub trait $trait_name where Self: $crate::Element + 'static {

            fn $set_method<F>(&self, cb: F) -> Self
                    where F: $crate::callback::Callback<(Self, $($fn_arg_ty),*), $cb_return> {

                use std::mem::transmute;
                #[allow(unused_imports)]
                use libc::c_int;
                use $crate::iup_sys;
                #[allow(unused_imports)]
                use $crate::callback::{CallbackReturn, IntoRust};

                fn resolve_args<Self0: $trait_name>($aa_argself: Self0, $($aa_arg: $aa_arg_ty),*)
                                                                       -> (Self0, $($aa_ret_ty),*) {
                    $resolve_args
                }

                extern "C" fn listener<Self0: $trait_name + 'static>(ih: *mut iup_sys::Ihandle, $($ls_arg: $ls_arg_ty),*) -> $raw_ret {
                    let fbox: &mut Box<_> = get_fbox_callback!(ih, $cb_name, Callback<(Self0, $($fn_arg_ty),*)>, $cb_return);
                    let element = unsafe { <Self0 as $crate::Element>::from_raw_unchecked(ih) };
                    fbox.on_callback(resolve_args::<Self0>(element, $($ls_arg),*)).into()
                }

                unsafe {
                    set_fbox_callback!(self.raw(), $cb_name, listener::<Self>, cb,
                                       Callback<(Self, $($fn_arg_ty),*)>, $cb_return);

                }

                self.clone()
            }

            fn $remove_method(&self)
                                -> Option<Box<dyn $crate::callback::Callback<(Self, $($fn_arg_ty),*), $cb_return>>> {
                unsafe {
                    let old_cb = clear_fbox_callback!(self.raw(), $cb_name,
                                                      Callback<(Self, $($fn_arg_ty),*)>, $cb_return);
                    old_cb
                }
            }
        }
    };

    // Used for global callbacks.
    (
            let name = $cb_name:expr;
            extern fn listener($($ls_arg:ident: $ls_arg_ty:ty),*) -> CallbackReturn;
            $(#[$set_func_attr:meta])*
            pub fn $set_func:ident<F: Callback($($fn_arg_ty:ty),*)>(cb: F);
            $(#[$rem_func_attr:meta])*
            pub fn $remove_func:ident() -> Option<Box<_>>;
    ) => {

            $(#[$set_func_attr])*
            pub fn $set_func<F>(cb: F)
                    where F: $crate::callback::Callback<($($fn_arg_ty),*), $crate::callback::CallbackReturn> {

                use std::mem::transmute;
                use std::ptr;
                #[allow(unused_imports)]
                use libc::c_int;
                use $crate::iup_sys;
                #[allow(unused_imports)]
                use $crate::callback::{CallbackReturn, IntoRust};

                extern "C" fn listener($($ls_arg: $ls_arg_ty),*) -> c_int {
                    let fbox: &mut Box<_> = get_fbox_callback!(ptr::null_mut(), $cb_name, Callback<($($fn_arg_ty),*)>, CallbackReturn);
                    fbox.on_callback(($($ls_arg.into_rust()),*)).into()
                }

                unsafe {
                    set_fbox_callback!(ptr::null_mut(), $cb_name, listener, cb,
                                       Callback<($($fn_arg_ty),*)>, CallbackReturn);
                }
            }

            $(#[$rem_func_attr])*
            pub fn $remove_func()
                    -> Option<Box<dyn $crate::callback::Callback<($($fn_arg_ty),*), $crate::callback::CallbackReturn >>> {
                unsafe {
                    //use std::ptr;
                    let old_cb = clear_fbox_callback!(ptr::null_mut(), $cb_name,
                                                      Callback<($($fn_arg_ty),*)>, $crate::callback::CallbackReturn);
                    old_cb
                }
            }
    };
}

/// Drops the closure associated with the `$cb_name` (literal) callback in the element `$ih`.
///
/// This is a **very hacky** method to free boxed closures, it takes advantage of the layout
/// of the dynamic dispatching of TraitObject to the destructor and also the fact our closures
/// are 'static (thus `Any`).
///
/// For this very reason this may not work on future versions of Rust since the language provides
/// no binary-compatibility guarantances between versions.
///
/// It was implemented this way to avoid [too much] extra work for freeing each closure, but as
/// soon as the library gets more mature it's recommended to find a replacement for this method.
#[macro_export]
macro_rules! drop_callback {
    ($ih:ident, $cb_name:expr) => {{
        use std::mem::transmute;
        use std::any::Any;
        let capsule_box = iup_sys::IupGetAttribute($ih, fbox_c_str!($cb_name))
                                                    as *mut Box<dyn Any>;   // HACK HACK HACK!!!!
        if !capsule_box.is_null() {
            // TODO when Box::from_raw gets stable use it instead of transmute here.
            let inner_box: Box<Box<dyn Any>> = transmute(capsule_box);
            drop(inner_box);
        }
    }}
}
