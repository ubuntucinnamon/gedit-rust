pub use gedit::*;
pub use gedit::gtk::prelude::*;
pub use glib::g_warning;

#[no_mangle]
pub fn peas_register_types() {
	gtk::init();
	g_warning!("", "HI");

	Document::new();
	Statusbar::new();
	ProgressInfoBar::new("no", "a", true).set_text("Hello");
}
