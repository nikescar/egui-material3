//! Material Design components for egui
//!
//! This crate provides Material Design 3 components for egui applications,
//! including buttons, text fields, and other UI elements that follow
//! Material Design guidelines.

pub mod icons;
pub mod image_utils;
pub mod button;
pub mod button_window;
pub mod checkbox;
pub mod checkbox_window;
pub mod chips;
pub mod chips_window;
pub mod dialog;
pub mod dialog_window;
pub mod fab;
pub mod fab_window;
pub mod icon;
pub mod iconbutton;
pub mod iconbutton_window;
pub mod list;
pub mod list_window;
pub mod menu;
pub mod menu_window;
pub mod progress;
pub mod progress_window;
pub mod radio;
pub mod radio_window;
pub mod select;
pub mod select_window;
pub mod slider;
pub mod slider_window;
pub mod switch;
pub mod switch_window;
pub mod tabs;
pub mod tabs_window;
pub mod datatable;
pub mod datatable_window;
pub mod drawer;
pub mod drawer_window;
pub mod imagelist;
pub mod imagelist_window;
pub mod layoutgrid;
pub mod layoutgrid_window;
pub mod snackbar;
pub mod snackbar_window;
pub mod topappbar;
pub mod topappbar_window;
pub mod card2;
pub mod card2_window;
pub mod theme;

pub use {
    button::{MaterialButton, MaterialButtonVariant},
    button_window::ButtonWindow,
    checkbox::{MaterialCheckbox, checkbox},
    checkbox_window::CheckboxWindow,
    chips::{MaterialChip, ChipVariant, assist_chip, filter_chip, input_chip, suggestion_chip},
    chips_window::ChipsWindow,
    dialog::{MaterialDialog, dialog},
    dialog_window::DialogWindow,
    fab::{MaterialFab, FabVariant, FabSize, SvgIcon, SvgPath, fab_surface, fab_primary, fab_secondary, fab_tertiary, fab_branded, google_branded_icon},
    fab_window::FabWindow,
    icon::{MaterialIcon, icon},
    iconbutton::{MaterialIconButton, IconButtonVariant, icon_button_standard, icon_button_filled, icon_button_filled_tonal, icon_button_outlined, icon_button_toggle},
    iconbutton_window::IconButtonWindow,
    list::{MaterialList, ListItem, list, list_item},
    list_window::ListWindow,
    menu::{MaterialMenu, MenuItem, Corner, FocusState, Positioning, menu, menu_item},
    menu_window::MenuWindow,
    progress::{MaterialProgress, ProgressVariant, linear_progress, circular_progress},
    progress_window::ProgressWindow,
    radio::{MaterialRadio, MaterialRadioGroup, radio, radio_group},
    radio_window::RadioWindow,
    select::{MaterialSelect, select},
    select_window::SelectWindow,
    slider::{MaterialSlider, slider},
    slider_window::SliderWindow,
    switch::{MaterialSwitch, switch},
    switch_window::SwitchWindow,
    tabs::{MaterialTabs, TabVariant, tabs_primary, tabs_secondary},
    tabs_window::TabsWindow,
    datatable::{MaterialDataTable, DataTableColumn, DataTableRow, SortDirection, data_table},
    datatable_window::DataTableWindow,
    drawer::{MaterialDrawer, DrawerVariant, DrawerItem, permanent_drawer, modal_drawer, dismissible_drawer, standard_drawer},
    drawer_window::DrawerWindow,
    imagelist::{MaterialImageList, ImageListVariant, ImageListItem, image_list, masonry_image_list, woven_image_list},
    imagelist_window::ImageListWindow,
    layoutgrid::{MaterialLayoutGrid, layout_grid, debug_layout_grid},
    layoutgrid_window::LayoutGridWindow,
    snackbar::{MaterialSnackbar, SnackbarPosition, snackbar, snackbar_with_action},
    snackbar_window::SnackbarWindow,
    topappbar::{MaterialTopAppBar, TopAppBarVariant, top_app_bar, center_aligned_top_app_bar, medium_top_app_bar, large_top_app_bar},
    topappbar_window::TopAppBarWindow,
    card2::{MaterialCard2, Card2Variant, elevated_card2, filled_card2, outlined_card2},
    card2_window::Card2Window,
    theme::{get_global_color, get_global_theme, update_global_theme, MaterialThemeContext, ThemeMode, ContrastLevel, MaterialThemeFile},
    egui::TextEdit, // Re-export egui's TextEdit
};
