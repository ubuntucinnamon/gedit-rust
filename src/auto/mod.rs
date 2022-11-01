// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

mod app;
pub use self::app::App;

mod app_activatable;
pub use self::app_activatable::AppActivatable;

mod document;
pub use self::document::Document;

mod menu_extension;
pub use self::menu_extension::MenuExtension;

mod message;
pub use self::message::Message;

mod message_bus;
pub use self::message_bus::MessageBus;

mod progress_info_bar;
pub use self::progress_info_bar::ProgressInfoBar;

mod statusbar;
pub use self::statusbar::Statusbar;

mod tab;
pub use self::tab::Tab;

mod view;
pub use self::view::View;

mod view_activatable;
pub use self::view_activatable::ViewActivatable;

mod window;
pub use self::window::Window;

mod window_activatable;
pub use self::window_activatable::WindowActivatable;

mod enums;
pub use self::enums::TabState;

mod flags;
pub use self::flags::WindowState;

#[doc(hidden)]
pub mod traits {
    pub use super::app::AppExt;
    pub use super::app_activatable::AppActivatableExt;
    pub use super::document::DocumentExt;
    pub use super::message::MessageExt;
    pub use super::message_bus::MessageBusExt;
    pub use super::view::ViewExt;
    pub use super::view_activatable::ViewActivatableExt;
    pub use super::window::WindowExt;
    pub use super::window_activatable::WindowActivatableExt;
}
#[doc(hidden)]
pub mod builders {
    pub use super::app::AppBuilder;
    pub use super::document::DocumentBuilder;
    pub use super::menu_extension::MenuExtensionBuilder;
    pub use super::message::MessageBuilder;
    pub use super::progress_info_bar::ProgressInfoBarBuilder;
    pub use super::statusbar::StatusbarBuilder;
    pub use super::tab::TabBuilder;
    pub use super::view::ViewBuilder;
    pub use super::window::WindowBuilder;
}