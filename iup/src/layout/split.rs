use iup_sys;


use Element;

pub struct Split(*mut iup_sys::Ihandle);

impl Split {
    pub fn new<T1: Element, T2: Element>(first: T1, second: T2) -> Self {
        unsafe { Split::from_raw(iup_sys::IupSplit(first.raw(), second.raw())) }
    }
}

impl_widget_container!(Split, "split");
