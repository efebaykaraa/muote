pub mod details;
pub mod input;
pub mod model;
pub mod static_components;
pub mod widget;

use adw::prelude::*;
pub use details::{background::bg_details, shadow::shadow_component};
use engyls::config::{HorizontalAlign, VerticalAlign};
use gtk::Revealer;
pub use input::AppInput;
pub use model::AppModel;
use relm4::{Component, ComponentParts, ComponentSender, RelmWidgetExt, adw, gtk};
pub use static_components::separator;

use crate::app::widget::AppWidgets;

const MIN_WINDOW_WIDTH: i32 = 560;

fn language_options() -> [(&'static str, &'static str); 11] {
    [
        ("ORIGINAL", "Original"),
        ("DE", "German"),
        ("FR", "French"),
        ("ES", "Spanish"),
        ("IT", "Italian"),
        ("PT", "Portuguese"),
        ("NL", "Dutch"),
        ("PL", "Polish"),
        ("RU", "Russian"),
        ("TR", "Turkish"),
        ("UK", "Ukrainian"),
    ]
}

fn restart_desktop_service() {
    match std::process::Command::new("systemctl")
        .args(["--user", "restart", "desktop-quote.service"])
        .status()
    {
        Ok(status) if status.success() => log::info!("Restarted desktop-quote.service"),
        Ok(status) => log::warn!("desktop-quote.service restart exited with {}", status),
        Err(err) => log::warn!("Failed to restart desktop-quote.service: {}", err),
    }
}

fn child_count(container: &gtk::Box) -> usize {
    let mut count = 0;
    let mut child = container.first_child();
    while let Some(widget) = child {
        count += 1;
        child = widget.next_sibling();
    }
    count
}

fn align_button(icon_name: &str, tooltip: &str, active: bool) -> gtk::ToggleButton {
    let button = gtk::ToggleButton::new();
    button.set_child(Some(&gtk::Image::from_icon_name(icon_name)));
    button.set_tooltip_text(Some(tooltip));
    button.set_active(active);
    button
}

fn horizontal_align_buttons(
    current: HorizontalAlign,
    on_change: impl Fn(HorizontalAlign) + Clone + 'static,
) -> gtk::Box {
    let group_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    group_box.add_css_class("linked");

    let left = align_button(
        "format-justify-left-symbolic",
        "Left",
        current == HorizontalAlign::Left,
    );
    let center = align_button(
        "format-justify-center-symbolic",
        "Center",
        current == HorizontalAlign::Center,
    );
    center.set_group(Some(&left));
    let right = align_button(
        "format-justify-right-symbolic",
        "Right",
        current == HorizontalAlign::Right,
    );
    right.set_group(Some(&left));

    let emit = on_change.clone();
    left.connect_toggled(move |button| {
        if button.is_active() {
            emit(HorizontalAlign::Left);
        }
    });
    let emit = on_change.clone();
    center.connect_toggled(move |button| {
        if button.is_active() {
            emit(HorizontalAlign::Center);
        }
    });
    right.connect_toggled(move |button| {
        if button.is_active() {
            on_change(HorizontalAlign::Right);
        }
    });

    group_box.append(&left);
    group_box.append(&center);
    group_box.append(&right);
    group_box
}

fn vertical_align_buttons(
    current: VerticalAlign,
    on_change: impl Fn(VerticalAlign) + Clone + 'static,
) -> gtk::Box {
    let group_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    group_box.add_css_class("linked");

    let top = align_button("go-up-symbolic", "Up", current == VerticalAlign::Top);
    let center = align_button(
        "format-justify-center-symbolic",
        "Center",
        current == VerticalAlign::Center,
    );
    center.set_group(Some(&top));
    let bottom = align_button("go-down-symbolic", "Down", current == VerticalAlign::Bottom);
    bottom.set_group(Some(&top));

    let emit = on_change.clone();
    top.connect_toggled(move |button| {
        if button.is_active() {
            emit(VerticalAlign::Top);
        }
    });
    let emit = on_change.clone();
    center.connect_toggled(move |button| {
        if button.is_active() {
            emit(VerticalAlign::Center);
        }
    });
    bottom.connect_toggled(move |button| {
        if button.is_active() {
            on_change(VerticalAlign::Bottom);
        }
    });

    group_box.append(&top);
    group_box.append(&center);
    group_box.append(&bottom);
    group_box
}

fn alignment_section(model: &AppModel, sender: &ComponentSender<AppModel>) -> gtk::Box {
    let section = gtk::Box::new(gtk::Orientation::Vertical, 8);
    section.append(
        &gtk::Label::builder()
            .label("Alignment")
            .halign(gtk::Align::Start)
            .css_classes(vec!["title-4".to_string()])
            .build(),
    );

    let phrase_row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    phrase_row.append(
        &gtk::Label::builder()
            .label("Phrase")
            .hexpand(true)
            .halign(gtk::Align::Start)
            .build(),
    );

    let s_clone = sender.clone();
    phrase_row.append(&horizontal_align_buttons(
        model.settings.appearance.quote_h_align,
        move |align| s_clone.input(AppInput::UpdateQuoteHAlign(align)),
    ));

    let s_clone = sender.clone();
    phrase_row.append(&vertical_align_buttons(
        model.settings.appearance.quote_v_align,
        move |align| s_clone.input(AppInput::UpdateQuoteVAlign(align)),
    ));
    section.append(&phrase_row);

    let quoter_row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    quoter_row.append(
        &gtk::Label::builder()
            .label("Quoter")
            .hexpand(true)
            .halign(gtk::Align::Start)
            .build(),
    );

    let s_clone = sender.clone();
    quoter_row.append(&horizontal_align_buttons(
        model.settings.appearance.author_h_align,
        move |align| s_clone.input(AppInput::UpdateAuthorHAlign(align)),
    ));
    section.append(&quoter_row);

    section
}

impl Component for AppModel {
    type Init = ();
    type Input = AppInput;
    type Output = ();
    type CommandOutput = ();
    type Root = adw::Window;
    type Widgets = widget::AppWidgets;
    fn init_root() -> Self::Root {
        let root = adw::Window::builder()
            .default_width(630)
            .default_height(850)
            .title("Marxist Quote Display - Settings")
            .build();
        root.set_size_request(MIN_WINDOW_WIDTH, -1);
        root
    }

    fn init(_params: (), root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = AppModel::init_model();

        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        root.set_content(Some(&main_box));

        let header = adw::HeaderBar::new();
        let title = adw::WindowTitle::new("", "");
        header.set_title_widget(Some(&title));

        let save_btn = gtk::Button::with_label("Save & Apply");
        save_btn.add_css_class("suggested-action");
        let s_clone = sender.clone();
        save_btn.connect_clicked(move |_| s_clone.input(AppInput::Save));
        header.pack_start(&save_btn);

        let picker_btn = gtk::Button::with_label("Text Position");
        let s_clone = sender.clone();
        picker_btn.connect_clicked(move |_| s_clone.input(AppInput::ShowInteractivePicker));
        header.pack_end(&picker_btn);

        main_box.append(&header);

        let scroll = gtk::ScrolledWindow::new();
        scroll.set_vexpand(true);
        scroll.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        main_box.append(&scroll);

        let content = gtk::Box::new(gtk::Orientation::Vertical, 20);
        content.set_margin_all(20);
        content.set_hexpand(true);
        scroll.set_child(Some(&content));

        let font_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        font_box.append(&gtk::Label::new(Some("Font:")));
        let combo = gtk::ComboBoxText::new();
        for f in [
            "Inter",
            "Roboto",
            "Ubuntu",
            "Cantarell",
            "sans-serif",
            "serif",
            "monospace",
        ] {
            combo.append(Some(f), f);
        }
        combo.set_active_id(Some(&model.settings.appearance.font));
        let s_clone = sender.clone();
        combo.connect_changed(move |c| {
            if let Some(id) = c.active_id() {
                s_clone.input(AppInput::UpdateFont(id.to_string()));
            }
        });
        font_box.append(&combo);
        content.append(&font_box);

        let size_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        size_box.append(&gtk::Label::new(Some("Size:")));
        let size_spin = gtk::SpinButton::with_range(8.0, 120.0, 1.0);
        size_spin.set_value(model.settings.appearance.font_size as f64);
        let s_clone = sender.clone();
        size_spin.connect_value_changed(move |s| {
            s_clone.input(AppInput::UpdateFontSize(s.value() as f32))
        });
        size_box.append(&size_spin);
        content.append(&size_box);

        let color_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        color_box.append(&gtk::Label::new(Some("Color:")));
        let quote_entry = gtk::Entry::new();
        quote_entry.set_text(&model.settings.appearance.text_color);
        quote_entry.set_hexpand(true);
        let s_clone = sender.clone();
        quote_entry.connect_changed(move |e| {
            s_clone.input(AppInput::UpdateTextColor(e.text().to_string()))
        });
        color_box.append(&quote_entry);

        let quote_color = gtk::ColorButton::new();
        let (r, g, b, a) =
            engyls::config::parse_color_to_rgba(&model.settings.appearance.text_color);
        quote_color.set_rgba(&gtk::gdk::RGBA::new(r as f32, g as f32, b as f32, a as f32));
        let s_clone = sender.clone();
        quote_color.connect_color_set(move |btn| {
            let rgba = btn.rgba();
            let hex = engyls::config::rgba_to_hex(
                rgba.red() as f64,
                rgba.green() as f64,
                rgba.blue() as f64,
                rgba.alpha() as f64,
            );
            s_clone.input(AppInput::UpdateTextColor(hex));
        });
        color_box.append(&quote_color);
        content.append(&color_box);

        let language_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        language_box.append(&gtk::Label::new(Some("Language:")));
        let language_combo = gtk::ComboBoxText::new();
        for (id, label) in language_options() {
            language_combo.append(Some(id), label);
        }
        language_combo.set_active_id(Some(&model.settings.appearance.language));
        let s_clone = sender.clone();
        language_combo.connect_changed(move |combo| {
            if let Some(id) = combo.active_id() {
                s_clone.input(AppInput::UpdateLanguage(id.to_string()));
            }
        });
        language_box.append(&language_combo);
        content.append(&language_box);

        content.append(&separator());

        content.append(&alignment_section(&model, &sender));

        content.append(&separator());

        // --- Background ---
        let bg_title_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        bg_title_box.append(
            &gtk::Label::builder()
                .label("Background")
                .hexpand(true)
                .halign(gtk::Align::Start)
                .css_classes(vec!["title-4".to_string()])
                .build(),
        );

        let bg_switch = gtk::Switch::new();
        bg_switch.set_active(model.settings.appearance.bg_enabled);
        bg_switch.set_halign(gtk::Align::End);
        let s_clone = sender.clone();
        bg_switch.connect_state_set(move |_, state| {
            s_clone.input(AppInput::UpdateBgEnabled(state));
            gtk::glib::Propagation::Proceed
        });
        bg_title_box.append(&bg_switch);
        content.append(&bg_title_box);

        let (bg_details, bg_entry, bg_rounded_switch, bg_fill_switch, bg_scale, bg_spin) =
            bg_details(&model, &sender);

        let bg_revealer = Revealer::new();
        bg_revealer.set_child(Some(&bg_details));
        bg_revealer.set_reveal_child(model.settings.appearance.bg_enabled);
        content.append(&bg_revealer);

        content.append(&separator());

        // --- Stroke ---
        let stroke_title_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        stroke_title_box.append(
            &gtk::Label::builder()
                .label("Text Stroke")
                .hexpand(true)
                .halign(gtk::Align::Start)
                .css_classes(vec!["title-4".to_string()])
                .build(),
        );
        let stroke_switch = gtk::Switch::new();
        stroke_switch.set_active(model.settings.appearance.stroke_enabled);
        let s_clone = sender.clone();
        stroke_switch.connect_state_set(move |_, state| {
            s_clone.input(AppInput::UpdateStrokeEnabled(state));
            gtk::glib::Propagation::Proceed
        });
        stroke_title_box.append(&stroke_switch);
        content.append(&stroke_title_box);

        let stroke_box = gtk::Box::new(gtk::Orientation::Vertical, 8);

        let stroke_color_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        stroke_color_box.append(
            &gtk::Label::builder()
                .label("Stroke color:")
                .hexpand(true)
                .halign(gtk::Align::Start)
                .build(),
        );
        let stroke_entry = gtk::Entry::new();
        stroke_entry.set_text(&model.settings.appearance.stroke_color);
        stroke_entry.set_width_chars(10);
        let s_clone = sender.clone();
        stroke_entry.connect_changed(move |e| {
            s_clone.input(AppInput::UpdateStrokeColor(e.text().to_string()))
        });
        stroke_color_box.append(&stroke_entry);
        let stroke_color = gtk::ColorButton::new();
        let (r, g, b, a) =
            engyls::config::parse_color_to_rgba(&model.settings.appearance.stroke_color);
        stroke_color.set_rgba(&gtk::gdk::RGBA::new(r as f32, g as f32, b as f32, a as f32));
        let s_clone = sender.clone();
        stroke_color.connect_color_set(move |btn| {
            let rgba = btn.rgba();
            let hex = engyls::config::rgba_to_hex(
                rgba.red() as f64,
                rgba.green() as f64,
                rgba.blue() as f64,
                rgba.alpha() as f64,
            );
            s_clone.input(AppInput::UpdateStrokeColor(hex));
        });
        stroke_color_box.append(&stroke_color);
        stroke_box.append(&stroke_color_box);

        let stroke_width_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        stroke_width_box.append(
            &gtk::Label::builder()
                .label("Stroke size:")
                .hexpand(true)
                .halign(gtk::Align::Start)
                .build(),
        );
        let stroke_width_scale =
            gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.5, 10.0, 0.5);
        stroke_width_scale.set_hexpand(true);
        stroke_width_scale.set_value(model.settings.appearance.stroke_width as f64);
        let s_clone = sender.clone();
        stroke_width_scale.connect_value_changed(move |s| {
            s_clone.input(AppInput::UpdateStrokeWidth(s.value() as f32))
        });
        stroke_width_box.append(&stroke_width_scale);
        let stroke_width_spin = gtk::SpinButton::with_range(0.5, 10.0, 0.5);
        stroke_width_spin.set_value(model.settings.appearance.stroke_width as f64);
        let s_clone = sender.clone();
        stroke_width_spin.connect_value_changed(move |s| {
            s_clone.input(AppInput::UpdateStrokeWidth(s.value() as f32))
        });
        stroke_width_box.append(&stroke_width_spin);
        stroke_box.append(&stroke_width_box);

        let stroke_opacity_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        stroke_opacity_box.append(
            &gtk::Label::builder()
                .label("Stroke opacity:")
                .hexpand(true)
                .halign(gtk::Align::Start)
                .build(),
        );
        let (_, _, _, stroke_alpha) =
            engyls::config::parse_color_to_rgba(&model.settings.appearance.stroke_color);
        let stroke_opacity_scale =
            gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 1.0, 0.01);
        stroke_opacity_scale.set_hexpand(true);
        stroke_opacity_scale.set_value(stroke_alpha);
        let s_clone = sender.clone();
        stroke_opacity_scale.connect_value_changed(move |s| {
            s_clone.input(AppInput::UpdateStrokeOpacity(s.value() as f32))
        });
        stroke_opacity_box.append(&stroke_opacity_scale);
        let stroke_opacity_spin = gtk::SpinButton::with_range(0.0, 1.0, 0.05);
        stroke_opacity_spin.set_value(stroke_alpha);
        let s_clone = sender.clone();
        stroke_opacity_spin.connect_value_changed(move |s| {
            s_clone.input(AppInput::UpdateStrokeOpacity(s.value() as f32))
        });
        stroke_opacity_box.append(&stroke_opacity_spin);
        stroke_box.append(&stroke_opacity_box);

        let stroke_revealer = Revealer::new();
        stroke_revealer.set_child(Some(&stroke_box));
        stroke_revealer.set_reveal_child(model.settings.appearance.stroke_enabled);
        content.append(&stroke_revealer);

        content.append(&separator());

        // --- Shadow ---
        let (shadow_section, shadow_widgets) = shadow_component(&model, &sender);
        content.append(&shadow_section);

        content.append(&separator());

        // --- Authors ---
        let author_header = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        author_header.append(
            &gtk::Label::builder()
                .label("Authors")
                .hexpand(true)
                .halign(gtk::Align::Start)
                .css_classes(vec!["title-4".to_string()])
                .build(),
        );
        let fetch_btn = gtk::Button::with_label("Fetch New Quote Now");
        let s_clone = sender.clone();
        fetch_btn.connect_clicked(move |_| s_clone.input(AppInput::FetchQuoteNow));
        author_header.append(&fetch_btn);

        let skip_btn = gtk::Button::with_label("Skip Quote");
        let s_clone = sender.clone();
        skip_btn.connect_clicked(move |_| s_clone.input(AppInput::SkipQuote));
        author_header.append(&skip_btn);

        let add_author_btn = gtk::Button::with_label("Add Author");
        let s_clone = sender.clone();
        add_author_btn.connect_clicked(move |_| s_clone.input(AppInput::AddAuthor));
        author_header.append(&add_author_btn);
        content.append(&author_header);

        let author_note_revealer = gtk::Revealer::new();
        let author_note = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        author_note.add_css_class("card");
        author_note.set_margin_top(2);
        author_note.set_margin_bottom(2);
        author_note.set_margin_start(0);
        author_note.set_margin_end(0);
        author_note.append(
            &gtk::Label::builder()
                .label("The numbers are relative weights: higher numbers make an author appear more often when a quote is selected.")
                .wrap(true)
                .hexpand(true)
                .halign(gtk::Align::Start)
                .build(),
        );
        let close_note = gtk::Button::from_icon_name("window-close-symbolic");
        close_note.set_tooltip_text(Some("Hide this note"));
        let s_clone = sender.clone();
        close_note.connect_clicked(move |_| s_clone.input(AppInput::DismissAuthorNote));
        author_note.append(&close_note);
        author_note_revealer.set_child(Some(&author_note));
        author_note_revealer.set_reveal_child(model.authors.show_weight_note);
        content.append(&author_note_revealer);

        let author_list = model.update_author_list(&sender);
        content.append(&author_list);

        let widgets = AppWidgets {
            language_combo,
            quote_entry,
            bg_widgets: widget::BgWidgets {
                bg_entry,
                bg_rounded_switch,
                bg_fill_switch,
                bg_scale,
                bg_spin,
                bg_revealer,
            },
            stroke_revealer,
            stroke_entry,
            stroke_width_scale,
            stroke_width_spin,
            stroke_opacity_scale,
            stroke_opacity_spin,
            shadow_widgets,
            author_note_revealer,
            author_list,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, input: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        log::debug!("Received input: {:?}", input);
        match input {
            AppInput::UpdateFont(val) => {
                if !val.is_empty() {
                    self.settings.appearance.font = val;
                }
            }
            AppInput::UpdateFontSize(val) => self.settings.appearance.font_size = val,
            AppInput::UpdateLanguage(val) => self.settings.appearance.language = val,
            AppInput::UpdateTextColor(val) => {
                if !val.is_empty() && val.starts_with('#') {
                    self.settings.appearance.text_color = val;
                }
            }
            AppInput::UpdateBgColor(val) => {
                if !val.is_empty() && val.starts_with('#') {
                    self.settings.appearance.bg_color = val;
                }
            }
            AppInput::UpdateBgOpacity(a) => {
                let (r, g, b, _) =
                    engyls::config::parse_color_to_rgba(&self.settings.appearance.bg_color);
                self.settings.appearance.bg_color = engyls::config::rgba_to_hex(r, g, b, a as f64);
            }
            AppInput::UpdateBgEnabled(val) => self.settings.appearance.bg_enabled = val,
            AppInput::UpdateBgRounded(val) => self.settings.appearance.bg_rounded = val,
            AppInput::UpdateBgFill(val) => self.settings.appearance.bg_fill = val,
            AppInput::UpdateStrokeColor(val) => {
                if !val.is_empty() && val.starts_with('#') {
                    self.settings.appearance.stroke_color = val;
                }
            }
            AppInput::UpdateStrokeOpacity(a) => {
                let (r, g, b, _) =
                    engyls::config::parse_color_to_rgba(&self.settings.appearance.stroke_color);
                self.settings.appearance.stroke_color =
                    engyls::config::rgba_to_hex(r, g, b, a as f64);
            }
            AppInput::UpdateStrokeEnabled(val) => self.settings.appearance.stroke_enabled = val,
            AppInput::UpdateStrokeWidth(val) => self.settings.appearance.stroke_width = val,
            AppInput::UpdateShadowColor(val) => {
                if !val.is_empty() && val.starts_with('#') {
                    self.settings.appearance.shadow_color = val;
                }
            }
            AppInput::UpdateShadowEnabled(val) => self.settings.appearance.shadow_enabled = val,
            AppInput::UpdateShadowOpacity(a) => {
                let (r, g, b, _) =
                    engyls::config::parse_color_to_rgba(&self.settings.appearance.shadow_color);
                self.settings.appearance.shadow_color =
                    engyls::config::rgba_to_hex(r, g, b, a as f64);
            }
            AppInput::UpdateShadowOffset(val) => self.settings.appearance.shadow_offset = val,
            AppInput::UpdateShadowBlur(val) => self.settings.appearance.shadow_blur = val,
            AppInput::UpdateShadowSize(val) => self.settings.appearance.shadow_size = val,
            AppInput::UpdateQuoteHAlign(val) => self.settings.appearance.quote_h_align = val,
            AppInput::UpdateQuoteVAlign(val) => self.settings.appearance.quote_v_align = val,
            AppInput::UpdateAuthorHAlign(val) => self.settings.appearance.author_h_align = val,
            AppInput::UpdateAuthorName(index, name) => {
                if let Some(author) = self.authors.authors.get_mut(index) {
                    author.name = name;
                }
            }
            AppInput::UpdateAuthorWeight(index, weight) => {
                if let Some(author) = self.authors.authors.get_mut(index) {
                    author.weight = weight;
                }
            }
            AppInput::AddAuthor => self.add_author(),
            AppInput::RemoveAuthor(index) => {
                if index < self.authors.authors.len() {
                    self.authors.authors.remove(index);
                }
            }
            AppInput::DismissAuthorNote => {
                self.authors.show_weight_note = false;
                let _ = marxist_quote_core::save_authors(&self.authors);
            }
            AppInput::Save => {
                let (saved_settings, _) = marxist_quote_core::load_settings();
                let language_changed =
                    saved_settings.appearance.language != self.settings.appearance.language;
                self.settings.appearance.quote_x = saved_settings.appearance.quote_x;
                self.settings.appearance.quote_y = saved_settings.appearance.quote_y;
                self.settings.appearance.author_x = saved_settings.appearance.author_x;
                self.settings.appearance.author_y = saved_settings.appearance.author_y;
                self.settings.appearance.quote_max_width =
                    saved_settings.appearance.quote_max_width;
                self.settings.appearance.quote_max_height =
                    saved_settings.appearance.quote_max_height;
                self.settings.appearance.max_quote_chars =
                    saved_settings.appearance.max_quote_chars;
                self.settings.appearance.position_hash = self.settings.calculate_position_hash();
                let _ = marxist_quote_core::save_authors(&self.authors);
                let _ = marxist_quote_core::save_settings(&self.settings);
                std::thread::spawn(move || {
                    if language_changed || !marxist_quote_core::current_quote_exists() {
                        if let Err(e) = marxist_quote_core::fetch_quote() {
                            log::error!("Error fetching quote after save: {}", e);
                        }
                    }
                    restart_desktop_service();
                });
            }
            AppInput::ShowInteractivePicker => {
                let bin_path = std::env::current_exe()
                    .ok()
                    .and_then(|p| p.parent().map(|p| p.to_path_buf()))
                    .and_then(|dir| {
                        let path = dir.join("position-containers");
                        if path.exists() { Some(path) } else { None }
                    });

                if let Some(ref path) = bin_path {
                    log::info!("Launching position-containers at: {:?}", path);
                } else {
                    log::error!("position-containers binary not found next to GUI binary");
                    return;
                }

                let json = serde_json::to_string(&self.settings).unwrap_or_default();
                match std::process::Command::new(bin_path.unwrap())
                    .env("ENGYLS_PLACE_ARGS", json)
                    .spawn()
                {
                    Ok(child) => log::info!("position-containers spawned with PID {}", child.id()),
                    Err(e) => log::error!("Failed to spawn position-containers: {}", e),
                }
            }
            AppInput::FetchQuoteNow => {
                std::thread::spawn(|| {
                    if let Err(e) = marxist_quote_core::fetch_quote() {
                        log::error!("Error: {}", e);
                    } else {
                        restart_desktop_service();
                    }
                });
            }
            AppInput::SkipQuote => {
                std::thread::spawn(|| {
                    if let Err(e) = marxist_quote_core::fetch_quote() {
                        log::error!("Error skipping quote: {}", e);
                    } else {
                        restart_desktop_service();
                    }
                });
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        // --- MANUAL VIEW UPDATE WITH EQUALITY CHECKS ---
        let a = &self.settings.appearance;

        // Quote Color Entry
        if widgets.quote_entry.text() != a.text_color {
            widgets.quote_entry.set_text(&a.text_color);
        }

        if widgets.language_combo.active_id().as_deref() != Some(a.language.as_str()) {
            widgets.language_combo.set_active_id(Some(&a.language));
        }

        // Background expander
        if widgets.bg_widgets.bg_revealer.reveals_child() != a.bg_enabled {
            widgets
                .bg_widgets
                .bg_revealer
                .set_reveal_child(a.bg_enabled);
        }

        // BG Color Entry
        if widgets.bg_widgets.bg_entry.text() != a.bg_color {
            widgets.bg_widgets.bg_entry.set_text(&a.bg_color);
        }

        // Background rounded
        if widgets.bg_widgets.bg_rounded_switch.is_active() != a.bg_rounded {
            widgets
                .bg_widgets
                .bg_rounded_switch
                .set_active(a.bg_rounded);
        }

        // Background fill
        if widgets.bg_widgets.bg_fill_switch.is_active() != a.bg_fill {
            widgets.bg_widgets.bg_fill_switch.set_active(a.bg_fill);
        }

        // Opacity widgets
        let (_, _, _, ba) = engyls::config::parse_color_to_rgba(&a.bg_color);
        if (widgets.bg_widgets.bg_scale.value() - ba).abs() > 0.001 {
            widgets.bg_widgets.bg_scale.set_value(ba);
        }
        if (widgets.bg_widgets.bg_spin.value() - ba).abs() > 0.001 {
            widgets.bg_widgets.bg_spin.set_value(ba);
        }

        // Stroke
        if widgets.stroke_revealer.reveals_child() != a.stroke_enabled {
            widgets.stroke_revealer.set_reveal_child(a.stroke_enabled);
        }

        if widgets.stroke_entry.text() != a.stroke_color {
            widgets.stroke_entry.set_text(&a.stroke_color);
        }
        if (widgets.stroke_width_scale.value() - a.stroke_width as f64).abs() > 0.001 {
            widgets.stroke_width_scale.set_value(a.stroke_width as f64);
        }
        if (widgets.stroke_width_spin.value() - a.stroke_width as f64).abs() > 0.001 {
            widgets.stroke_width_spin.set_value(a.stroke_width as f64);
        }
        let (_, _, _, stroke_alpha) = engyls::config::parse_color_to_rgba(&a.stroke_color);
        if (widgets.stroke_opacity_scale.value() - stroke_alpha).abs() > 0.001 {
            widgets.stroke_opacity_scale.set_value(stroke_alpha);
        }
        if (widgets.stroke_opacity_spin.value() - stroke_alpha).abs() > 0.001 {
            widgets.stroke_opacity_spin.set_value(stroke_alpha);
        }

        // Shadow
        if widgets.shadow_widgets.shadow_revealer.reveals_child() != a.shadow_enabled {
            widgets
                .shadow_widgets
                .shadow_revealer
                .set_reveal_child(a.shadow_enabled);
        }

        if widgets.shadow_widgets.shadow_entry.text() != a.shadow_color {
            widgets
                .shadow_widgets
                .shadow_entry
                .set_text(&a.shadow_color);
        }

        if (widgets.shadow_widgets.shadow_scale.value() - a.shadow_offset as f64).abs() > 0.001 {
            widgets
                .shadow_widgets
                .shadow_scale
                .set_value(a.shadow_offset as f64);
        }
        if (widgets.shadow_widgets.shadow_spin.value() - a.shadow_offset as f64).abs() > 0.001 {
            widgets
                .shadow_widgets
                .shadow_spin
                .set_value(a.shadow_offset as f64);
        }
        let (_, _, _, sa) = engyls::config::parse_color_to_rgba(&a.shadow_color);
        if (widgets.shadow_widgets.shadow_opacity_scale.value() - sa).abs() > 0.001 {
            widgets.shadow_widgets.shadow_opacity_scale.set_value(sa);
        }
        if (widgets.shadow_widgets.shadow_opacity_spin.value() - sa).abs() > 0.001 {
            widgets.shadow_widgets.shadow_opacity_spin.set_value(sa);
        }
        if (widgets.shadow_widgets.shadow_blur_scale.value() - a.shadow_blur as f64).abs() > 0.001 {
            widgets
                .shadow_widgets
                .shadow_blur_scale
                .set_value(a.shadow_blur as f64);
        }
        if (widgets.shadow_widgets.shadow_blur_spin.value() - a.shadow_blur as f64).abs() > 0.001 {
            widgets
                .shadow_widgets
                .shadow_blur_spin
                .set_value(a.shadow_blur as f64);
        }
        if (widgets.shadow_widgets.shadow_size_scale.value() - a.shadow_size as f64).abs() > 0.001 {
            widgets
                .shadow_widgets
                .shadow_size_scale
                .set_value(a.shadow_size as f64);
        }
        if (widgets.shadow_widgets.shadow_size_spin.value() - a.shadow_size as f64).abs() > 0.001 {
            widgets
                .shadow_widgets
                .shadow_size_spin
                .set_value(a.shadow_size as f64);
        }

        if widgets.author_note_revealer.reveals_child() != self.authors.show_weight_note {
            widgets
                .author_note_revealer
                .set_reveal_child(self.authors.show_weight_note);
        }

        if child_count(&widgets.author_list) != self.authors.authors.len() {
            self.populate_author_list(&widgets.author_list, &_sender);
        }
    }
}
