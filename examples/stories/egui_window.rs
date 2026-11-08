#![doc(hidden)]

use eframe::egui::{self, Color32, Ui, Window};

#[derive(Debug, PartialEq)]
enum Enum {
    First,
    Second,
    Third,
}

impl Default for Enum {
    fn default() -> Self {
        Self::First
    }
}

#[doc(hidden)]
pub struct EguiWindow {
    pub open: bool,
    enabled: bool,
    visible: bool,
    opacity: f32,
    boolean: bool,
    radio: Enum,
    scalar: f32,
    range_min: f32,
    range_max: f32,
    string: String,
    color: Color32,
    animate_progress_bar: bool,
}

impl Default for EguiWindow {
    fn default() -> Self {
        Self {
            open: false,
            enabled: true,
            visible: true,
            opacity: 1.0,
            boolean: false,
            radio: Enum::default(),
            scalar: 42.0,
            range_min: 90.0,
            range_max: 250.0,
            string: String::new(),
            color: Color32::LIGHT_BLUE.linear_multiply(0.5),
            animate_progress_bar: false,
        }
    }
}

impl EguiWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Egui Widget Gallery")
            .open(&mut open)
            .default_size([900.0, 700.0])
            .resizable(true)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_richtext_showcase(ui);
                    ui.add_space(20.0);
                    self.render_widget_gallery(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.heading("Controls");

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.visible, "Visible")
                .on_hover_text("Uncheck to hide all widgets");

            if self.visible {
                ui.checkbox(&mut self.enabled, "Interactive")
                    .on_hover_text("Uncheck to see disabled state");

                ui.label("Opacity:");
                ui.add(
                    egui::DragValue::new(&mut self.opacity)
                        .speed(0.01)
                        .range(0.0..=1.0),
                );
            }
        });

        ui.separator();
    }

    fn render_richtext_showcase(&mut self, ui: &mut Ui) {
        ui.heading("RichText Showcase");
        ui.label("Various text formatting options using egui::RichText:");

        ui.add_space(10.0);

        egui::Grid::new("richtext_grid")
            .num_columns(2)
            .spacing([40.0, 8.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Plain text:");
                ui.label("This is plain text");
                ui.end_row();

                ui.label("Heading:");
                ui.label(egui::RichText::new("Large Heading").heading());
                ui.end_row();

                ui.label("Strong:");
                ui.label(egui::RichText::new("Bold Text").strong());
                ui.end_row();

                ui.label("Weak:");
                ui.label(egui::RichText::new("Dimmed Text").weak());
                ui.end_row();

                ui.label("Small:");
                ui.label(egui::RichText::new("Small Text").small());
                ui.end_row();

                ui.label("Monospace:");
                ui.label(egui::RichText::new("Code Style Text").monospace());
                ui.end_row();

                ui.label("Colored:");
                ui.label(egui::RichText::new("Red Text").color(Color32::RED));
                ui.end_row();

                ui.label("Background:");
                ui.label(
                    egui::RichText::new("Highlighted")
                        .background_color(Color32::from_rgb(255, 255, 0))
                );
                ui.end_row();

                ui.label("Underline:");
                ui.label(egui::RichText::new("Underlined Text").underline());
                ui.end_row();

                ui.label("Strikethrough:");
                ui.label(egui::RichText::new("Strikethrough Text").strikethrough());
                ui.end_row();

                ui.label("Italics:");
                ui.label(egui::RichText::new("Italic Text").italics());
                ui.end_row();

                ui.label("Combined:");
                ui.label(
                    egui::RichText::new("Bold + Colored + Large")
                        .strong()
                        .color(Color32::from_rgb(0, 128, 255))
                        .heading()
                );
                ui.end_row();
            });

        ui.separator();
    }

    fn render_widget_gallery(&mut self, ui: &mut Ui) {
        ui.heading("Widget Gallery (from egui)");
        ui.label("All standard egui widgets to verify theme compliance:");

        ui.add_space(10.0);

        let mut ui_builder = egui::UiBuilder::new();
        if !self.enabled {
            ui_builder = ui_builder.disabled();
        }
        if !self.visible {
            ui_builder = ui_builder.invisible();
        }

        ui.scope_builder(ui_builder, |ui| {
            ui.multiply_opacity(self.opacity);

            egui::Grid::new("widget_gallery_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    self.gallery_grid_contents(ui);
                });
        });

        ui.separator();

        ui.vertical_centered(|ui| {
            ui.hyperlink_to(
                "egui documentation",
                "https://docs.rs/egui/"
            ).on_hover_text("Full egui documentation");
        });
    }

    fn gallery_grid_contents(&mut self, ui: &mut Ui) {
        // Label
        ui.label("Label:");
        ui.label("Welcome to the widget gallery!");
        ui.end_row();

        // Hyperlink
        ui.label("Hyperlink:");
        ui.hyperlink_to(
            "egui on GitHub",
            "https://github.com/emilk/egui"
        );
        ui.end_row();

        // TextEdit single line
        ui.label("TextEdit:");
        ui.add(
            egui::TextEdit::singleline(&mut self.string)
                .hint_text("Write something here")
        );
        ui.end_row();

        // Button
        ui.label("Button:");
        if ui.button("Click me!").clicked() {
            self.boolean = !self.boolean;
        }
        ui.end_row();

        // Link
        ui.label("Link:");
        if ui.link("Click me!").clicked() {
            self.boolean = !self.boolean;
        }
        ui.end_row();

        // Checkbox
        ui.label("Checkbox:");
        ui.checkbox(&mut self.boolean, "Checkbox");
        ui.end_row();

        // RadioButton
        ui.label("RadioButton:");
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.radio, Enum::First, "First");
            ui.radio_value(&mut self.radio, Enum::Second, "Second");
            ui.radio_value(&mut self.radio, Enum::Third, "Third");
        });
        ui.end_row();

        // SelectableLabel
        ui.label("SelectableLabel:");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.radio, Enum::First, "First");
            ui.selectable_value(&mut self.radio, Enum::Second, "Second");
            ui.selectable_value(&mut self.radio, Enum::Third, "Third");
        });
        ui.end_row();

        // ComboBox
        ui.label("ComboBox:");
        egui::ComboBox::from_label("Pick one")
            .selected_text(format!("{:?}", self.radio))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.radio, Enum::First, "First");
                ui.selectable_value(&mut self.radio, Enum::Second, "Second");
                ui.selectable_value(&mut self.radio, Enum::Third, "Third");
            });
        ui.end_row();

        // Slider
        ui.label("Slider:");
        ui.add(egui::Slider::new(&mut self.scalar, 0.0..=360.0).suffix("°"));
        ui.end_row();

        // Range sliders (two sliders for min/max in egui 0.33)
        ui.label("Range (min):");
        ui.add(egui::Slider::new(&mut self.range_min, 0.0..=360.0).suffix("°"));
        ui.end_row();

        ui.label("Range (max):");
        ui.add(egui::Slider::new(&mut self.range_max, 0.0..=360.0).suffix("°"));
        ui.end_row();

        // DragValue
        ui.label("DragValue:");
        ui.add(egui::DragValue::new(&mut self.scalar).speed(1.0));
        ui.end_row();

        // ProgressBar
        ui.label("ProgressBar:");
        let progress = self.scalar / 360.0;
        let progress_bar = egui::ProgressBar::new(progress)
            .show_percentage()
            .animate(self.animate_progress_bar);
        self.animate_progress_bar = ui
            .add(progress_bar)
            .on_hover_text("Hover to animate!")
            .hovered();
        ui.end_row();

        // Color picker
        ui.label("Color picker:");
        ui.color_edit_button_srgba(&mut self.color);
        ui.end_row();

        // Separator
        ui.label("Separator:");
        ui.separator();
        ui.end_row();

        // CollapsingHeader
        ui.label("CollapsingHeader:");
        ui.collapsing("Click to expand", |ui| {
            ui.label("Hidden content revealed!");
            ui.add(egui::Spinner::new());
        });
        ui.end_row();

        // Spinner
        ui.label("Spinner:");
        ui.add(egui::Spinner::new());
        ui.end_row();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state() {
        let window = EguiWindow::default();

        assert!(!window.open);
        assert!(window.enabled);
        assert!(window.visible);
        assert_eq!(window.opacity, 1.0);
        assert!(!window.boolean);
        assert_eq!(window.radio, Enum::First);
        assert_eq!(window.scalar, 42.0);
        assert_eq!(window.range_min, 90.0);
        assert_eq!(window.range_max, 250.0);
        assert_eq!(window.string, "");
    }

    #[test]
    fn test_boolean_toggle() {
        let mut window = EguiWindow::default();

        assert!(!window.boolean);
        window.boolean = !window.boolean;
        assert!(window.boolean);
        window.boolean = !window.boolean;
        assert!(!window.boolean);
    }

    #[test]
    fn test_radio_selection() {
        let mut window = EguiWindow::default();

        assert_eq!(window.radio, Enum::First);
        window.radio = Enum::Second;
        assert_eq!(window.radio, Enum::Second);
        window.radio = Enum::Third;
        assert_eq!(window.radio, Enum::Third);
    }

    #[test]
    fn test_slider_bounds() {
        let mut window = EguiWindow::default();

        window.scalar = 0.0;
        assert_eq!(window.scalar, 0.0);

        window.scalar = 360.0;
        assert_eq!(window.scalar, 360.0);

        window.scalar = 180.0;
        assert_eq!(window.scalar, 180.0);
    }

    #[test]
    fn test_range_slider_bounds() {
        let mut window = EguiWindow::default();

        assert_eq!(window.range_min, 90.0);
        assert_eq!(window.range_max, 250.0);

        window.range_min = 0.0;
        window.range_max = 360.0;

        assert_eq!(window.range_min, 0.0);
        assert_eq!(window.range_max, 360.0);
    }

    #[test]
    fn test_string_edit() {
        let mut window = EguiWindow::default();

        assert_eq!(window.string, "");

        window.string = "test".to_string();
        assert_eq!(window.string, "test");

        window.string.clear();
        assert_eq!(window.string, "");
    }

    #[test]
    fn test_opacity_range() {
        let mut window = EguiWindow::default();

        assert_eq!(window.opacity, 1.0);

        window.opacity = 0.5;
        assert_eq!(window.opacity, 0.5);

        window.opacity = 0.0;
        assert_eq!(window.opacity, 0.0);
    }
}
