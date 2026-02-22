# egui-material3

A Material Design component library for egui, providing Material Design 3 components with theme support.

## Screenshots

<img src="./resources/screenshot.png" alt="Material Design Components" width="600"/>

## Usage

Add this to your `Cargo.toml`:

```bash
$ cargo add egui-material3
```

### Basic Example

```rust
use eframe::egui;
use egui_material3::{
    MaterialButton, MaterialCheckbox, MaterialSlider,
    theme::{setup_google_fonts, setup_local_fonts, setup_local_theme,
           load_fonts, load_themes, update_window_background}
};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Material Design App",
        options,
        Box::new(|cc| {
            // Setup Material Design fonts and themes
            setup_google_fonts(Some("Roboto"));
            setup_local_fonts(Some("resources/MaterialSymbolsOutlined.ttf"));
            setup_local_theme(None); // Use default theme

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
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Material Design Components");

            // Use Material Design components
            ui.add(MaterialButton::new("Click me"));
            ui.add(MaterialCheckbox::new(&mut self.checked, "Check me"));
            ui.add(MaterialSlider::new(&mut self.slider_value, 0.0..=100.0));
        });
    }
}
```

## Theme System

The library provides comprehensive Material Design 3 theming capabilities:

### Build-time Theme Inclusion

Themes are automatically included from JSON files during compilation:

```rust
use egui_material3::theme::{setup_local_theme, load_themes};

// Uses themes from resources/ and examples/ directories automatically
setup_local_theme(None);
load_themes();
```

### Runtime Theme Loading

Load custom themes dynamically:

```rust
use egui_material3::theme::{setup_local_theme, load_themes};

// Load specific theme file
setup_local_theme(Some("path/to/my-theme.json"));
load_themes();
```

### Theme Modes and Contrast Levels

Change theme appearance at runtime:

```rust
use egui_material3::theme::{get_global_theme, update_window_background, ThemeMode, ContrastLevel};

// Change theme mode at runtime
if let Ok(mut theme) = get_global_theme().lock() {
    theme.theme_mode = ThemeMode::Dark;
    theme.contrast_level = ContrastLevel::High;
}
// Apply changes
update_window_background(ctx);
```

## Available Components

### Basic Components

- **MaterialButton** - Material Design buttons with multiple variants (filled, outlined, text, elevated, tonal)
- **MaterialCheckbox** - Checkboxes following Material Design guidelines
- **MaterialSlider** / **MaterialRangeSlider** - Sliders with Material Design styling
- **MaterialSwitch** - Toggle switches
- **MaterialRadio** / **MaterialRadioGroup** - Radio button groups with list tile support
- **MaterialSelect** - Dropdown selection components with menu alignment options

### Advanced Components

- **MaterialChip** - Filter, assist, input, and suggestion chips
- **MaterialCard2** - Material Design cards (elevated, filled, outlined variants)
- **MaterialDialog** - Modal dialogs and alerts
- **MaterialFab** - Floating Action Buttons (primary, secondary, tertiary, surface, branded)
- **MaterialProgress** - Progress indicators (circular and linear)
- **MaterialDataTable** - Data tables with sorting, selection, and custom cell content
- **MaterialCarousel** - Carousel for displaying items in a scrollable view
- **MaterialSnackbar** - Toast notifications with optional actions

### Navigation Components

- **MaterialTabs** - Tab navigation (primary and secondary variants)
- **MaterialDrawer** - Navigation drawers (permanent, dismissible, modal, standard)
- **MaterialTopAppBar** - App bars and toolbars (standard, center-aligned, medium, large)

### Visual Elements

- **MaterialIcon** - Material Design icons with font support
- **MaterialSymbol** - Material Symbols rendering
- **MaterialList** - Lists following Material Design patterns with visual density control
- **MaterialImageList** - Image lists with online/offline support and smart caching (standard, masonry, woven variants)
- **MaterialLayoutGrid** - Grid layout with tile bars
- **MaterialIconButton** - Icon buttons (standard, filled, filled tonal, outlined, toggle)
- **MaterialMenu** - Context menus and menu items

### Emoji Collections (svg_emoji feature)

When the `svg_emoji` feature is enabled, you get access to comprehensive SVG emoji and icon collections:

- **Solar Icons** (~1200 icons): UI/UX icon set with variants
- **Noto Emoji** (~3600 emoji): Google's emoji collection with skin tone and gender variants
- **Twemoji** (~3700 emoji): Twitter's emoji collection

```toml
[dependencies]
egui-material3 = { version = "0.0.7", features = ["svg_emoji"] }
```

```rust
use egui_material3::svg_emoji::{SOLAR_ICONS, NOTO_EMOJIS, TWEMOJI};

// Get a Solar icon
if let Some(svg) = SOLAR_ICONS.get("home") {
    // Use the SVG data
}

// Get a Noto emoji (filename without .svg suffix)
if let Some(svg) = NOTO_EMOJIS.get("emoji_u1f600") {
    // Grinning face emoji
}

// Get a Twemoji (Unicode codepoint)
if let Some(svg) = TWEMOJI.get("1f600") {
    // Grinning face emoji
}
```

## Features

### OnDemand Feature

Enable online image support for `MaterialImageList`:

```toml
[dependencies]
egui-material3 = { version = "0.0.7", features = ["ondemand"] }
```

The `MaterialImageList` component supports multiple image sources:

```rust
use egui_material3::image_list;

// Local image files
ui.add(image_list()
    .columns(3)
    .item_spacing(8.0)
    .items_from_paths(glob::glob("resources/*.png")?));

// Online images (requires 'ondemand' feature)
ui.add(image_list()
    .columns(4)
    .item_spacing(8.0)
    .items_from_urls(vec![
        "https://example.com/image1.jpg".to_string(),
        "https://example.com/image2.png".to_string(),
    ]));

// Embedded images from byte arrays
ui.add(image_list()
    .columns(2)
    .item_spacing(8.0)
    .items_from_bytes(vec![
        include_bytes!("image1.png").to_vec(),
        include_bytes!("image2.png").to_vec(),
    ]));
```

Key capabilities:
- **Smart caching**: Downloaded images cached locally with correct file extensions
- **Format detection**: Automatically detects PNG, JPEG, GIF, and WebP formats
- **Efficient loading**: Images downloaded once and reused from cache
- **Performance optimized**: UI repaints only when new images available
- **Error handling**: Graceful fallback with visual indicators for failed loads

### Spreadsheet Feature

Enable spreadsheet components with DuckDB backend:

```toml
[dependencies]
egui-material3 = { version = "0.0.7", features = ["spreadsheet"] }
```

The spreadsheet feature provides:

- **MaterialSpreadsheet** - Full-featured spreadsheet widget with DuckDB backend
- **Column types**: Text, Integer, Real, Boolean
- **File formats**: Import/export CSV, Excel (xls/xlsx), Parquet formats
- **Async loading**: Background data loading with progress indicators
- **Data manipulation**: Full SQL query support via DuckDB

```rust
use egui_material3::{MaterialSpreadsheet, SpreadsheetDataModel, ColumnDef, ColumnType};

// Create spreadsheet with column definitions
let columns = vec![
    ColumnDef { name: "Name".to_string(), col_type: ColumnType::Text, width: 150.0 },
    ColumnDef { name: "Age".to_string(), col_type: ColumnType::Integer, width: 80.0 },
    ColumnDef { name: "Score".to_string(), col_type: ColumnType::Real, width: 100.0 },
];

let mut model = SpreadsheetDataModel::new("my_table", columns)?;

// Import data from CSV/Excel/Parquet
model.import_file("data.csv", FileFormat::Csv)?;

// Display in UI
ui.add(MaterialSpreadsheet::new(&mut model));
```

## Examples

The crate includes comprehensive examples:

```bash
# Showcase of all Material components with theme switching
cargo run --example widget_gallery_example

# Real-world data table implementation
cargo run --example nobel_prizes_example

# Individual component showcase windows
cargo run --example stories

# SVG icon demonstration (shows Solar icons embedded in FAB)
cargo run --example svg_icon_demo

# OnDemand example with online image support
cd examples/ondemand && cargo run

# Standalone package example with bundled resources
cd examples/package && cargo run
```

Example structure:
- `widget_gallery_example.rs` - Complete showcase with theme controls
- `nobel_prizes_example.rs` - Data table with real Nobel Prize data
- `stories/` - Individual component demos (button, card, dialog, drawer, fab, etc.)
- `ondemand/` - Standalone crate demonstrating online image loading
- `package/` - Standalone deployable example with all resources

<details markdown>
<summary> Todos </summary>

## Todos

* svg sprite

</details>
