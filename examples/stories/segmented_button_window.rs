// ============================================================================
// Module:       stories::segmented_button_window (example)
// Description:  Story for segmented buttons and dividers.
//
// Dependencies: eframe, egui_material3
// ============================================================================

//! Segmented buttons (single and multi select, icons, icon-only, disabled,
//! full width) and dividers (full, inset, middle inset, vertical).

#![doc(hidden)]

use eframe::egui::{self, Window};
use egui_material3::{divider, vertical_divider, MaterialDivider, Segment, SegmentedButton};

#[doc(hidden)]
pub struct SegmentedButtonWindow {
    pub open: bool,
    view: usize,
    density: Option<usize>,
    sizes: [bool; 4],
    format: [bool; 3],
    full_width: usize,
    show_check: bool,
    enabled: bool,
}

impl Default for SegmentedButtonWindow {
    fn default() -> Self {
        Self {
            open: false,
            view: 1,
            density: None,
            sizes: [false, true, true, false],
            format: [true, false, false],
            full_width: 0,
            show_check: true,
            enabled: true,
        }
    }
}

impl SegmentedButtonWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Segmented Button & Divider Stories")
            .open(&mut open)
            .default_size([640.0, 600.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render(ui);
                });
            });
        self.open = open;
    }

    fn render(&mut self, ui: &mut egui::Ui) {
        ui.heading("Segmented buttons");
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_check, "Show check");
            ui.checkbox(&mut self.enabled, "Enabled");
        });
        ui.add_space(8.0);

        ui.label("Single select (arrows move the selection)");
        ui.add(
            SegmentedButton::single(&mut self.view)
                .label("View")
                .show_check(self.show_check)
                .enabled(self.enabled)
                .segment(Segment::new("Day").icon("calendar_view_day"))
                .segment(Segment::new("Week").icon("calendar_view_week"))
                .segment(Segment::new("Month").icon("calendar_view_month"))
                .segment(Segment::new("Year").enabled(false)),
        );
        ui.label(format!("Selected: {}", self.view));
        ui.add_space(12.0);

        ui.label("Single select, nothing selected yet");
        ui.add(
            SegmentedButton::single_optional(&mut self.density)
                .label("Density")
                .show_check(self.show_check)
                .enabled(self.enabled)
                .segments(["Compact", "Default", "Comfortable"]),
        );
        ui.add_space(12.0);

        ui.label("Multi select");
        ui.add(
            SegmentedButton::multi(&mut self.sizes)
                .label("Sizes")
                .show_check(self.show_check)
                .enabled(self.enabled)
                .segments(["XS", "S", "M", "L"]),
        );
        ui.add_space(12.0);

        ui.label("Icon-only multi select");
        ui.add(
            SegmentedButton::multi(&mut self.format)
                .label("Text format")
                .show_check(false)
                .enabled(self.enabled)
                .segment(Segment::icon_only("format_bold", "Bold"))
                .segment(Segment::icon_only("format_italic", "Italic"))
                .segment(Segment::icon_only("format_underlined", "Underline")),
        );
        ui.add_space(12.0);

        ui.label("Full width");
        ui.add(
            SegmentedButton::single(&mut self.full_width)
                .full_width(true)
                .show_check(self.show_check)
                .enabled(self.enabled)
                .segments(["Songs", "Albums", "Podcasts"]),
        );

        ui.add_space(24.0);
        ui.heading("Dividers");
        ui.label("Full width");
        ui.add(divider());
        ui.label("Inset (16 dp at the start)");
        ui.add(MaterialDivider::inset());
        ui.label("Middle inset (16 dp at both ends)");
        ui.add(MaterialDivider::middle_inset());
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.set_height(40.0);
            ui.label("Left");
            ui.add(vertical_divider());
            ui.label("Right");
        });
    }
}
