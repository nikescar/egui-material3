use eframe::egui::{self, Ui, Window};
use crate::{MaterialButton, MaterialCheckbox, image_list, masonry_image_list, woven_image_list};

pub struct ImageListWindow {
    pub open: bool,
    columns: usize,
    text_protected: bool,
    show_supporting_text: bool,
    item_spacing: f32,
}

impl Default for ImageListWindow {
    fn default() -> Self {
        Self {
            open: false,
            columns: 3,
            text_protected: false,
            show_supporting_text: true,
            item_spacing: 8.0,
        }
    }
}

impl ImageListWindow {
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

                if ui.button("Target").clicked() {
                    let _ = webbrowser::open("https://material-web.dev/components/image-list/stories/");
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
                ui.add(MaterialCheckbox::new(&mut self.text_protected, "Text Protected"));
                ui.add(MaterialCheckbox::new(&mut self.show_supporting_text, "Show Supporting Text"));
            });
        });
    }

    fn render_image_list_examples(&mut self, ui: &mut Ui) {
        ui.heading("Standard Image List");
        ui.label("Images displayed in a regular grid with consistent sizing.");
        
        let standard_list = image_list()
            .columns(self.columns)
            .item_spacing(self.item_spacing)
            .text_protected(self.text_protected)
            .item_with_callback("Sunset Beach", "image1.jpg", || println!("Sunset Beach clicked!"))
            .item_with_callback("Mountain View", "image2.jpg", || println!("Mountain View clicked!"))
            .item_with_callback("City Lights", "image3.jpg", || println!("City Lights clicked!"))
            .item_with_callback("Forest Path", "image4.jpg", || println!("Forest Path clicked!"))
            .item_with_callback("Ocean Waves", "image5.jpg", || println!("Ocean Waves clicked!"))
            .item_with_callback("Desert Dunes", "image6.jpg", || println!("Desert Dunes clicked!"));

        // Add supporting text conditionally
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
            .columns(self.columns)
            .item_spacing(self.item_spacing)
            .text_protected(self.text_protected)
            .item_with_callback("Architecture", "arch1.jpg", || println!("Architecture clicked!"))
            .item_with_callback("Nature", "nature1.jpg", || println!("Nature clicked!"))
            .item_with_callback("Abstract Art", "art1.jpg", || println!("Abstract Art clicked!"))
            .item_with_callback("Street Photo", "street1.jpg", || println!("Street Photo clicked!"))
            .item_with_callback("Portrait", "portrait1.jpg", || println!("Portrait clicked!"))
            .item_with_callback("Landscape", "landscape1.jpg", || println!("Landscape clicked!"));
        
        ui.add(masonry_list);

        ui.add_space(30.0);
        
        ui.heading("Woven Image List");
        ui.label("Images arranged in a woven pattern with varied sizing.");
        
        let woven_list = woven_image_list()
            .columns(self.columns)
            .item_spacing(self.item_spacing)
            .text_protected(self.text_protected)
            .item_with_callback("Texture 1", "texture1.jpg", || println!("Texture 1 clicked!"))
            .item_with_callback("Pattern 2", "pattern2.jpg", || println!("Pattern 2 clicked!"))
            .item_with_callback("Color 3", "color3.jpg", || println!("Color 3 clicked!"))
            .item_with_callback("Design 4", "design4.jpg", || println!("Design 4 clicked!"))
            .item_with_callback("Style 5", "style5.jpg", || println!("Style 5 clicked!"))
            .item_with_callback("Art 6", "art6.jpg", || println!("Art 6 clicked!"));
        
        ui.add(woven_list);

        ui.add_space(30.0);
        
        ui.heading("Interactive Demo");
        
        ui.horizontal(|ui| {
            if ui.add(MaterialButton::filled("Add Image")).clicked() {
                println!("Add image clicked!");
            }
            if ui.add(MaterialButton::outlined("Remove Image")).clicked() {
                println!("Remove image clicked!");
            }
            if ui.add(MaterialButton::text("Select All")).clicked() {
                println!("Select all clicked!");
            }
        });

        ui.add_space(10.0);
        
        let interactive_list = image_list()
            .columns(4)
            .item_spacing(self.item_spacing)
            .text_protected(true)
            .item_with_callback("Photo 001", "photo1.jpg", || println!("Photo 001 selected!"))
            .item_with_callback("Photo 002", "photo2.jpg", || println!("Photo 002 selected!"))
            .item_with_callback("Photo 003", "photo3.jpg", || println!("Photo 003 selected!"))
            .item_with_callback("Photo 004", "photo4.jpg", || println!("Photo 004 selected!"))
            .item_with_callback("Photo 005", "photo5.jpg", || println!("Photo 005 selected!"))
            .item_with_callback("Photo 006", "photo6.jpg", || println!("Photo 006 selected!"))
            .item_with_callback("Photo 007", "photo7.jpg", || println!("Photo 007 selected!"))
            .item_with_callback("Photo 008", "photo8.jpg", || println!("Photo 008 selected!"));
        
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