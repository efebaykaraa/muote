use crate::app::input::AppInput;
use adw::prelude::*;
use muote_core::config::DisplayArgs;
use muote_core::{Author, AuthorsConfig, QuoteIntervalUnit};
use relm4::{ComponentSender, gtk};

#[derive(Clone)]
pub struct AppModel {
    pub authors: AuthorsConfig,
    pub settings: DisplayArgs,
    pub quote_interval_value: u32,
    pub quote_interval_unit: QuoteIntervalUnit,
    pub saved_quote_interval_value: u32,
    pub saved_quote_interval_unit: QuoteIntervalUnit,
    pub _authors_hash: String,
    pub _settings_hash: String,
}

impl AppModel {
    pub fn init_model() -> Self {
        let (authors, authors_hash) = muote_core::load_authors();
        let (settings, settings_hash) = muote_core::load_settings();
        let (quote_interval_value, quote_interval_unit) =
            muote_core::load_quote_timer_interval().unwrap_or((1, QuoteIntervalUnit::Days));

        AppModel {
            authors,
            settings,
            quote_interval_value,
            quote_interval_unit,
            saved_quote_interval_value: quote_interval_value,
            saved_quote_interval_unit: quote_interval_unit,
            _authors_hash: authors_hash,
            _settings_hash: settings_hash,
        }
    }

    pub fn update_author_list(&self, sender: &ComponentSender<super::AppModel>) -> gtk::Box {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 10);
        self.populate_author_list(&container, sender);
        container
    }

    pub fn populate_author_list(
        &self,
        container: &gtk::Box,
        sender: &ComponentSender<super::AppModel>,
    ) {
        while let Some(child) = container.first_child() {
            container.remove(&child);
        }

        for (index, author) in self.authors.authors.iter().enumerate() {
            let row = gtk::Box::new(gtk::Orientation::Horizontal, 12);

            let name_entry = gtk::Entry::new();
            name_entry.set_text(&author.name);
            name_entry.set_hexpand(true);
            name_entry.set_tooltip_text(Some("WikiQuote author page title."));
            let sender_clone = sender.clone();
            name_entry.connect_changed(move |entry| {
                sender_clone.input(AppInput::UpdateAuthorName(index, entry.text().to_string()));
            });
            row.append(&name_entry);

            let weight_spin = gtk::SpinButton::with_range(0.0, 10.0, 1.0);
            weight_spin.set_value(author.weight as f64);
            weight_spin.set_tooltip_text(Some(
                "Relative weight: how often this author is picked compared to others.",
            ));

            let sender_clone = sender.clone();
            weight_spin.connect_value_changed(move |spin| {
                sender_clone.input(AppInput::UpdateAuthorWeight(index, spin.value() as u32));
            });

            row.append(&weight_spin);

            let remove_btn = gtk::Button::with_label("Remove");
            remove_btn.set_tooltip_text(Some("Remove this author."));
            let sender_clone = sender.clone();
            remove_btn.connect_clicked(move |_| {
                sender_clone.input(AppInput::RemoveAuthor(index));
            });
            row.append(&remove_btn);

            container.append(&row);
        }
    }

    pub fn add_author(&mut self) {
        self.authors.authors.push(Author {
            name: "New Author".into(),
            weight: 1,
        });
    }
}
