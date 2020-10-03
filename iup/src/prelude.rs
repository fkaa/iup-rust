//! The IUP-Rust Prelude.
//!
//! This is a prelude module that reexports many of the most common traits, types and functions.
//!
//! The users can glob import this prelude module to get access to many of the most common traits,
//! types and functions.
//!
//! ```ignore
//! use iup::prelude::*;
//! ```
//!
//! The prelude is primarily concerned with exporting traits that are so pervasive that it would
//! be obnoxious to import each them every time.

// Common Types
pub use ::Orientation;
pub use dialog::{Dialog, DialogPos};

// Common Traits
pub use element::{Element, Widget, Container, Node, ConvertXYToPos};
pub use image::ImageElement;
pub use dialog::DialogElement;

// Callbacks
pub use callback::{CallbackReturn, Action, ActionCb, DestroyCb};
pub use callback::{KeyCb, MapCb, UnmapCb, GetFocusCb, KillFocusCb, EnterWindowCb, LeaveWindowCb, HelpCb};
pub use callback::{CaretCb, SpinCb, ValueChangedCb, DropFilesCb, DropDataCb};
pub use callback::{CloseCb, MoveCb, ResizeCb, PostMessageCb};
pub use callback::key::{KeyCode};
pub use callback::button::{ButtonCb, MotionCb};
pub use control::{SelectionCb, TextAction, ToggleAction, ListAction};
pub use dialog::{CopyDataCb, MdiActivateCb, ShowCb, TrayClickCb};
