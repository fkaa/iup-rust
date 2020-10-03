//! See [IUP Controls][1].
//! [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/controls.html

pub mod label;
pub mod text;
pub mod button;
pub mod progress;
pub mod toggle;
pub mod frame;
pub mod flat_frame;
pub mod list;
pub mod menu;
pub mod tabs;
pub mod tree;
pub mod canvas;
pub mod val;
pub mod flat_val;
pub mod flat_list;

pub use self::text::{Text, TextAction};
pub use self::label::Label;
pub use self::button::Button;
pub use self::progress::ProgressBar;
pub use self::toggle::{Toggle, ToggleAction};
pub use self::frame::Frame;
pub use self::flat_frame::FlatFrame;
pub use self::list::{List, ListAction};
pub use self::menu::{Menu, Submenu, Item, Separator};
pub use self::tabs::{Tabs};
pub use self::tree::{Tree, SelectionCb};
pub use self::canvas::{Canvas};
pub use self::val::{Val};
pub use self::flat_val::{FlatVal};
pub use self::flat_list::{FlatList};
