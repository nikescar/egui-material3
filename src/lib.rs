//! # egui-material3
//!
//! A comprehensive Material Design 3 component library for [egui](https://github.com/emilk/egui),
//! providing a complete set of Material Design components with advanced theming support.
//!
//! ## Features
//!
//! - **Complete Material Design 3 Components**: Buttons, checkboxes, sliders, dialogs, data tables, and more
//! - **Advanced Theming System**: Support for light/dark modes, contrast levels, and custom Material Design themes
//! - **Build-time Theme Inclusion**: Automatically include theme JSON files at compile time for optimal performance
//! - **Runtime Theme Loading**: Load and switch themes dynamically at runtime
//! - **Material Design Icons**: Full support for Material Symbols with built-in icon font loading
//! - **Responsive Design**: Components adapt to different screen sizes and orientations
//!
//! ## Quick Start
//!
//! Add this to your `Cargo.toml`:
//! ```bash
//! $ cargo add egui-material3
//! ```
//!
//! ### Basic Usage
//!
//! ```rust,no_run
//! use eframe::egui;
//! use egui_material3::{
//!     MaterialButton, MaterialCheckbox, MaterialSlider,
//!     theme::{setup_google_fonts, setup_local_fonts, setup_local_theme,
//!            load_fonts, load_themes, update_window_background}
//! };
//!
//! fn main() -> Result<(), eframe::Error> {
//!     let options = eframe::NativeOptions {
//!         viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
//!         ..Default::default()
//!     };
//!     
//!     eframe::run_native(
//!         "Material Design App",
//!         options,
//!         Box::new(|cc| {
//!             // Setup Material Design fonts and themes
//!             setup_google_fonts(Some("Roboto"));
//!             setup_local_fonts(Some("resources/MaterialSymbolsOutlined.ttf"));
//!             setup_local_theme(None); // Use default theme
//!             
//!             // Load fonts and themes
//!             load_fonts(&cc.egui_ctx);
//!             load_themes();
//!             
//!             // Apply theme background
//!             update_window_background(&cc.egui_ctx);
//!             
//!             Ok(Box::<MyApp>::default())
//!         }),
//!     )
//! }
//!
//! #[derive(Default)]
//! struct MyApp {
//!     checked: bool,
//!     slider_value: f32,
//! }
//!
//! impl eframe::App for MyApp {
//!     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//!         egui::CentralPanel::default().show(ctx, |ui| {
//!             ui.heading("Material Design Components");
//!             
//!             // Use Material Design components
//!             ui.add(MaterialButton::new("Click me"));
//!             ui.add(MaterialCheckbox::new(&mut self.checked, "Check me"));
//!             ui.add(MaterialSlider::new(&mut self.slider_value, 0.0..=100.0));
//!         });
//!     }
//! }
//! ```
//!
//! ## Theme System
//!
//! The theme system supports Material Design 3 with comprehensive theming capabilities:
//!
//! ### Build-time Theme Inclusion
//!
//! Themes can be automatically included at build time from JSON files:
//!
//! ```rust,no_run
//! use egui_material3::theme::{setup_local_theme, load_themes};
//!
//! // Uses themes from resources/ and examples/ directories automatically
//! setup_local_theme(None);
//! load_themes();
//! ```
//!
//! ### Runtime Theme Loading
//!
//! Load custom themes dynamically:
//!
//! ```rust,no_run
//! use egui_material3::theme::{setup_local_theme, load_themes};
//!
//! // Load specific theme file
//! setup_local_theme(Some("path/to/my-theme.json"));
//! load_themes();
//! ```
//!
//! ### Theme Modes and Contrast
//!
//! Support for multiple theme modes and contrast levels:
//!
//! ```rust,no_run
//! use egui_material3::theme::{get_global_theme, update_window_background, ThemeMode, ContrastLevel};
//!
//! // Change theme mode at runtime
//! if let Ok(mut theme) = get_global_theme().lock() {
//!     theme.theme_mode = ThemeMode::Dark;
//!     theme.contrast_level = ContrastLevel::High;
//! }
//! // Apply changes
//! update_window_background(ctx);
//! ```
//!
//! ## Available Components
//!
//! ### Basic Components
//! - [`MaterialButton`] - Material Design buttons with multiple variants
//! - [`MaterialCheckbox`] - Checkboxes following Material Design guidelines  
//! - [`MaterialSlider`] - Sliders with Material Design styling
//! - [`MaterialSwitch`] - Toggle switches
//! - [`MaterialRadio`] - Radio button groups
//! - [`MaterialSelect`] - Dropdown selection components
//!
//! ### Advanced Components
//! - [`MaterialChip`] - Filter and action chips
//! - [`MaterialCard2`] - Material Design cards
//! - [`MaterialDialog`] - Modal dialogs and alerts
//! - [`MaterialFab`] - Floating Action Buttons
//! - [`MaterialProgress`] - Progress indicators and loading states
//! - [`MaterialDataTable`] - Data tables with sorting and selection
//!
//! ### Navigation Components  
//! - [`MaterialTabs`] - Tab navigation
//! - [`MaterialDrawer`] - Navigation drawers
//! - [`MaterialTopAppBar`] - App bars and toolbars
//!
//! ### Icons and Visual Elements
//! - [`MaterialIcon`] - Material Design icons with font support
//! - [`MaterialList`] - Lists following Material Design patterns
//! - [`MaterialImageList`] - Image lists with online/offline support and smart caching
//!
//! ## Image Lists and OnDemand Feature
//!
//! The [`MaterialImageList`] component provides comprehensive image display capabilities:
//!
//! ```rust,no_run
//! use egui_material3::image_list;
//!
//! // Local image files
//! ui.add(image_list()
//!     .columns(3)
//!     .item_spacing(8.0)
//!     .items_from_paths(glob::glob("resources/*.png")?));
//!
//! // Online images (requires 'ondemand' feature)
//! ui.add(image_list()
//!     .columns(4)
//!     .item_spacing(8.0)
//!     .items_from_urls(vec![
//!         "https://example.com/image1.jpg".to_string(),
//!         "https://example.com/image2.png".to_string(),
//!     ]));
//!
//! // Embedded images from byte arrays
//! ui.add(image_list()
//!     .columns(2)
//!     .item_spacing(8.0)
//!     .items_from_bytes(vec![
//!         include_bytes!("image1.png").to_vec(),
//!         include_bytes!("image2.png").to_vec(),
//!     ]));
//! ```
//!
//! ### OnDemand Feature
//!
//! Enable the `ondemand` feature for online image support:
//!
//! ```toml
//! [dependencies]
//! egui-material3 = { version = "0.0.6", features = ["ondemand"] }
//! ```
//!
//! Key capabilities:
//! - **Smart caching**: Downloaded images are cached locally with correct file extensions
//! - **Format detection**: Automatically detects PNG, JPEG, GIF, and WebP formats
//! - **Efficient loading**: Images are downloaded once and reused from cache
//! - **Performance optimized**: UI repaints only when new images are available
//! - **Error handling**: Graceful fallback with visual indicators for failed loads
//!
//! ## Examples
//!
//! The crate includes comprehensive examples:
//!
//! - `widget_gallery_example` - Showcase of all Material components with theme switching
//! - `nobel_prizes_example` - Real-world data table implementation
//! - `stories` - Individual component showcase windows for detailed exploration
//! - `package` - Standalone example with bundled resources and themes
//! - `ondemand` - Image list demonstration with online/offline images and smart caching
//!
//! Run examples with:
//! ```bash
//! cargo run --example widget_gallery_example
//! cargo run --example nobel_prizes_example
//! cargo run --example stories
//!
//! # OnDemand example with online image support
//! cd examples/ondemand && cargo run
//!
//! # Package example runs independently with its own Cargo.toml
//! cd examples/package && cargo run
//! ```
//!
//! ## Material Design Resources
//!
//! - [Material Design 3](https://m3.material.io/)
//! - [Material Theme Builder](https://m3.material.io/theme-builder)
//! - [Material Design Icons](https://fonts.google.com/icons)
//!
//! This crate follows the Material Design 3 specifications and guidelines for consistent,
//! accessible, and beautiful user interfaces.

pub mod bottomappbar;
pub mod button;
pub mod card2;
pub mod carousel;
pub mod checkbox;
pub mod chips;
pub mod datatable;
pub mod dialog;
pub mod drawer;
pub mod fab;
pub mod icon;
pub mod iconbutton;
pub mod material_symbol;
pub mod noto_emoji;
pub mod image_utils;
pub mod imagelist;
pub mod layoutgrid;
pub mod list;
pub mod menu;
pub mod progress;
pub mod radio;
pub mod select;
pub mod slider;
pub mod snackbar;
pub mod switch;
pub mod tabs;
pub mod theme;
pub mod topappbar;

pub use {
    bottomappbar::{bottom_app_bar, MaterialBottomAppBar},
    button::{MaterialButton, MaterialButtonVariant},
    card2::{elevated_card2, filled_card2, outlined_card2, Card2Variant, MaterialCard2},
    carousel::{carousel, CarouselItem, MaterialCarousel},
    checkbox::{checkbox, MaterialCheckbox},
    chips::{assist_chip, filter_chip, input_chip, suggestion_chip, ChipVariant, MaterialChip},
    datatable::{
        data_table, CellContent, ColumnWidth, DataTableCell, DataTableColumn, DataTableRow,
        DataTableSource, DataTableState, DataTableTheme, HAlign, MaterialDataTable, RowAction,
        SortDirection, VAlign,
    },
    dialog::{dialog, MaterialDialog},
    drawer::{
        dismissible_drawer, modal_drawer, permanent_drawer, standard_drawer, DrawerAlignment,
        DrawerHeader, DrawerItem, DrawerSection, DrawerThemeData, DrawerVariant, MaterialDrawer,
    },
    egui::TextEdit, // Re-export egui's TextEdit
    fab::{
        fab_branded, fab_primary, fab_secondary, fab_surface, fab_tertiary, google_branded_icon,
        FabSize, FabVariant, MaterialFab, SvgIcon, SvgPath,
    },
    icon::{icon, MaterialIcon},
    iconbutton::{
        icon_button_filled, icon_button_filled_tonal, icon_button_outlined, icon_button_standard,
        icon_button_toggle, IconButtonVariant, MaterialIconButton,
    },
    imagelist::{
        image_list, masonry_image_list, woven_image_list, ImageListItem, ImageListVariant,
        MaterialImageList,
    },
    layoutgrid::{debug_layout_grid, layout_grid, GridTile, GridTileBar, MaterialLayoutGrid},
    list::{list, list_item, ListItem, ListTileStyle, ListTileTitleAlignment, MaterialList, VisualDensity},
    menu::{
        menu, menu_item, Corner, FocusState, MaterialMenu, MenuBarThemeData,
        MenuButtonThemeData, MenuItem, MenuStyle, MenuThemeData, Positioning,
    },
    progress::{circular_progress, linear_progress, MaterialProgress, ProgressVariant},
    radio::{radio, radio_group, radio_list_tile, MaterialRadio, MaterialRadioGroup, RadioListTile, ListTileControlAffinity},
    select::{select, MaterialSelect, SelectVariant, MenuAlignment},
    slider::{slider, range_slider, MaterialSlider, MaterialRangeSlider, RangeValues, SliderInteraction, ThumbShape},
    snackbar::{snackbar, snackbar_with_action, MaterialSnackbar, SnackbarPosition, SnackBarBehavior},
    switch::{switch, MaterialSwitch},
    tabs::{tabs_primary, tabs_secondary, MaterialTabs, TabVariant},
    theme::{
        get_global_color, get_global_theme, update_global_theme, ContrastLevel,
        MaterialThemeContext, MaterialThemeFile, ThemeMode,
    },
    topappbar::{
        center_aligned_top_app_bar, large_top_app_bar, medium_top_app_bar, top_app_bar,
        MaterialTopAppBar, TopAppBarVariant,
    },
};
