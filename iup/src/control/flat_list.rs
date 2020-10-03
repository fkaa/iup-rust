use iup_sys;
use libc::{c_char, c_int};
use std::ptr;

use Element;
use callback::IntoRust;

/// See the [IUP List Documentation][1].
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/elem/iuplist.html
pub struct FlatList(*mut iup_sys::Ihandle);

impl FlatList {
    /// Creates an interface element that displays a list of items.
    pub fn new() -> FlatList {
        unsafe { FlatList::from_raw(iup_sys::IupFlatList(ptr::null())) }
    }

    /// Creates an interface element that displays a list of items in a dropdown.
    pub fn new_dropdown() -> FlatList {
        FlatList::new().set_attrib_data("DROPDOWN", cstr!("YES") as *const _)
    }

    /// Creates an interface element that displays a list of items with a edit box for text input.
    pub fn new_editbox() -> FlatList {
        FlatList::new().set_attrib_data("EDITBOX", cstr!("YES") as *const _)
    }

    // TODO how to call the fusion of dropbox and editbox?

    /// Sets the list of items.
    pub fn set_items<A>(&mut self, items: A) -> Self where A: AsRef<[String]> {
        self.clear();
        for (i, value) in items.as_ref().iter().enumerate() {
            self.set_attrib((i+1).to_string(), value.clone());
        }
        *self
    }

    /// Gets the item at the specified id (starts from 1).
    ///
    /// # Panics
    /// Panics if id is less than 1.
    pub fn item<A>(&self, id: u32) -> Option<String> {
        assert!(id > 0);
        self.attrib(id.to_string())
    }

    /// Clears the list of items. Ignored if called before being mapped.
    pub fn clear(&mut self) -> Self {
        self.set_attrib("REMOVEITEM", "ALL")
    }
}

impl_widget!(FlatList, "flatlist");

/// Returns a list item position from it's xy coordinate.
impl ::element::ConvertXYToPos for FlatList {}

impl ::callback::MapCb for FlatList {}
impl ::callback::UnmapCb for FlatList {}
impl ::callback::GetFocusCb for FlatList {}
impl ::callback::KillFocusCb for FlatList {}
impl ::callback::EnterWindowCb for FlatList {}
impl ::callback::LeaveWindowCb for FlatList {}
impl ::callback::HelpCb for FlatList {}
// TODO impl K_ callbacks when it's implemented.

// TODO impl future DragSource and DragTarget traits.

/// Action generated when any mouse button is pressed or released inside the list.
///
/// Called only when DROPDOWN=NO. If the list has an editbox the message is called when cursor
/// is at the listbox only (ignored at the editbox).
///
/// Use `convert_xy_to_pos` to convert (x,y) coordinates in item position.
impl ::callback::button::ButtonCb for FlatList {}

/// Action generated when the caret/cursor position is changed. Valid only when EDITBOX=YES.
///
/// For lists `lin` (2nd param) is always *1*, and pos (3rd param) is always *col-1*.
impl ::callback::CaretCb for FlatList {}

/// Action generated when one or more files are dropped in the element.
impl ::callback::DropFilesCb for FlatList {}

///  Action generated when the mouse is moved over the list. Called only when DROPDOWN=NO.
///
/// If the list has an editbox the message is called when cursor is at the listbox only (ignored
/// at the editbox).
///
/// Use `convert_xy_to_pos` to convert (x,y) coordinates in item position.
impl ::callback::button::MotionCb for FlatList {}

/// Called after the value was interactively changed by the user. Called when the selection is
/// changed or when the text is edited.
impl ::callback::ValueChangedCb for FlatList {}

// TODO:
// DBLCLICK_CB
// MULTISELECT_CB
// EDIT_CB
// DROPDOWN_CB
// DRAGDROP_CB

/// See the `FlatListAction` documentation.
impl super::list::ListAction for FlatList {}
