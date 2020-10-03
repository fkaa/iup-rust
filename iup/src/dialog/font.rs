use iup_sys;


use Element;

pub struct FontDlg(*mut iup_sys::Ihandle);

impl FontDlg {
    pub fn new() -> Self {
        unsafe { FontDlg::from_raw(iup_sys::IupFontDlg()) }
    }

    pub fn font(&self) -> Option<String> {
        self.attrib("VALUE")
    }
}

impl_dialog!(FontDlg, "fontdlg");
