use iup_sys;

use Handle;
use Element;

pub struct ZBox(*mut iup_sys::Ihandle);

impl ZBox {
    /// Creates a vertical container box with the specified childs.
    pub fn new<A>(elems: A) -> ZBox where A: AsRef<[Handle]>  {
        let mut carray = slice_to_ih_array!(elems.as_ref());
        unsafe { ZBox::from_raw(iup_sys::IupZboxv(carray.as_mut_ptr())) }
    }
}

impl_widget_container!(ZBox, "zbox");
