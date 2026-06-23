# nala-egui-material3

A [Nala](https://github.com/brave-experiments) fork of [egui-material3](https://github.com/nikescar/egui-material3) — Material Design components for [egui](https://github.com/emilk/egui), extended with **Leo / Nala design tokens** and component styling from the [Leo component library](https://github.com/brave/leo).

This crate keeps the full upstream component set while aligning key controls with Nala’s visual language: color tokens, typography, buttons, toggles, checkboxes, and navigation.

## Fork highlights

| Area | What changed |
|------|----------------|
| **Theme** | Default theme is `resources/nala-material-theme.json`, derived from Leo universal color tokens |
| **Typography** | Inter (body), Poppins (display), Roboto Mono — via `setup_nala_fonts()` / `apply_nala_text_styles()` |
| **Buttons** | Nala kinds: `filled`, `outline`, `plain`, `plain-faint`, `hero` + sizes `tiny` → `jumbo` |
| **Switch** | Leo toggle styling (pill track, two sizes, animated thumb) |
| **Checkbox** | Leo SVG icons, normal/small sizes, interactive colors |
| **Navigation** | New `MaterialNavigation` sidebar matching Leo `Navigation` + nested items |

Upstream components (drawer, tabs, data table, chips, etc.) are unchanged unless noted in stories.

## Screenshots

<img src="./resources/screenshot.png" alt="Material Design Components" width="600"/>

## Installation

### From this fork (recommended)

```toml
[dependencies]
egui-material3 = { git = "https://github.com/brave-experiments/nala-egui-material3.git", branch = "main" }

# Optional: download Nala fonts from Google Fonts at runtime
# egui-material3 = { git = "...", features = ["ondemand"] }
```

After updating the dependency:

```bash
cargo update -p egui-material3
cargo build
```

### From crates.io (upstream)

The crates.io package is the original upstream library and does **not** include these Nala customizations:

```bash
cargo add egui-material3
```

## Usage

### Nala quick start

Initialize the Nala theme and fonts in your app startup (same pattern as the `stories` example):

```rust
use eframe::egui;
use egui_material3::{
    MaterialButton, MaterialButtonSize, MaterialCheckbox, MaterialNavigation,
    MaterialSwitch, NavigationHeader, NavigationItem, navigation,
    theme::{
        apply_nala_text_styles, load_fonts, load_themes, setup_local_fonts_from_bytes,
        setup_local_theme, setup_nala_fonts, update_window_background,
    },
};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Nala App",
        options,
        Box::new(|cc| {
            // Material Symbols (icons)
            setup_local_fonts_from_bytes(
                "MaterialSymbolsOutlined",
                include_bytes!("resources/MaterialSymbolsOutlined.ttf"),
            );

            // Nala fonts + theme (see "Fonts" below)
            setup_nala_fonts();
            setup_local_theme(None); // loads bundled nala-material-theme.json

            load_fonts(&cc.egui_ctx);
            apply_nala_text_styles(&cc.egui_ctx);
            load_themes();
            update_window_background(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Default)]
struct MyApp {
    checked: bool,
    switch_on: bool,
    nav_selected: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("nav").show(ctx, |ui| {
            navigation(&mut self.nav_selected)
                .header(NavigationHeader {
                    highlight: Some("Nala".into()),
                    title: "App".into(),
                    icon: Some("shield".into()),
                    ..Default::default()
                })
                .item(NavigationItem::new("home", "Home").icon("home"))
                .item(NavigationItem::new("settings", "Settings").icon("settings"))
                .show(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Nala components");

            ui.horizontal(|ui| {
                ui.add(MaterialButton::filled("Filled"));
                ui.add(MaterialButton::outline("Outline"));
                ui.add(MaterialButton::plain("Plain"));
                ui.add(MaterialButton::hero("Hero").size(MaterialButtonSize::Small));
            });

            ui.add(MaterialCheckbox::new(&mut self.checked, "Accept terms"));
            ui.add(MaterialSwitch::new(&mut self.switch_on).text("Enable feature"));
        });
    }
}
```

### Fonts

Nala body/display fonts are **not** committed to git (license/size). Either:

1. Place `inter.ttf`, `poppins.ttf`, and `roboto-mono.ttf` in your app’s `resources/` directory, or  
2. Enable the `ondemand` feature so they download from Google Fonts at runtime.

Material Symbols (`MaterialSymbolsOutlined.ttf`) is still required for icon glyphs.

### Quick start (upstream Material theme)

<details>
<summary>Using the original Material theme and Roboto instead of Nala</summary>

```rust
use egui_material3::{
    MaterialButton, MaterialCheckbox, MaterialSlider,
    theme::{setup_google_fonts, setup_local_fonts, setup_local_theme,
           load_fonts, load_themes, update_window_background},
};

// In eframe startup:
setup_google_fonts(Some("Roboto"));
setup_local_fonts(Some("resources/MaterialSymbolsOutlined.ttf"));
setup_local_theme(Some("resources/material-theme.json")); // custom M3 JSON
load_fonts(&cc.egui_ctx);
load_themes();
update_window_background(&cc.egui_ctx);
```

</details>

### Advanced Example

Here's a more comprehensive example showcasing recent additions:

```rust
use egui_material3::{
    MaterialButton, MaterialBadge, MaterialToolbar, MaterialBreadcrumbs,
    MaterialNotification, MaterialTimeline, MaterialTooltip, MaterialTreeView,
    ButtonVariant, TimelineItem, TreeNode,
};

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Toolbar with actions
            ui.add(MaterialToolbar::new()
                .title("My Application")
                .add_action("search", || println!("Search"))
                .add_action("settings", || println!("Settings")));

            // Breadcrumb navigation
            ui.add(MaterialBreadcrumbs::new()
                .add_item("Home", || println!("Home"))
                .add_item("Projects", || println!("Projects"))
                .add_current("Current Project"));

            // Badge example
            ui.horizontal(|ui| {
                ui.add(MaterialBadge::new().value(3).show(ui, |ui| {
                    ui.add(MaterialButton::new("Messages"));
                }));

                // Tooltip example
                MaterialTooltip::new("Click to refresh").show(ui, |ui| {
                    ui.add(MaterialButton::new("Refresh")
                        .variant(ButtonVariant::Outlined));
                });
            });

            // Notification
            ui.add(MaterialNotification::new("Update available")
                .description("Version 2.0 is ready to install")
                .add_action("Update", || println!("Updating..."))
                .closable(true));

            // Timeline
            let events = vec![
                TimelineItem::new("Project created").with_timestamp("2024-01-01"),
                TimelineItem::new("First commit").with_timestamp("2024-01-02"),
                TimelineItem::new("Version 1.0 released").with_timestamp("2024-02-01"),
            ];
            ui.add(MaterialTimeline::new(events));

            // TreeView for hierarchical data
            let tree = vec![
                TreeNode::new("Root")
                    .add_child(TreeNode::new("Child 1"))
                    .add_child(TreeNode::new("Child 2")
                        .add_child(TreeNode::new("Grandchild"))),
            ];
            ui.add(MaterialTreeView::new(tree));
        });
    }
}
```

## Theme System

The library provides Material Design 3 theming. **In this fork, `setup_local_theme(None)` loads the bundled Nala theme** (`resources/nala-material-theme.json`) derived from Leo color tokens. Light/dark and contrast variants are included.

### Nala theme (default in this fork)

```rust
use egui_material3::theme::{
    apply_nala_text_styles, load_fonts, load_themes,
    setup_local_theme, setup_nala_fonts, update_window_background,
};

setup_nala_fonts();
setup_local_theme(None); // nala-material-theme.json
load_fonts(ctx);
apply_nala_text_styles(ctx);
load_themes();
update_window_background(ctx);
```

### Build-time theme inclusion

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

Dynamically change theme appearance at runtime:

```rust
use egui_material3::theme::{get_global_theme, update_window_background, ThemeMode, ContrastLevel};

// Switch between light and dark modes
if let Ok(mut theme) = get_global_theme().lock() {
    theme.theme_mode = ThemeMode::Dark; // or ThemeMode::Light
    theme.contrast_level = ContrastLevel::High; // Standard, Medium, or High
}
update_window_background(ctx);

// Or toggle mode with a button
if ui.add(MaterialButton::new("Toggle Dark Mode")).clicked() {
    if let Ok(mut theme) = get_global_theme().lock() {
        theme.theme_mode = match theme.theme_mode {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        };
    }
    update_window_background(ctx);
}
```

### Component size variants (Nala)

Nala-aligned components support Leo size scales:

```rust
use egui_material3::{
    MaterialButton, MaterialButtonSize, MaterialCheckbox, MaterialCheckboxSize,
    MaterialSwitch, MaterialSwitchSize,
};

// Buttons: tiny, small, medium, large, jumbo
ui.add(MaterialButton::filled("Save").size(MaterialButtonSize::Large));
ui.add(MaterialButton::plain("Cancel").size(MaterialButtonSize::Small));

// Checkbox: normal (20px), small (16px)
ui.add(MaterialCheckbox::new(&mut checked, "Label").size(MaterialCheckboxSize::Small));

// Switch / toggle: medium (52×32), small (40×24)
ui.add(MaterialSwitch::new(&mut on).size(MaterialSwitchSize::Small).text("Wi-Fi"));
```

### Component size variants (upstream)

Other components (chips, badges, etc.) may use upstream Material size APIs — see individual story windows.

## Available Components

### Nala-aligned (Leo styling)

- **MaterialButton** — `filled`, `outline`, `plain`, `plain-faint`, `hero` (+ legacy M3 variants). Sizes: `tiny` → `jumbo`
- **MaterialSwitch** — Leo toggle; sizes `medium` / `small`
- **MaterialCheckbox** — Leo SVG icons; sizes `normal` / `small`; indeterminate supported
- **MaterialNavigation** — Vertical sidebar with header, dividers, nested subnav, animated active indicator

### Input & Selection

- **MaterialButton** — See Nala-aligned section above; also `elevated`, `filled_tonal` for legacy M3
- **MaterialIconButton** - Icon buttons (standard, filled, filled tonal, outlined, toggle)
- **MaterialCheckbox** - See Nala-aligned section above
- **MaterialSwitch** - See Nala-aligned section above
- **MaterialRadio** / **MaterialRadioGroup** - Radio button groups with list tile support
- **MaterialSlider** / **MaterialRangeSlider** - Sliders with Material Design styling
- **MaterialSelect** - Dropdown selection components with menu alignment options
- **MaterialChip** - Filter, assist, input, and suggestion chips with size variants

### Navigation & Layout

- **MaterialNavigation** - Nala sidebar navigation (Leo `Navigation`)
- **MaterialTabs** - Tab navigation (primary and secondary variants)
- **MaterialDrawer** - Navigation drawers (permanent, dismissible, modal, standard)
- **MaterialTopAppBar** - App bars and toolbars (standard, center-aligned, medium, large)
- **MaterialToolbar** - Flexible toolbar component with action items
- **MaterialBreadcrumbs** - Breadcrumb navigation for hierarchical paths
- **MaterialMenu** - Context menus and menu items with nested support

### Feedback & Information

- **MaterialDialog** - Modal dialogs and alerts
- **MaterialSnackbar** - Toast notifications with optional actions
- **MaterialNotification** - Notification cards with actions and dismissal
- **MaterialBadge** - Badge indicators for counts and status
- **MaterialProgress** - Progress indicators (circular and linear)
- **MaterialTooltip** - Contextual tooltips with rich text support
- **MaterialActionSheet** - Bottom sheets for action selection

### Data Display

- **MaterialCard2** - Material Design cards (elevated, filled, outlined variants)
- **MaterialList** - Lists following Material Design patterns with visual density control
- **MaterialDataTable** - Data tables with sorting, selection, and custom cell content
- **MaterialSpreadsheet** - Full-featured spreadsheet with DuckDB backend (requires `spreadsheet` feature)
- **MaterialTimeline** - Timeline component for chronological data
- **MaterialTreeView** - Hierarchical tree view with expand/collapse

### Media & Content

- **MaterialCarousel** - Carousel for displaying items in a scrollable view with mouse drag support
- **MaterialImageList** - Image lists with online/offline support and smart caching (standard, masonry, woven variants)
- **MaterialLayoutGrid** - Grid layout with tile bars
- **MaterialFab** - Floating Action Buttons (primary, secondary, tertiary, surface, branded)

### Icons & Symbols

- **MaterialIcon** - Material Design icons with font support
- **MaterialSymbol** - Material Symbols rendering (outlined, rounded, sharp variants)

## Common Patterns

### Building a Complete UI

Combine components to create rich user interfaces:

```rust
use egui_material3::*;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top app bar
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.add(MaterialTopAppBar::new()
                .title("My App")
                .add_action("notifications", |ui| {
                    // Badge on icon button
                    MaterialBadge::new().value(5).show(ui, |ui| {
                        ui.add(MaterialIconButton::new("notifications"));
                    });
                }));
        });

        // Navigation drawer
        egui::SidePanel::left("drawer").show(ctx, |ui| {
            ui.add(MaterialDrawer::new()
                .add_item("Home", "home", || println!("Home"))
                .add_item("Settings", "settings", || println!("Settings")));
        });

        // Main content
        egui::CentralPanel::default().show(ctx, |ui| {
            // Action buttons with tooltips
            ui.horizontal(|ui| {
                MaterialTooltip::new("Create new item").show(ui, |ui| {
                    if ui.add(MaterialButton::new("New")
                        .variant(ButtonVariant::Filled)).clicked() {
                        // Show action sheet
                        self.show_action_sheet = true;
                    }
                });
            });

            // Data display with cards
            ui.add(MaterialCard2::elevated()
                .show(ui, |ui| {
                    ui.heading("Recent Activity");
                    ui.add(MaterialTimeline::new(self.recent_events.clone()));
                }));

            // Notifications
            if self.has_updates {
                ui.add(MaterialNotification::new("Update available")
                    .add_action("Install", || self.install_update())
                    .closable(true));
            }
        });

        // Bottom action sheet
        if self.show_action_sheet {
            ui.add(MaterialActionSheet::new()
                .add_action("Create Document", || println!("Document"))
                .add_action("Create Folder", || println!("Folder"))
                .on_dismiss(|| self.show_action_sheet = false));
        }

        // FAB (Floating Action Button)
        egui::Area::new("fab")
            .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-16.0, -16.0))
            .show(ctx, |ui| {
                if ui.add(MaterialFab::primary().icon("add")).clicked() {
                    println!("FAB clicked");
                }
            });
    }
}
```

### Form with Validation

Create forms with Material components:

```rust
ui.vertical(|ui| {
    ui.label("User Information");

    // Text inputs with validation
    ui.add(MaterialTextField::new(&mut self.name)
        .label("Full Name")
        .required(true));

    // Selection controls
    ui.add(MaterialSelect::new(&mut self.country)
        .label("Country")
        .options(vec!["USA", "UK", "Canada"]));

    // Checkboxes for preferences
    ui.add(MaterialCheckbox::new(&mut self.newsletter,
        "Subscribe to newsletter"));

    ui.add(MaterialCheckbox::new(&mut self.terms,
        "I agree to the terms"));

    // Action buttons
    ui.horizontal(|ui| {
        if ui.add(MaterialButton::new("Submit")
            .variant(ButtonVariant::Filled)
            .enabled(self.terms)).clicked() {
            self.submit_form();
        }

        ui.add(MaterialButton::new("Cancel")
            .variant(ButtonVariant::Text));
    });
});
```

## Optional Icon & Emoji Collections

The library provides three comprehensive SVG collections as optional features. Each can be enabled independently:

- **Solar Icons** (`svg_solar`) - ~1,200 UI/UX icons with variants
- **Noto Emoji** (`svg_noto`) - ~3,600 Google emojis with skin tone and gender variants
- **Twemoji** (`svg_twemoji`) - ~3,700 Twitter emoji

### Feature Configuration

Choose the collections you need:

```toml
[dependencies]
# Enable individual collections (recommended - smaller binary size)
egui-material3 = { version = "...", features = ["svg_solar"] }

# Or enable specific combinations
egui-material3 = { version = "...", features = ["svg_solar", "svg_noto"] }

# Or enable all collections
egui-material3 = { version = "...", features = ["svg_emoji"] }
```

### Usage

Icons and emojis are accessible through HashMaps with O(1) lookup:

```rust
use egui_material3::svg_emoji::{SOLAR_ICONS, NOTO_EMOJIS, TWEMOJI};

// Access Solar icons (requires svg_solar feature)
if let Some(svg) = SOLAR_ICONS.get("home") {
    // Use SVG data for rendering
}

// Access Noto emoji (requires svg_noto feature)
// Filename format: "emoji_u" + unicode codepoint
if let Some(svg) = NOTO_EMOJIS.get("emoji_u1f600") {
    // 😀 Grinning face emoji
}

// Access Twemoji (requires svg_twemoji feature)
// Filename format: unicode codepoint
if let Some(svg) = TWEMOJI.get("1f600") {
    // 😀 Grinning face emoji
}
```

**Note**: SVG files are embedded at compile time when features are enabled. If building from crates.io (not git), files are automatically downloaded during build.

## Features

### OnDemand Feature

Enable online image support for `MaterialImageList`:

```toml
[dependencies]
egui-material3 = { version = "...", features = ["ondemand"] }
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
egui-material3 = { version = "...", features = ["spreadsheet"] }
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

The crate includes comprehensive examples demonstrating all components:

```bash
# Complete showcase of all Material components with theme switching
cargo run --example widget_gallery_example

# Real-world data table implementation with Nobel Prize data
cargo run --example nobel_prizes_example

# Interactive component gallery (recommended for exploration)
cargo run --example stories

# SVG icon demonstration (requires svg_solar feature)
cargo run --example svg_icon_demo --features svg_solar
```

### Stories Example - Component Explorer

The `stories` example provides an interactive gallery with individual showcases for each component:

**Input & Selection**: actionsheet, button, checkbox, chips, iconbutton, radio, select, slider, switch  
**Navigation**: breadcrumbs, drawer, **navigation**, menu, tabs, toolbar, topappbar, treeview
**Feedback**: badge, dialog, notification, progress, snackbar, tooltip
**Data Display**: card2, datatable, list, spreadsheet, timeline
**Media**: carousel, imagelist, layoutgrid, svgemoji, symbol

Each story window demonstrates component variants, states, and common usage patterns. **Start with `button`, `checkbox`, `switch`, and `navigation` stories to preview Nala styling.**

### Standalone Examples

```bash
# OnDemand example - demonstrates online image loading
cd examples/ondemand && cargo run

# Package example - standalone deployable app with bundled resources
cd examples/package && cargo run
```

### Running with Features

```bash
# Run with spreadsheet support
cargo run --example stories --features spreadsheet

# Run with all SVG icon collections
cargo run --example stories --features svg_emoji

# Run with specific features
cargo run --example stories --features "ondemand,svg_solar"
```

## Documentation

- [API Documentation](https://docs.rs/egui-material3) (upstream; fork APIs may differ until published)
- [Leo design system](https://github.com/brave/leo)
- [Material Design 3 Guidelines](https://m3.material.io/)
- [Examples](./examples/)

## Contributing

This fork is maintained at [brave-experiments/nala-egui-material3](https://github.com/brave-experiments/nala-egui-material3). Contributions welcome via issues and pull requests.

Upstream egui-material3: [nikescar/egui-material3](https://github.com/nikescar/egui-material3)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-Apache-2.0](LICENSE-Apache-2.0) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---

<details markdown>
<summary>Development Notes</summary>

## Upstream

Based on [egui-material3](https://github.com/nikescar/egui-material3) by Woojae Park. Nala customizations live alongside upstream components.

## Todos

* Align more components with Leo (radio, tabs, segmented control, …)
* SVG sprite support
* Bump egui_extras to match resvg version (currently using patched 0.47)
* Performance optimizations for large datasets

</details>
