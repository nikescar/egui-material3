#![doc(hidden)]

use crate::{timeline, MaterialButton, MaterialTimeline, TimelineDot, TimelineDotColor, TimelineDotVariant, TimelineItem, TimelinePosition};
use eframe::egui::{self, Color32, Window};

#[doc(hidden)]
pub struct TimelineWindow {
    pub open: bool,
    position_selection: TimelinePosition,
}

impl Default for TimelineWindow {
    fn default() -> Self {
        Self {
            open: false,
            position_selection: TimelinePosition::Right,
        }
    }
}

impl TimelineWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Timeline Stories")
            .open(&mut open)
            .default_size([900.0, 700.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_basic_timeline(ui);
                    ui.add_space(20.0);
                    self.render_opposite_content(ui);
                    ui.add_space(20.0);
                    self.render_customization(ui);
                    ui.add_space(20.0);
                    self.render_position_variants(ui);
                    ui.add_space(20.0);
                    self.render_color_variants(ui);
                    ui.add_space(20.0);
                    self.render_interactive_timeline(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.push_id("timeline_controls", |ui| {
            ui.horizontal(|ui| {
                ui.heading("Timeline Controls");

                if ui.add(MaterialButton::filled("Target").small()).clicked() {
                    let _ = webbrowser::open("https://mui.com/material-ui/react-timeline/");
                }
            });

            ui.horizontal(|ui| {
                ui.label("Position:");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", self.position_selection))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.position_selection, TimelinePosition::Left, "Left");
                        ui.selectable_value(&mut self.position_selection, TimelinePosition::Right, "Right");
                        ui.selectable_value(&mut self.position_selection, TimelinePosition::Alternate, "Alternate");
                        ui.selectable_value(&mut self.position_selection, TimelinePosition::AlternateReverse, "Alternate Reverse");
                    });
            });
        });
    }

    fn render_basic_timeline(&self, ui: &mut egui::Ui) {
        ui.heading("Basic Timeline");
        ui.label("A simple timeline showing events in chronological order");

        ui.add_space(10.0);

        ui.add(
            timeline()
                .position(TimelinePosition::Right)
                .item(
                    TimelineItem::new()
                        .content("Eat")
                        .dot(TimelineDot::new().color(TimelineDotColor::Grey)),
                )
                .item(
                    TimelineItem::new()
                        .content("Code")
                        .dot(TimelineDot::new().color(TimelineDotColor::Grey)),
                )
                .item(
                    TimelineItem::new()
                        .content("Sleep")
                        .dot(TimelineDot::new().color(TimelineDotColor::Grey)),
                )
                .item(
                    TimelineItem::new()
                        .content("Repeat")
                        .dot(TimelineDot::new().color(TimelineDotColor::Grey)),
                ),
        );
    }

    fn render_opposite_content(&self, ui: &mut egui::Ui) {
        ui.heading("Opposite Content");
        ui.label("Timeline with time stamps on the opposite side");

        ui.add_space(10.0);

        ui.add(
            timeline()
                .position(TimelinePosition::Right)
                .item(
                    TimelineItem::new()
                        .opposite_content("09:30 am")
                        .content("Eat")
                        .dot(TimelineDot::new().color(TimelineDotColor::Primary)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("10:00 am")
                        .content("Code")
                        .dot(TimelineDot::new().color(TimelineDotColor::Primary)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("12:00 am")
                        .content("Sleep")
                        .dot(TimelineDot::new().color(TimelineDotColor::Primary)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("9:00 am")
                        .content("Repeat")
                        .dot(TimelineDot::new().color(TimelineDotColor::Primary)),
                ),
        );

        ui.add_space(20.0);

        ui.label("Alternate timeline with opposite content");
        ui.add_space(10.0);

        ui.add(
            timeline()
                .position(TimelinePosition::Alternate)
                .item(
                    TimelineItem::new()
                        .opposite_content("09:30 am")
                        .content("Eat")
                        .dot(TimelineDot::new().color(TimelineDotColor::Primary)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("10:00 am")
                        .content("Code")
                        .dot(TimelineDot::new().color(TimelineDotColor::Primary)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("12:00 am")
                        .content("Sleep")
                        .dot(TimelineDot::new().color(TimelineDotColor::Primary)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("9:00 am")
                        .content("Repeat")
                        .dot(TimelineDot::new().color(TimelineDotColor::Primary)),
                ),
        );
    }

    fn render_customization(&self, ui: &mut egui::Ui) {
        ui.heading("Customization");
        ui.label("Timeline with custom colors, icons, and variants");

        ui.add_space(10.0);

        ui.label("📝 Outlined variant with custom colors and larger icons:");
        ui.add_space(5.0);

        ui.add(
            timeline()
                .position(TimelinePosition::Right)
                .item(
                    TimelineItem::new()
                        .opposite_content("2024-01-15")
                        .content("Project Kickoff")
                        .dot(
                            TimelineDot::new()
                                .variant(TimelineDotVariant::Outlined)
                                .color(TimelineDotColor::Primary)
                                .icon("🚀")
                                .size(32.0),
                        )
                        .content_color(Color32::from_rgb(25, 118, 210)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("2024-02-01")
                        .content("Design Phase")
                        .dot(
                            TimelineDot::new()
                                .variant(TimelineDotVariant::Outlined)
                                .color(TimelineDotColor::Info)
                                .icon("🎨")
                                .size(32.0),
                        )
                        .content_color(Color32::from_rgb(156, 39, 176)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("2024-03-01")
                        .content("Development")
                        .dot(
                            TimelineDot::new()
                                .variant(TimelineDotVariant::Outlined)
                                .color(TimelineDotColor::Warning)
                                .icon("⚙️")
                                .size(32.0),
                        )
                        .content_color(Color32::from_rgb(255, 152, 0)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("2024-04-15")
                        .content("Launch")
                        .dot(
                            TimelineDot::new()
                                .variant(TimelineDotVariant::Filled)
                                .color(TimelineDotColor::Success)
                                .icon("✓")
                                .size(32.0),
                        )
                        .content_color(Color32::from_rgb(76, 175, 80)),
                ),
        );

        ui.add_space(20.0);

        ui.label("🎭 Custom themed timeline with large icons:");
        ui.add_space(5.0);

        ui.add(
            timeline()
                .position(TimelinePosition::Alternate)
                .item(
                    TimelineItem::new()
                        .opposite_content("Stage 1")
                        .content("Planning & Research")
                        .dot(
                            TimelineDot::new()
                                .variant(TimelineDotVariant::Filled)
                                .custom_color(Color32::from_rgb(103, 80, 164))
                                .icon("📋")
                                .size(40.0),
                        )
                        .content_color(Color32::from_rgb(103, 80, 164))
                        .opposite_content_color(Color32::from_rgb(156, 39, 176)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("Stage 2")
                        .content("Design & Prototyping")
                        .dot(
                            TimelineDot::new()
                                .variant(TimelineDotVariant::Filled)
                                .custom_color(Color32::from_rgb(233, 30, 99))
                                .icon("✏️")
                                .size(40.0),
                        )
                        .content_color(Color32::from_rgb(233, 30, 99))
                        .opposite_content_color(Color32::from_rgb(156, 39, 176)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("Stage 3")
                        .content("Implementation")
                        .dot(
                            TimelineDot::new()
                                .variant(TimelineDotVariant::Filled)
                                .custom_color(Color32::from_rgb(0, 150, 136))
                                .icon("💻")
                                .size(40.0),
                        )
                        .content_color(Color32::from_rgb(0, 150, 136))
                        .opposite_content_color(Color32::from_rgb(156, 39, 176)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("Stage 4")
                        .content("Testing & Deployment")
                        .dot(
                            TimelineDot::new()
                                .variant(TimelineDotVariant::Filled)
                                .custom_color(Color32::from_rgb(255, 193, 7))
                                .icon("🚢")
                                .size(40.0),
                        )
                        .content_color(Color32::from_rgb(255, 193, 7))
                        .opposite_content_color(Color32::from_rgb(156, 39, 176)),
                ),
        );
    }

    fn render_position_variants(&self, ui: &mut egui::Ui) {
        ui.heading("Position Variants");
        ui.label(format!("Current position: {:?}", self.position_selection));

        ui.add_space(10.0);

        ui.add(
            timeline()
                .position(self.position_selection)
                .item(
                    TimelineItem::new()
                        .content("First event")
                        .dot(TimelineDot::new().color(TimelineDotColor::Primary)),
                )
                .item(
                    TimelineItem::new()
                        .content("Second event")
                        .dot(TimelineDot::new().color(TimelineDotColor::Secondary)),
                )
                .item(
                    TimelineItem::new()
                        .content("Third event")
                        .dot(TimelineDot::new().color(TimelineDotColor::Error)),
                )
                .item(
                    TimelineItem::new()
                        .content("Fourth event")
                        .dot(TimelineDot::new().color(TimelineDotColor::Success)),
                ),
        );
    }

    fn render_color_variants(&self, ui: &mut egui::Ui) {
        ui.heading("Color Variants");
        ui.label("Timeline showcasing all available color schemes");

        ui.add_space(10.0);

        ui.add(
            timeline()
                .position(TimelinePosition::Right)
                .item(
                    TimelineItem::new()
                        .opposite_content("Grey")
                        .content("Default grey color scheme")
                        .dot(TimelineDot::new().color(TimelineDotColor::Grey)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("Primary")
                        .content("Primary theme color")
                        .dot(TimelineDot::new().color(TimelineDotColor::Primary)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("Secondary")
                        .content("Secondary theme color")
                        .dot(TimelineDot::new().color(TimelineDotColor::Secondary)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("Error")
                        .content("Error or danger state")
                        .dot(TimelineDot::new().color(TimelineDotColor::Error)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("Info")
                        .content("Informational state")
                        .dot(TimelineDot::new().color(TimelineDotColor::Info)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("Success")
                        .content("Success or completion state")
                        .dot(TimelineDot::new().color(TimelineDotColor::Success)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("Warning")
                        .content("Warning or caution state")
                        .dot(TimelineDot::new().color(TimelineDotColor::Warning)),
                ),
        );
    }

    fn render_interactive_timeline(&self, ui: &mut egui::Ui) {
        ui.heading("Interactive Timeline");
        ui.label("Click on timeline items to trigger actions");

        ui.add_space(10.0);

        ui.add(
            timeline()
                .position(TimelinePosition::Right)
                .item(
                    TimelineItem::new()
                        .opposite_content("Step 1")
                        .content("🖱️ Click me!")
                        .dot(
                            TimelineDot::new()
                                .color(TimelineDotColor::Primary)
                                .icon("1"),
                        )
                        .on_click(|| {
                            println!("First step clicked!");
                        }),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("Step 2")
                        .content("🖱️ I'm clickable too!")
                        .dot(
                            TimelineDot::new()
                                .color(TimelineDotColor::Info)
                                .icon("2"),
                        )
                        .on_click(|| {
                            println!("Second step clicked!");
                        }),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("Step 3")
                        .content("🖱️ Try clicking all of us!")
                        .dot(
                            TimelineDot::new()
                                .color(TimelineDotColor::Success)
                                .icon("3"),
                        )
                        .on_click(|| {
                            println!("Third step clicked!");
                        }),
                ),
        );

        ui.add_space(10.0);
        ui.label("💡 Tip: Check your console for click events");
    }
}
