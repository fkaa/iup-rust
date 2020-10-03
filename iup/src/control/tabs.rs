use iup_sys;

use Handle;
use Element;

pub struct Tabs(*mut iup_sys::Ihandle);

impl Tabs {
    /// Creates a vertical container box with the specified childs.
    pub fn new<A>(elems: A) -> Self where A: AsRef<[Handle]>  {
        let mut carray = slice_to_ih_array!(elems.as_ref());
        unsafe { Tabs::from_raw(iup_sys::IupTabsv(carray.as_mut_ptr())) }
    }
}

impl_widget_container!(Tabs, "tabs");
