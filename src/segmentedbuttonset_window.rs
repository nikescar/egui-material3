use eframe::egui::{self, Ui, Window};
use crate::{MaterialSegmentedButtonSet, segmented_button};

pub struct SegmentedButtonSetWindow {
    pub open: bool,
    // Different examples with their own state
    example1_selected: Vec<bool>,
    example2_selected: Vec<bool>,
    example3_selected: Vec<bool>,
    example4_selected: Vec<bool>,
    example5_selected: Vec<bool>,
    example6_selected: Vec<bool>,
    example7_selected: Vec<bool>,
}

impl Default for SegmentedButtonSetWindow {
    fn default() -> Self {
        Self {
            open: false,
            example1_selected: vec![false, true, false], // Match Material Web example
            example2_selected: vec![true, false, false],
            example3_selected: vec![true, true, false, false], // Multiselect
            example4_selected: vec![false, false, true], // Transportation
            example5_selected: vec![true, false, false], // Icon only
            example6_selected: vec![false, false, false, false], // Single select
            example7_selected: vec![false, true, false, false], // Multi select
        }
    }
}

impl SegmentedButtonSetWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Segmented Button Set Stories")
            .open(&mut open)
            .default_size([700.0, 700.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_examples(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Segmented Button Set Controls");

            if ui.button("Target").clicked() {
                let _ = webbrowser::open("https://m3.material.io/components/segmented-buttons/overview");
            }
        });
        
        ui.label("Interactive examples demonstrating various segmented button set configurations:");
    }

    fn render_examples(&mut self, ui: &mut Ui) {
        // Example 1: Labels and icons
        ui.heading("Labels and Icons");
        ui.label("Standard segmented button set with labels and icons:");
        ui.add(MaterialSegmentedButtonSet::new(&mut self.example1_selected)
            .add_button(segmented_button().label("Enabled").icon("⭐"))
            .add_button(segmented_button().label("Selected").icon("❤️"))
            .add_button(segmented_button().label("Enabled").icon("🔺"))
            .width(325.0));

        ui.add_space(20.0);

        // Example 2: Without icons
        ui.heading("Without Icons");
        ui.label("Segmented button set with text labels only:");
        ui.add(MaterialSegmentedButtonSet::new(&mut self.example2_selected)
            .button("Selected", None)
            .button("Enabled", None)
            .button("Enabled", None)
            .width(325.0));

        ui.add_space(20.0);

        // Example 3: Multiselect
        ui.heading("Multiselect");
        ui.label("Multiple buttons can be selected simultaneously:");
        ui.add(MaterialSegmentedButtonSet::new(&mut self.example3_selected)
            .button("$", None)
            .button("$$", None)
            .button("$$$", None)
            .button("$$$$", None)
            .multiselect(true)
            .width(325.0));

        ui.add_space(20.0);

        // Example 4: Transportation modes
        ui.heading("Transportation Modes");
        ui.label("Icon-only buttons for transportation selection:");
        ui.add(MaterialSegmentedButtonSet::new(&mut self.example4_selected)
            .icon_button("🚶", false)
            .icon_button("🚇", false)
            .icon_button("🚗", false)
            .width(202.0));

        ui.add_space(20.0);

        // Example 5: Icon only (no checkmark)
        ui.heading("Icon Only (No Checkmark)");
        ui.label("Icon-only buttons without checkmarks when selected:");
        ui.add(MaterialSegmentedButtonSet::new(&mut self.example5_selected)
            .icon_button("◀", true)
            .icon_button("■", true)
            .icon_button("▶", true)
            .width(202.0));

        ui.add_space(20.0);

        // Example 6: Single select
        ui.heading("Single Select");
        ui.label("Single selection with mixed content and disabled button:");
        ui.add(MaterialSegmentedButtonSet::new(&mut self.example6_selected)
            .add_button(segmented_button().label("Label").icon("⭐"))
            .add_button(segmented_button().label("Label").disabled(true))
            .icon_button("🚌", false)
            .button("Label", None)
            .width(400.0));

        ui.add_space(20.0);

        // Example 7: Multi select
        ui.heading("Multi Select");
        ui.label("Multi-selection with disabled selected button:");
        ui.add(MaterialSegmentedButtonSet::new(&mut self.example7_selected)
            .button("Label", None)
            .add_button(segmented_button().label("Label").disabled(true))
            .button("Label", None)
            .button("Label", None)
            .multiselect(true)
            .width(400.0));

        ui.add_space(20.0);

        // Additional interactive examples
        ui.heading("Additional Examples");

        ui.label("Basic 3-button set:");
        let mut basic_selected = vec![true, false, false];
        ui.add(MaterialSegmentedButtonSet::new(&mut basic_selected)
            .button("Option A", Some("🅰️"))
            .button("Option B", Some("🅱️"))
            .button("Option C", Some("🅾️"))
            .width(300.0));

        ui.add_space(10.0);

        ui.label("Text formatting buttons (multi-select):");
        let mut format_selected = vec![false, true, false, false];
        ui.add(MaterialSegmentedButtonSet::new(&mut format_selected)
            .icon_button("B", true)  // Bold
            .icon_button("I", true)  // Italic
            .icon_button("U", true)  // Underline
            .icon_button("S", true)  // Strikethrough
            .multiselect(true)
            .width(200.0));

        ui.add_space(10.0);

        ui.label("View modes:");
        let mut view_selected = vec![false, true, false];
        ui.add(MaterialSegmentedButtonSet::new(&mut view_selected)
            .icon_button("📝", false)  // List view
            .icon_button("🔳", false)  // Grid view
            .icon_button("📊", false)  // Chart view
            .width(150.0));

        ui.add_space(20.0);

        // Show current selections
        ui.separator();
        ui.heading("Current Selections");
        ui.label(format!("Labels and Icons: {:?}", self.example1_selected));
        ui.label(format!("Without Icons: {:?}", self.example2_selected));
        ui.label(format!("Multiselect: {:?}", self.example3_selected));
        ui.label(format!("Transportation: {:?}", self.example4_selected));
        ui.label(format!("Icon Only: {:?}", self.example5_selected));
        ui.label(format!("Single Select: {:?}", self.example6_selected));
        ui.label(format!("Multi Select: {:?}", self.example7_selected));

        ui.add_space(10.0);
        ui.label("💡 Tips:");
        ui.label("• Single-select mode: Only one button can be selected at a time");
        ui.label("• Multi-select mode: Multiple buttons can be selected");
        ui.label("• Disabled buttons cannot be interacted with");
        ui.label("• Icon-only buttons can hide checkmarks when selected");
    }
}