use gtk::prelude::*;

pub fn force_x11_backend() {
    unsafe {
        std::env::set_var("GDK_BACKEND", "x11");
    }
}

pub fn init_gtk() {
    gtk::init().expect("Failed to initialize GTK.");
}

pub fn build_desktop_window(title: &str) -> gtk::Window {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title(title);
    window.set_decorated(false);
    window.set_app_paintable(true);
    window.set_keep_below(true);
    window.set_skip_taskbar_hint(true);
    window.set_skip_pager_hint(true);
    window.set_accept_focus(false);
    window.set_type_hint(gtk::gdk::WindowTypeHint::Desktop);
    window
}

pub fn enable_transparency(window: &gtk::Window) {
    let screen = gtk::prelude::WidgetExt::screen(window).unwrap();
    let visual = screen.rgba_visual().unwrap();
    window.set_visual(Some(&visual));
}

pub fn run_main() {
    gtk::main();
}
