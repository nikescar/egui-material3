// ============================================================================
// Module:       stories::search_window (example)
// Description:  Story for the search bar and the docked/full-screen view.
//
// Dependencies: eframe, egui_material3
// ============================================================================

//! The search story: a search bar over a list of fruit, with the view
//! docked or full screen, suggestions filtered as you type, and the last
//! submitted query or picked suggestion shown underneath.

use eframe::egui::{self, Window};
use egui_material3::search::{MaterialSearch, SearchViewMode, Suggestion};

const FRUIT: [&str; 12] = [
    "Apple",
    "Apricot",
    "Avocado",
    "Banana",
    "Blackberry",
    "Blueberry",
    "Cherry",
    "Coconut",
    "Grape",
    "Lemon",
    "Mango",
    "Orange",
];

pub(crate) struct SearchWindow {
    pub(crate) open: bool,
    query: String,
    full_screen: bool,
    with_actions: bool,
    last_event: String,
}

impl Default for SearchWindow {
    fn default() -> Self {
        Self {
            open: false,
            query: String::new(),
            full_screen: false,
            with_actions: true,
            last_event: "Nothing yet".to_owned(),
        }
    }
}

impl SearchWindow {
    pub(crate) fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Search Stories")
            .open(&mut open)
            .default_size([560.0, 520.0])
            .show(ctx, |ui| self.contents(ui));
        self.open = open;
    }

    fn contents(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.full_screen, "Full-screen view");
            ui.checkbox(&mut self.with_actions, "Trailing actions");
        });
        ui.label("Click the bar or Tab into it. Up/Down move through the suggestions, Enter picks, Escape closes.");
        ui.add_space(12.0);

        let needle = self.query.to_lowercase();
        let suggestions: Vec<Suggestion> = FRUIT
            .iter()
            .filter(|f| f.to_lowercase().contains(&needle))
            .map(|f| Suggestion::new(*f).icon("history"))
            .collect();
        let mode = if self.full_screen {
            SearchViewMode::FullScreen
        } else {
            SearchViewMode::Docked
        };
        let mut search = MaterialSearch::new("search_story", &mut self.query)
            .placeholder("Search fruit")
            .mode(mode)
            .suggestions(suggestions.clone());
        if self.with_actions {
            search = search
                .trailing_icon("mic", "Voice search")
                .trailing_icon("account_circle", "Account");
        }
        let out = search.show(ui);

        if let Some(text) = out.submitted {
            self.last_event = format!("Submitted \"{text}\"");
        }
        if let Some(text) = out.selected.and_then(|i| suggestions.get(i)) {
            self.last_event = format!("Picked \"{}\"", text.text);
        }
        if let Some(i) = out.trailing_clicked {
            self.last_event = format!("Trailing action {i} clicked");
        }
        ui.add_space(16.0);
        ui.label(format!("Last event: {}", self.last_event));
        ui.label(format!("Expanded: {}", out.expanded));
        ui.add_space(8.0);
        for fruit in FRUIT {
            ui.label(fruit);
        }
    }
}
