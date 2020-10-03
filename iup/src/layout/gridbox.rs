use iup_sys;

use Handle;
use Element;

pub struct GridBox(*mut iup_sys::Ihandle);

impl GridBox {
    /// Creates a vertical container box with the specified childs.
    pub fn new<A>(elems: A) -> GridBox where A: AsRef<[Handle]>  {
        let mut carray = slice_to_ih_array!(elems.as_ref());
        unsafe { GridBox::from_raw(iup_sys::IupGridBoxv(carray.as_mut_ptr())) }
    }
}

impl_widget_container!(GridBox, "gridbox");
