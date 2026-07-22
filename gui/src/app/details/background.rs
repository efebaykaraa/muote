use gtk::prelude::{BoxExt, ColorChooserExt, EditableExt, RangeExt, WidgetExt};
use relm4::ComponentSender;

use crate::app::{AppInput, AppModel};

pub fn bg_details(
    model: &AppModel,
    sender: &ComponentSender<AppModel>,
) -> (
    gtk::Box,
    gtk::Entry,
    gtk::Switch,
    gtk::Switch,
    gtk::Scale,
    gtk::SpinButton,
) {
    let content = gtk::Box::new(gtk::Orientation::Vertical, 8);

    let bg_color_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    bg_color_box.append(&gtk::Label::new(Some("BG Color:")));
    let bg_entry = gtk::Entry::new();
    bg_entry.set_text(&model.settings.appearance.bg_color);
    bg_entry.set_hexpand(true);
    let s_clone = sender.clone();
    bg_entry.connect_changed(move |e| s_clone.input(AppInput::UpdateBgColor(e.text().to_string())));
    bg_color_box.append(&bg_entry);

    let bg_color_btn = gtk::ColorButton::new();
    let (r, g, b, a) = muote_core::config::parse_color_to_rgba(&model.settings.appearance.bg_color);
    bg_color_btn.set_rgba(&gtk::gdk::RGBA::new(r as f32, g as f32, b as f32, a as f32));
    let s_clone = sender.clone();
    bg_color_btn.connect_color_set(move |btn| {
        let rgba = btn.rgba();
        let hex = muote_core::config::rgba_to_hex(
            rgba.red() as f64,
            rgba.green() as f64,
            rgba.blue() as f64,
            rgba.alpha() as f64,
        );
        s_clone.input(AppInput::UpdateBgColor(hex));
    });
    bg_color_box.append(&bg_color_btn);
    content.append(&bg_color_box);

    let bg_rounded_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    bg_rounded_box.append(&gtk::Label::new(Some("Rounded:")));
    let bg_rounded_switch = gtk::Switch::new();
    bg_rounded_switch.set_active(model.settings.appearance.bg_rounded);
    let s_clone = sender.clone();
    bg_rounded_switch.connect_state_set(move |_, state| {
        s_clone.input(AppInput::UpdateBgRounded(state));
        gtk::glib::Propagation::Proceed
    });
    bg_rounded_box.append(&bg_rounded_switch);
    content.append(&bg_rounded_box);

    let bg_fill_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    bg_fill_box.append(&gtk::Label::new(Some("Fill:")));
    let bg_fill_switch = gtk::Switch::new();
    bg_fill_switch.set_active(model.settings.appearance.bg_fill);
    let s_clone = sender.clone();
    bg_fill_switch.connect_state_set(move |_, state| {
        s_clone.input(AppInput::UpdateBgFill(state));
        gtk::glib::Propagation::Proceed
    });
    bg_fill_box.append(&bg_fill_switch);
    content.append(&bg_fill_box);

    let opacity_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    opacity_box.append(&gtk::Label::new(Some("Opacity:")));
    let bg_scale = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 1.0, 0.01);
    bg_scale.set_hexpand(true);
    let (_, _, _, ba) =
        muote_core::config::parse_color_to_rgba(&model.settings.appearance.bg_color);
    bg_scale.set_value(ba);
    let s_clone = sender.clone();
    bg_scale
        .connect_value_changed(move |s| s_clone.input(AppInput::UpdateBgOpacity(s.value() as f32)));
    opacity_box.append(&bg_scale);

    let bg_spin = gtk::SpinButton::with_range(0.0, 1.0, 0.05);
    bg_spin.set_value(ba);
    let s_clone = sender.clone();
    bg_spin
        .connect_value_changed(move |s| s_clone.input(AppInput::UpdateBgOpacity(s.value() as f32)));
    opacity_box.append(&bg_spin);
    content.append(&opacity_box);

    (
        content,
        bg_entry,
        bg_rounded_switch,
        bg_fill_switch,
        bg_scale,
        bg_spin,
    )
}
