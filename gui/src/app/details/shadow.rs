use gtk::prelude::{BoxExt, ColorChooserExt, EditableExt, RangeExt, WidgetExt};
use relm4::ComponentSender;

use crate::app::{AppInput, AppModel, widget::ShadowWidgets};

pub fn shadow_component(
    model: &AppModel,
    sender: &ComponentSender<AppModel>,
) -> (gtk::Box, ShadowWidgets) {
    let content = gtk::Box::new(gtk::Orientation::Vertical, 8);

    let shadow_title_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    shadow_title_box.append(
        &gtk::Label::builder()
            .label("Text Shadow")
            .hexpand(true)
            .halign(gtk::Align::Start)
            .css_classes(vec!["title-4".to_string()])
            .build(),
    );
    let shadow_switch = gtk::Switch::new();
    shadow_switch.set_active(model.settings.appearance.shadow_enabled);
    let s_clone = sender.clone();
    shadow_switch.connect_state_set(move |_, state| {
        s_clone.input(AppInput::UpdateShadowEnabled(state));
        gtk::glib::Propagation::Proceed
    });
    shadow_title_box.append(&shadow_switch);
    content.append(&shadow_title_box);

    let (
        shadow_details,
        shadow_entry,
        shadow_scale,
        shadow_spin,
        shadow_opacity_scale,
        shadow_opacity_spin,
        shadow_blur_scale,
        shadow_blur_spin,
        shadow_size_scale,
        shadow_size_spin,
    ) = shadow_details(model, sender);
    let shadow_revealer = gtk::Revealer::new();
    shadow_revealer.set_child(Some(&shadow_details));
    shadow_revealer.set_reveal_child(model.settings.appearance.shadow_enabled);
    content.append(&shadow_revealer);

    (
        content,
        ShadowWidgets {
            shadow_entry,
            shadow_revealer,
            shadow_scale,
            shadow_spin,
            shadow_opacity_scale,
            shadow_opacity_spin,
            shadow_blur_scale,
            shadow_blur_spin,
            shadow_size_scale,
            shadow_size_spin,
        },
    )
}

pub fn shadow_details(
    model: &AppModel,
    sender: &ComponentSender<AppModel>,
) -> (
    gtk::Box,
    gtk::Entry,
    gtk::Scale,
    gtk::SpinButton,
    gtk::Scale,
    gtk::SpinButton,
    gtk::Scale,
    gtk::SpinButton,
    gtk::Scale,
    gtk::SpinButton,
) {
    let content = gtk::Box::new(gtk::Orientation::Vertical, 8);

    let shadow_color_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    shadow_color_box.append(
        &gtk::Label::builder()
            .label("Shadow color:")
            .hexpand(true)
            .halign(gtk::Align::Start)
            .build(),
    );
    let shadow_entry = gtk::Entry::new();
    shadow_entry.set_text(&model.settings.appearance.shadow_color);
    shadow_entry.set_width_chars(10);
    let s_clone = sender.clone();
    shadow_entry
        .connect_changed(move |e| s_clone.input(AppInput::UpdateShadowColor(e.text().to_string())));
    shadow_color_box.append(&shadow_entry);
    let shadow_color_btn = gtk::ColorButton::new();
    let (r, g, b, a) =
        muote_core::config::parse_color_to_rgba(&model.settings.appearance.shadow_color);
    shadow_color_btn.set_rgba(&gtk::gdk::RGBA::new(r as f32, g as f32, b as f32, a as f32));
    let s_clone = sender.clone();
    shadow_color_btn.connect_color_set(move |btn| {
        let rgba = btn.rgba();
        let hex = muote_core::config::rgba_to_hex(
            rgba.red() as f64,
            rgba.green() as f64,
            rgba.blue() as f64,
            rgba.alpha() as f64,
        );
        s_clone.input(AppInput::UpdateShadowColor(hex));
    });
    shadow_color_box.append(&shadow_color_btn);
    content.append(&shadow_color_box);

    let offset_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    offset_box.append(
        &gtk::Label::builder()
            .label("Shadow offset:")
            .hexpand(true)
            .halign(gtk::Align::Start)
            .build(),
    );
    let shadow_scale = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 8.0, 0.25);
    shadow_scale.set_hexpand(true);
    shadow_scale.set_value(model.settings.appearance.shadow_offset as f64);
    let s_clone = sender.clone();
    shadow_scale.connect_value_changed(move |s| {
        s_clone.input(AppInput::UpdateShadowOffset(s.value() as f32))
    });
    offset_box.append(&shadow_scale);

    let shadow_spin = gtk::SpinButton::with_range(0.0, 8.0, 0.25);
    shadow_spin.set_value(model.settings.appearance.shadow_offset as f64);
    let s_clone = sender.clone();
    shadow_spin.connect_value_changed(move |s| {
        s_clone.input(AppInput::UpdateShadowOffset(s.value() as f32))
    });
    offset_box.append(&shadow_spin);
    content.append(&offset_box);

    let opacity_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    opacity_box.append(
        &gtk::Label::builder()
            .label("Shadow opacity:")
            .hexpand(true)
            .halign(gtk::Align::Start)
            .build(),
    );
    let shadow_opacity = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 1.0, 0.01);
    shadow_opacity.set_hexpand(true);
    let (_, _, _, alpha) =
        muote_core::config::parse_color_to_rgba(&model.settings.appearance.shadow_color);
    shadow_opacity.set_value(alpha);
    let s_clone = sender.clone();
    shadow_opacity.connect_value_changed(move |s| {
        s_clone.input(AppInput::UpdateShadowOpacity(s.value() as f32))
    });
    opacity_box.append(&shadow_opacity);
    let shadow_opacity_spin = gtk::SpinButton::with_range(0.0, 1.0, 0.05);
    shadow_opacity_spin.set_value(alpha);
    let s_clone = sender.clone();
    shadow_opacity_spin.connect_value_changed(move |s| {
        s_clone.input(AppInput::UpdateShadowOpacity(s.value() as f32))
    });
    opacity_box.append(&shadow_opacity_spin);
    content.append(&opacity_box);

    let blur_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    blur_box.append(
        &gtk::Label::builder()
            .label("Shadow blur:")
            .hexpand(true)
            .halign(gtk::Align::Start)
            .build(),
    );
    let shadow_blur = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 12.0, 0.5);
    shadow_blur.set_hexpand(true);
    shadow_blur.set_value(model.settings.appearance.shadow_blur as f64);
    let s_clone = sender.clone();
    shadow_blur.connect_value_changed(move |s| {
        s_clone.input(AppInput::UpdateShadowBlur(s.value() as f32))
    });
    blur_box.append(&shadow_blur);
    let shadow_blur_spin = gtk::SpinButton::with_range(0.0, 12.0, 0.5);
    shadow_blur_spin.set_value(model.settings.appearance.shadow_blur as f64);
    let s_clone = sender.clone();
    shadow_blur_spin.connect_value_changed(move |s| {
        s_clone.input(AppInput::UpdateShadowBlur(s.value() as f32))
    });
    blur_box.append(&shadow_blur_spin);
    content.append(&blur_box);

    let size_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    size_box.append(
        &gtk::Label::builder()
            .label("Shadow size:")
            .hexpand(true)
            .halign(gtk::Align::Start)
            .build(),
    );
    let shadow_size = gtk::Scale::with_range(gtk::Orientation::Horizontal, 1.0, 2.0, 0.05);
    shadow_size.set_hexpand(true);
    shadow_size.set_value(model.settings.appearance.shadow_size as f64);
    let s_clone = sender.clone();
    shadow_size.connect_value_changed(move |s| {
        s_clone.input(AppInput::UpdateShadowSize(s.value() as f32))
    });
    size_box.append(&shadow_size);
    let shadow_size_spin = gtk::SpinButton::with_range(1.0, 2.0, 0.05);
    shadow_size_spin.set_value(model.settings.appearance.shadow_size as f64);
    let s_clone = sender.clone();
    shadow_size_spin.connect_value_changed(move |s| {
        s_clone.input(AppInput::UpdateShadowSize(s.value() as f32))
    });
    size_box.append(&shadow_size_spin);
    content.append(&size_box);

    (
        content,
        shadow_entry,
        shadow_scale,
        shadow_spin,
        shadow_opacity,
        shadow_opacity_spin,
        shadow_blur,
        shadow_blur_spin,
        shadow_size,
        shadow_size_spin,
    )
}
