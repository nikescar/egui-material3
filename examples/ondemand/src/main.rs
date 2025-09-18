// Minimal egui-material3 example
use eframe::egui;
use egui_material3::{
    MaterialButton, MaterialCheckbox, MaterialSlider, MaterialSwitch,
    theme::{setup_local_fonts, setup_google_fonts, setup_local_theme, load_fonts, load_themes, update_window_background},
    image_list, masonry_image_list, woven_image_list
};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Material Design Ondemand Imagelist Example",
        options,
        Box::new(|cc| {
            // Setup Material Design fonts and themes
            setup_local_theme(Some("resources/material-theme.json")); // Use default theme
            egui_extras::install_image_loaders(&cc.egui_ctx);

            setup_google_fonts(Some("Nanum Gothic"));

            // Load fonts and themes
            load_fonts(&cc.egui_ctx);
            load_themes();
            
            // Apply theme background
            update_window_background(&cc.egui_ctx);
            
            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Default)]
struct MyApp {
    checked: bool,
    slider_value: f32,
    switch_enabled: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // image_list control with offline images on /home/wj/Desktop/work/egui-material3/examples/ondemand/resources/*.png
        let offline_images = glob::glob("/home/wj/Desktop/work/egui-material3/examples/ondemand/resources/*.png");

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {

                ui.heading("Material Design Ondemand Imagelist Example 고");
                ui.add_space(10.0);
                
                // Display image lists
                ui.label("Image List:");
                if let Ok(offline_images) = offline_images {
                    ui.add(image_list()
                        .columns(3)
                        .item_spacing(8.0)
                        .items_from_paths(offline_images.filter_map(|p| p.ok())));
                }

                // image_list control with online images
                ui.add_space(20.0);
                ui.label("Image List with Online Images:");
                ui.add(image_list()
                    .columns(4)
                    .item_spacing(8.0)
                    .items_from_urls(vec![
                        "https://i.imgur.com/Y5Ozbdm.png".to_string(),
                        "https://i.imgur.com/Y5Ozbdm.png".to_string(),
                        "https://i.imgur.com/Y5Ozbdm.png".to_string(),
                        "https://i.imgur.com/Y5Ozbdm.png".to_string(),
                        "https://i.imgur.com/Y5Ozbdm.png".to_string(),
                        "https://i.imgur.com/Y5Ozbdm.png".to_string(),
                        "https://i.imgur.com/Y5Ozbdm.png".to_string(),
                        "https://i.imgur.com/Y5Ozbdm.png".to_string(),
                        "https://i.imgur.com/Y5Ozbdm.png".to_string(),
                        "https://i.imgur.com/Y5Ozbdm.png".to_string(),
                        "https://i.imgur.com/Y5Ozbdm.png".to_string(),
                        "https://i.imgur.com/Y5Ozbdm.png".to_string(),
                    ]));

                // image_list control with embedded images with image bytes in string
                ui.add_space(20.0);
                ui.label("Image List with Embedded Images:");
                let embedded_images = vec![
                    include_bytes!("../resources/320x240_1.png").to_vec(),
                    include_bytes!("../resources/320x240_2.png").to_vec(),
                    include_bytes!("../resources/320x240_3.png").to_vec(),
                    include_bytes!("../resources/320x240_4.png").to_vec(),
                    include_bytes!("../resources/320x240_5.png").to_vec(),
                    include_bytes!("../resources/320x240_6.png").to_vec(),
                    include_bytes!("../resources/320x240_7.png").to_vec(),
                    include_bytes!("../resources/320x240_8.png").to_vec(),
                    include_bytes!("../resources/320x240_9.png").to_vec(),
                ];
                ui.add(image_list()
                    .columns(3)
                    .item_spacing(8.0)
                    .items_from_bytes(embedded_images));
            });
        });

    }
}