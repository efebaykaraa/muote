pub struct BgWidgets {
    pub bg_entry: gtk::Entry,
    pub bg_rounded_switch: gtk::Switch,
    pub bg_fill_switch: gtk::Switch,
    pub bg_scale: gtk::Scale,
    pub bg_spin: gtk::SpinButton,
    pub bg_revealer: gtk::Revealer,
}

pub struct ShadowWidgets {
    pub shadow_entry: gtk::Entry,
    pub shadow_revealer: gtk::Revealer,
    pub shadow_scale: gtk::Scale,
    pub shadow_spin: gtk::SpinButton,
    pub shadow_opacity_scale: gtk::Scale,
    pub shadow_opacity_spin: gtk::SpinButton,
    pub shadow_blur_scale: gtk::Scale,
    pub shadow_blur_spin: gtk::SpinButton,
    pub shadow_size_scale: gtk::Scale,
    pub shadow_size_spin: gtk::SpinButton,
}

pub struct AppWidgets {
    pub language_combo: gtk::ComboBoxText,
    pub quote_entry: gtk::Entry,
    pub bg_widgets: BgWidgets,
    pub stroke_revealer: gtk::Revealer,
    pub stroke_entry: gtk::Entry,
    pub stroke_width_scale: gtk::Scale,
    pub stroke_width_spin: gtk::SpinButton,
    pub stroke_opacity_scale: gtk::Scale,
    pub stroke_opacity_spin: gtk::SpinButton,
    pub shadow_widgets: ShadowWidgets,
    pub author_note_revealer: gtk::Revealer,
    pub author_list: gtk::Box,
}
