#![doc(hidden)]

use crate::{image_list, masonry_image_list, woven_image_list, MaterialButton, MaterialCheckbox};
use eframe::egui::{self, Ui, Window};

#[doc(hidden)]
pub struct ImageListWindow {
    pub open: bool,
    columns: usize,
    text_protected: bool,
    show_supporting_text: bool,
    item_spacing: f32,
    // Dynamic image list for interactive demo
    dynamic_images: Vec<DynamicImageItem>,
    next_image_id: usize,
}

#[derive(Clone)]
struct DynamicImageItem {
    _id: usize,
    label: String,
    image_source: String,
}

impl Default for ImageListWindow {
    fn default() -> Self {
        // Initialize with some default images
        let mut dynamic_images = Vec::new();
        for i in 1..=8 {
            dynamic_images.push(DynamicImageItem {
                _id: i,
                label: format!("Photo {:03}", i),
                image_source: "resources/320x240.png".to_string(),
            });
        }

        Self {
            open: false,
            columns: 3,
            text_protected: false,
            show_supporting_text: true,
            item_spacing: 8.0,
            dynamic_images,
            next_image_id: 9,
        }
    }
}

impl ImageListWindow {
    fn add_image(&mut self) {
        let new_image = DynamicImageItem {
            _id: self.next_image_id,
            label: format!("Photo {:03}", self.next_image_id),
            image_source: "resources/320x240.png".to_string(),
        };
        self.dynamic_images.push(new_image);
        self.next_image_id += 1;
    }

    fn remove_image(&mut self) {
        if !self.dynamic_images.is_empty() {
            self.dynamic_images.pop();
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        Window::new("Image List Stories")
            .open(&mut open)
            .default_size([1000.0, 700.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_controls(ui);
                    ui.add_space(20.0);
                    self.render_image_list_examples(ui);
                });
            });
        self.open = open;
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.push_id("imagelist_controls", |ui| {
            ui.horizontal(|ui| {
                ui.heading("Image List Controls");

                if ui.add(MaterialButton::filled("Target").small()).clicked() {
                    let _ =
                        webbrowser::open("https://material-web.dev/components/image-list/stories/");
                }
            });

            ui.horizontal(|ui| {
                ui.label("Columns:");
                ui.add(egui::Slider::new(&mut self.columns, 1..=6));
            });

            ui.horizontal(|ui| {
                ui.label("Item Spacing:");
                ui.add(egui::Slider::new(&mut self.item_spacing, 0.0..=20.0).suffix("px"));
            });

            ui.horizontal(|ui| {
                ui.add(MaterialCheckbox::new(
                    &mut self.text_protected,
                    "Text Protected",
                ));
                ui.add(MaterialCheckbox::new(
                    &mut self.show_supporting_text,
                    "Show Supporting Text",
                ));
            });
        });
    }

    fn render_image_list_examples(&mut self, ui: &mut Ui) {
        ui.heading("Standard Image List");
        ui.label("Images displayed in a regular grid with consistent sizing.");

        let standard_list = image_list()
            .id_salt("standard_imagelist")
            .columns(self.columns)
            .item_spacing(self.item_spacing)
            .text_protected(self.text_protected)
            .item_with_callback("Architecture", "resources/320x240.png", || {
                println!("Architecture clicked!")
            })
            .item_with_callback("Nature", "resources/320x240.png", || {
                println!("Nature clicked!")
            })
            .item_with_callback("Abstract Art", "resources/320x240.png", || {
                println!("Abstract Art clicked!")
            })
            .item_with_callback("Street Photo", "resources/320x240.png", || {
                println!("Street Photo clicked!")
            })
            .item_with_callback("Portrait", "resources/320x240.png", || {
                println!("Portrait clicked!")
            })
            .item_with_callback("Landscape", "resources/320x240.png", || {
                println!("Landscape clicked!")
            });

        if self.show_supporting_text {
            // Note: This is a simplified example. In a real implementation,
            // you'd need to recreate the list with supporting text items
            ui.label("(Supporting text would be shown here in a full implementation)");
        }

        ui.add(standard_list);

        ui.add_space(30.0);

        ui.heading("Masonry Image List");
        ui.label("Images with varying heights creating a masonry layout.");

        let masonry_list = masonry_image_list()
            .id_salt("masonry_imagelist")
            .columns(self.columns)
            .item_spacing(self.item_spacing)
            .text_protected(self.text_protected)
            .item_with_callback("Architecture", "https://i.imgur.com/Y5Ozbdm.png", || {
                println!("Architecture clicked!")
            })
            .item_with_callback("Nature", "https://i.imgur.com/Y5Ozbdm.png", || {
                println!("Nature clicked!")
            })
            .item_with_callback("Abstract Art", "https://i.imgur.com/Y5Ozbdm.png", || {
                println!("Abstract Art clicked!")
            })
            .item_with_callback("Street Photo", "https://i.imgur.com/Y5Ozbdm.png", || {
                println!("Street Photo clicked!")
            })
            .item_with_callback("Portrait", "https://i.imgur.com/Y5Ozbdm.png", || {
                println!("Portrait clicked!")
            })
            .item_with_callback("Landscape", "https://i.imgur.com/Y5Ozbdm.png", || {
                println!("Landscape clicked!")
            });

        ui.add(masonry_list);

        ui.add_space(30.0);

        ui.heading("Woven Image List");
        ui.label("Images arranged in a woven pattern with varied sizing.");

        let woven_list = woven_image_list()
            .id_salt("woven_imagelist")
            .columns(self.columns)
            .item_spacing(self.item_spacing)
            .text_protected(self.text_protected)
            .item_with_callback("Texture 1", "https://i.imgur.com/Y5Ozbdm.png", || {
                println!("Texture 1 clicked!")
            })
            .item_with_callback("Pattern 2", "https://i.imgur.com/Y5Ozbdm.png", || {
                println!("Pattern 2 clicked!")
            })
            .item_with_callback("Color 3", "https://i.imgur.com/Y5Ozbdm.png", || {
                println!("Color 3 clicked!")
            })
            .item_with_callback("Design 4", "https://i.imgur.com/Y5Ozbdm.png", || {
                println!("Design 4 clicked!")
            })
            .item_with_callback("Style 5", "https://i.imgur.com/Y5Ozbdm.png", || {
                println!("Style 5 clicked!")
            })
            .item_with_callback("Art 6", "https://i.imgur.com/Y5Ozbdm.png", || {
                println!("Art 6 clicked!")
            });

        ui.add(woven_list);

        ui.add_space(30.0);

        ui.heading("Interactive Demo");

        ui.horizontal(|ui| {
            if ui.add(MaterialButton::filled("Add Image")).clicked() {
                self.add_image();
            }
            if ui.add(MaterialButton::outlined("Remove Image")).clicked() {
                self.remove_image();
            }
            if ui.add(MaterialButton::text("Select All")).clicked() {
                println!("Select all clicked!");
            }
        });

        ui.add_space(10.0);

        let mut interactive_list = image_list()
            .id_salt("interactive_imagelist")
            .columns(4)
            .item_spacing(self.item_spacing)
            .text_protected(true);

        // Add dynamic images from vector
        for image in &self.dynamic_images {
            let label = image.label.clone();
            let image_source = image.image_source.clone();
            interactive_list =
                interactive_list.item_with_callback(label.clone(), image_source, move || {
                    println!("{} selected!", label)
                });
        }

        ui.add(interactive_list);

        ui.add_space(20.0);

        ui.heading("Usage Examples");

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Standard:");
                ui.label("• Photo galleries");
                ui.label("• Product catalogs");
                ui.label("• Icon collections");
            });

            ui.vertical(|ui| {
                ui.label("Masonry:");
                ui.label("• Pinterest-style layouts");
                ui.label("• Varied content heights");
                ui.label("• Creative portfolios");
            });

            ui.vertical(|ui| {
                ui.label("Woven:");
                ui.label("• Design showcases");
                ui.label("• Mixed media content");
                ui.label("• Artistic layouts");
            });
        });
    }
}
