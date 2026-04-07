#![doc(hidden)]

use crate::{timeline, MaterialButton, MaterialCard2, TimelineDot, TimelineDotColor, TimelineDotVariant, TimelineItem, TimelinePosition};
use eframe::egui::{self, Color32, Window};

#[doc(hidden)]
pub struct TimelineWindow {
    pub open: bool,
    position_selection: TimelinePosition,
    sample_textures: Vec<egui::TextureHandle>,
}

impl Default for TimelineWindow {
    fn default() -> Self {
        Self {
            open: false,
            position_selection: TimelinePosition::Right,
            sample_textures: Vec::new(),
        }
    }
}

impl TimelineWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        // Create sample textures if not already created
        if self.sample_textures.is_empty() {
            self.sample_textures = Self::create_sample_textures(ctx);
        }

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
                    ui.add_space(20.0);
                    self.render_timeline_with_buttons(ui);
                    ui.add_space(20.0);
                    self.render_timeline_with_cards(ui);
                    ui.add_space(20.0);
                    self.render_timeline_with_texture_icons(ui);
                });
            });
        self.open = open;
    }

    /// Create sample textures for demonstration
    fn create_sample_textures(ctx: &egui::Context) -> Vec<egui::TextureHandle> {
        let mut textures = Vec::new();

        // Texture 1: Red gradient circle (YouTube-style)
        textures.push(Self::create_circle_texture(ctx, "icon_1", egui::Color32::from_rgb(255, 0, 0), egui::Color32::from_rgb(200, 0, 0)));

        // Texture 2: Blue/Green gradient circle (Chrome-style)
        textures.push(Self::create_circle_texture(ctx, "icon_2", egui::Color32::from_rgb(66, 133, 244), egui::Color32::from_rgb(52, 168, 83)));

        // Texture 3: Green gradient circle (Spotify-style)
        textures.push(Self::create_circle_texture(ctx, "icon_3", egui::Color32::from_rgb(30, 215, 96), egui::Color32::from_rgb(25, 180, 80)));

        // Texture 4: Orange/Yellow gradient square (Gmail-style)
        textures.push(Self::create_square_texture(ctx, "icon_4", egui::Color32::from_rgb(234, 67, 53), egui::Color32::from_rgb(251, 188, 5)));

        textures
    }

    /// Create a gradient circle texture
    fn create_circle_texture(ctx: &egui::Context, name: &str, color1: egui::Color32, color2: egui::Color32) -> egui::TextureHandle {
        let size = 48;
        let mut pixels = vec![egui::Color32::TRANSPARENT; size * size];

        let center = size as f32 / 2.0;
        let radius = center - 2.0;

        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 - center;
                let dy = y as f32 - center;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist <= radius {
                    // Create a gradient from center to edge
                    let ratio = dist / radius;
                    let r = (color1.r() as f32 * (1.0 - ratio) + color2.r() as f32 * ratio) as u8;
                    let g = (color1.g() as f32 * (1.0 - ratio) + color2.g() as f32 * ratio) as u8;
                    let b = (color1.b() as f32 * (1.0 - ratio) + color2.b() as f32 * ratio) as u8;
                    pixels[y * size + x] = egui::Color32::from_rgb(r, g, b);
                }
            }
        }

        let color_image = egui::ColorImage::from_rgba_unmultiplied([size, size], &{
            let mut bytes = Vec::with_capacity(size * size * 4);
            for pixel in &pixels {
                bytes.extend_from_slice(&pixel.to_array());
            }
            bytes
        });

        ctx.load_texture(name, color_image, egui::TextureOptions::LINEAR)
    }

    /// Create a gradient square texture with rounded corners
    fn create_square_texture(ctx: &egui::Context, name: &str, color1: egui::Color32, color2: egui::Color32) -> egui::TextureHandle {
        let size = 48;
        let mut pixels = vec![egui::Color32::TRANSPARENT; size * size];

        let corner_radius = 8.0;

        for y in 0..size {
            for x in 0..size {
                let fx = x as f32;
                let fy = y as f32;

                // Check if inside rounded rectangle
                let mut inside = true;

                // Top-left corner
                if fx < corner_radius && fy < corner_radius {
                    let dx = fx - corner_radius;
                    let dy = fy - corner_radius;
                    if dx * dx + dy * dy > corner_radius * corner_radius {
                        inside = false;
                    }
                }
                // Top-right corner
                else if fx > size as f32 - corner_radius && fy < corner_radius {
                    let dx = fx - (size as f32 - corner_radius);
                    let dy = fy - corner_radius;
                    if dx * dx + dy * dy > corner_radius * corner_radius {
                        inside = false;
                    }
                }
                // Bottom-left corner
                else if fx < corner_radius && fy > size as f32 - corner_radius {
                    let dx = fx - corner_radius;
                    let dy = fy - (size as f32 - corner_radius);
                    if dx * dx + dy * dy > corner_radius * corner_radius {
                        inside = false;
                    }
                }
                // Bottom-right corner
                else if fx > size as f32 - corner_radius && fy > size as f32 - corner_radius {
                    let dx = fx - (size as f32 - corner_radius);
                    let dy = fy - (size as f32 - corner_radius);
                    if dx * dx + dy * dy > corner_radius * corner_radius {
                        inside = false;
                    }
                }

                if inside {
                    // Create a gradient from left to right
                    let ratio = fx / size as f32;
                    let r = (color1.r() as f32 * (1.0 - ratio) + color2.r() as f32 * ratio) as u8;
                    let g = (color1.g() as f32 * (1.0 - ratio) + color2.g() as f32 * ratio) as u8;
                    let b = (color1.b() as f32 * (1.0 - ratio) + color2.b() as f32 * ratio) as u8;
                    pixels[y * size + x] = egui::Color32::from_rgb(r, g, b);
                }
            }
        }

        let color_image = egui::ColorImage::from_rgba_unmultiplied([size, size], &{
            let mut bytes = Vec::with_capacity(size * size * 4);
            for pixel in &pixels {
                bytes.extend_from_slice(&pixel.to_array());
            }
            bytes
        });

        ctx.load_texture(name, color_image, egui::TextureOptions::LINEAR)
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

    fn render_timeline_with_buttons(&self, ui: &mut egui::Ui) {
        ui.heading("Timeline with Action Buttons");
        ui.label("Timeline items with small action buttons for interactive content");

        ui.add_space(10.0);

        ui.add(
            timeline()
                .position(TimelinePosition::Right)
                .item(
                    TimelineItem::new()
                        .opposite_content("2024-03-10")
                        .content_custom(|ui| {
                            ui.label("Task Created: Review Pull Request");
                            ui.add_space(5.0);
                            ui.horizontal(|ui| {
                                if ui.add(MaterialButton::filled("Approve").small()).clicked() {
                                    println!("Approved!");
                                }
                                if ui.add(MaterialButton::outlined("Comment").small()).clicked() {
                                    println!("Adding comment...");
                                }
                                if ui.add(MaterialButton::text("Dismiss").small()).clicked() {
                                    println!("Dismissed");
                                }
                            });
                        })
                        .min_height(80.0)
                        .dot(TimelineDot::new().color(TimelineDotColor::Primary).icon("📝").size(48.0)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("2024-03-11")
                        .content_custom(|ui| {
                            ui.label("Meeting Invitation: Sprint Planning");
                            ui.add_space(5.0);
                            ui.horizontal(|ui| {
                                if ui.add(MaterialButton::filled("Accept").small()).clicked() {
                                    println!("Meeting accepted!");
                                }
                                if ui.add(MaterialButton::outlined("Tentative").small()).clicked() {
                                    println!("Marked as tentative");
                                }
                                if ui.add(MaterialButton::text("Decline").small()).clicked() {
                                    println!("Meeting declined");
                                }
                            });
                        })
                        .min_height(80.0)
                        .dot(TimelineDot::new().color(TimelineDotColor::Info).icon("📅").size(48.0)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("2024-03-12")
                        .content_custom(|ui| {
                            ui.label("Deploy Request: Version 2.0.1");
                            ui.add_space(5.0);
                            ui.horizontal(|ui| {
                                if ui.add(MaterialButton::filled("Deploy").small()).clicked() {
                                    println!("Deploying...");
                                }
                                if ui.add(MaterialButton::outlined("Rollback").small()).clicked() {
                                    println!("Rolling back...");
                                }
                                if ui.add(MaterialButton::text("Details").small()).clicked() {
                                    println!("Showing details...");
                                }
                            });
                        })
                        .min_height(80.0)
                        .dot(TimelineDot::new().color(TimelineDotColor::Success).icon("🚀").size(48.0)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("2024-03-13")
                        .content_custom(|ui| {
                            ui.label("Alert: High CPU Usage Detected");
                            ui.add_space(5.0);
                            ui.horizontal(|ui| {
                                if ui.add(MaterialButton::filled("Investigate").small()).clicked() {
                                    println!("Opening monitoring dashboard...");
                                }
                                if ui.add(MaterialButton::outlined("Acknowledge").small()).clicked() {
                                    println!("Alert acknowledged");
                                }
                                if ui.add(MaterialButton::text("Snooze").small()).clicked() {
                                    println!("Alert snoozed");
                                }
                            });
                        })
                        .min_height(80.0)
                        .dot(TimelineDot::new().color(TimelineDotColor::Warning).icon("⚠️").size(48.0)),
                ),
        );
    }

    fn render_timeline_with_cards(&self, ui: &mut egui::Ui) {
        ui.heading("Timeline with Cards");
        ui.label("Timeline items using Material Design cards for rich content");

        ui.add_space(10.0);

        ui.add(
            timeline()
                .position(TimelinePosition::Alternate)
                .item(
                    TimelineItem::new()
                        .opposite_content("March 10, 2024")
                        .content_custom(|ui| {
                            let available_width = ui.available_width();
                            ui.add(
                                MaterialCard2::elevated()
                                    .header("Feature Released", None::<String>)
                                    .content(|ui| {
                                        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                                            ui.add(egui::Label::new("New dark mode theme is now available!").wrap_mode(egui::TextWrapMode::Wrap));
                                            ui.add_space(5.0);
                                            ui.add(egui::Label::new("Users can now switch between light and dark themes in settings.").wrap_mode(egui::TextWrapMode::Wrap));
                                            ui.add_space(8.0);
                                            ui.horizontal(|ui| {
                                                if ui.add(MaterialButton::outlined("Learn More").small()).clicked() {
                                                    println!("Opening documentation...");
                                                }
                                                if ui.add(MaterialButton::outlined("Share").small()).clicked() {
                                                    println!("Sharing...");
                                                }
                                            });
                                        });
                                    })
                                    .min_size(egui::Vec2::new(available_width.min(280.0), 0.0)),
                            );
                        })
                        .min_height(180.0)
                        .dot(TimelineDot::new().color(TimelineDotColor::Primary).icon("✨").size(48.0)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("March 11, 2024")
                        .content_custom(|ui| {
                            let available_width = ui.available_width();
                            ui.add(
                                MaterialCard2::filled()
                                    .header("Bug Fix", None::<String>)
                                    .content(|ui| {
                                        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                                            ui.add(egui::Label::new("Fixed issue with notification sounds").wrap_mode(egui::TextWrapMode::Wrap));
                                            ui.add_space(5.0);
                                            ui.add(egui::Label::new("Sound notifications now work correctly across all platforms.").wrap_mode(egui::TextWrapMode::Wrap));
                                            ui.add_space(8.0);
                                            ui.horizontal(|ui| {
                                                if ui.add(MaterialButton::outlined("View PR").small()).clicked() {
                                                    println!("Opening pull request...");
                                                }
                                                if ui.add(MaterialButton::outlined("Changelog").small()).clicked() {
                                                    println!("Opening changelog...");
                                                }
                                            });
                                        });
                                    })
                                    .min_size(egui::Vec2::new(available_width.min(280.0), 0.0)),
                            );
                        })
                        .min_height(180.0)
                        .dot(TimelineDot::new().color(TimelineDotColor::Success).icon("🐛").size(48.0)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("March 12, 2024")
                        .content_custom(|ui| {
                            let available_width = ui.available_width();
                            ui.add(
                                MaterialCard2::outlined()
                                    .header("Performance Update", None::<String>)
                                    .content(|ui| {
                                        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                                            ui.add(egui::Label::new("App now loads 50% faster!").wrap_mode(egui::TextWrapMode::Wrap));
                                            ui.add_space(5.0);
                                            ui.add(egui::Label::new("Optimized rendering pipeline and reduced bundle size.").wrap_mode(egui::TextWrapMode::Wrap));
                                            ui.add_space(8.0);
                                            ui.horizontal(|ui| {
                                                if ui.add(MaterialButton::outlined("Benchmarks").small()).clicked() {
                                                    println!("Showing benchmarks...");
                                                }
                                                if ui.add(MaterialButton::outlined("Details").small()).clicked() {
                                                    println!("Showing technical details...");
                                                }
                                            });
                                        });
                                    })
                                    .min_size(egui::Vec2::new(available_width.min(280.0), 0.0)),
                            );
                        })
                        .min_height(180.0)
                        .dot(TimelineDot::new().color(TimelineDotColor::Info).icon("⚡").size(48.0)),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("March 13, 2024")
                        .content_custom(|ui| {
                            let available_width = ui.available_width();
                            ui.add(
                                MaterialCard2::elevated()
                                    .header("Announcement", None::<String>)
                                    .content(|ui| {
                                        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                                            ui.add(egui::Label::new("Scheduled maintenance tomorrow").wrap_mode(egui::TextWrapMode::Wrap));
                                            ui.add_space(5.0);
                                            ui.add(egui::Label::new("System will be down from 2 AM to 4 AM UTC for upgrades.").wrap_mode(egui::TextWrapMode::Wrap));
                                            ui.add_space(8.0);
                                            ui.horizontal(|ui| {
                                                if ui.add(MaterialButton::outlined("Subscribe").small()).clicked() {
                                                    println!("Subscribed to updates");
                                                }
                                                if ui.add(MaterialButton::outlined("Remind Me").small()).clicked() {
                                                    println!("Setting reminder...");
                                                }
                                            });
                                        });
                                    })
                                    .min_size(egui::Vec2::new(available_width.min(280.0), 0.0)),
                            );
                        })
                        .min_height(180.0)
                        .dot(TimelineDot::new().color(TimelineDotColor::Warning).icon("🔧").size(48.0)),
                ),
        );
    }

    fn render_timeline_with_texture_icons(&self, ui: &mut egui::Ui) {
        ui.heading("Timeline with Texture Icons");
        ui.label("Timeline items using egui texture icons instead of emoji/text icons");

        ui.add_space(10.0);

        ui.add(
            timeline()
                .position(TimelinePosition::Right)
                .item(
                    TimelineItem::new()
                        .opposite_content("09:00 AM")
                        .content("App Launched: YouTube")
                        .min_height(60.0)
                        .dot(
                            if let Some(tex) = self.sample_textures.get(0) {
                                TimelineDot::new()
                                    .color(TimelineDotColor::Primary)
                                    .texture_icon(tex.id())
                                    .size(48.0)
                            } else {
                                TimelineDot::new()
                                    .color(TimelineDotColor::Primary)
                                    .icon("📱")
                                    .size(48.0)
                            }
                        ),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("09:15 AM")
                        .content("App Launched: Chrome Browser")
                        .min_height(60.0)
                        .dot(
                            if let Some(tex) = self.sample_textures.get(1) {
                                TimelineDot::new()
                                    .color(TimelineDotColor::Success)
                                    .texture_icon(tex.id())
                                    .size(48.0)
                            } else {
                                TimelineDot::new()
                                    .color(TimelineDotColor::Success)
                                    .icon("🌐")
                                    .size(48.0)
                            }
                        ),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("09:30 AM")
                        .content("App Launched: Spotify")
                        .min_height(60.0)
                        .dot(
                            if let Some(tex) = self.sample_textures.get(2) {
                                TimelineDot::new()
                                    .color(TimelineDotColor::Info)
                                    .texture_icon(tex.id())
                                    .size(48.0)
                            } else {
                                TimelineDot::new()
                                    .color(TimelineDotColor::Info)
                                    .icon("🎵")
                                    .size(48.0)
                            }
                        ),
                )
                .item(
                    TimelineItem::new()
                        .opposite_content("10:00 AM")
                        .content_custom(|ui| {
                            ui.label("App Launched: Gmail");
                            ui.add_space(5.0);
                            ui.group(|ui| {
                                ui.label(egui::RichText::new("Usage Stats").small().strong());
                                ui.horizontal(|ui| {
                                    ui.label("🚀 Launches: 15");
                                    ui.label("⏱️ Used: 2:30:15");
                                    ui.label("👁️ Visible: 2:45:30");
                                });
                            });
                        })
                        .min_height(100.0)
                        .dot(
                            if let Some(tex) = self.sample_textures.get(3) {
                                TimelineDot::new()
                                    .color(TimelineDotColor::Warning)
                                    .texture_icon(tex.id())
                                    .size(48.0)
                            } else {
                                TimelineDot::new()
                                    .color(TimelineDotColor::Warning)
                                    .icon("📧")
                                    .size(48.0)
                            }
                        ),
                ),
        );

        ui.add_space(10.0);
        ui.label("💡 Texture icons are loaded from egui::TextureHandle and displayed in timeline dots");
        ui.label("💡 Different textures shown: Red circle (YouTube), Blue-Green circle (Chrome), Green circle (Spotify), Orange-Yellow square (Gmail)");
        ui.label("💡 Falls back to emoji/text icons if texture is not available");
        ui.label("💡 Icon size can be customized with .size() method (48.0 shown here)");
    }
}
