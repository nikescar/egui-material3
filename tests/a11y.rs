//! Every interactive widget reports its role, label and state to AccessKit,
//! disabled widgets report disabled and do not click, and the keyboard
//! reaches and activates them (#22).

mod support;

use std::cell::Cell;

use egui::accesskit::{Role, Toggled};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_material3::{
    assist_chip, filter_chip, slider, MaterialButton, MaterialCheckbox, MaterialFab,
    MaterialIconButton, MaterialRadio, MaterialSwitch, MaterialTabs,
};

#[test]
fn widgets_report_role_label_and_state() {
    for (name, theme) in support::themes() {
        let mut checked = true;
        let mut on = false;
        let mut choice = Some(1);
        let mut value = 0.25_f32;
        let mut picked = true;
        let mut tab = 1;
        let harness = support::harness_for(theme, egui::vec2(600.0, 700.0), |ui| {
            ui.add(MaterialButton::filled("Save"));
            ui.add(MaterialCheckbox::new(&mut checked, "Remember me"));
            ui.add(MaterialSwitch::new(&mut on).text("Wi-Fi"));
            ui.add(MaterialRadio::new(&mut choice, 1, "Option one"));
            ui.add(slider(&mut value, 0.0..=1.0).text("Volume"));
            ui.add(MaterialIconButton::standard("delete").label("Delete item"));
            ui.add(assist_chip("Directions"));
            ui.add(filter_chip("Vegan", &mut picked));
            ui.add(MaterialFab::primary().icon("add").label("Compose"));
            ui.add(MaterialTabs::primary(&mut tab).tab("Home").tab("Settings"));
        });
        let role = |label: &str| harness.get_by_label(label).accesskit_node().role();
        let toggled = |label: &str| harness.get_by_label(label).accesskit_node().toggled();
        assert_eq!(role("Save"), Role::Button, "{name}: button");
        assert_eq!(role("Remember me"), Role::CheckBox, "{name}: checkbox");
        assert_eq!(
            toggled("Remember me"),
            Some(Toggled::True),
            "{name}: checked"
        );
        assert_eq!(role("Wi-Fi"), Role::Switch, "{name}: switch");
        assert_eq!(toggled("Wi-Fi"), Some(Toggled::False), "{name}: switch off");
        assert_eq!(role("Option one"), Role::RadioButton, "{name}: radio");
        assert_eq!(
            toggled("Option one"),
            Some(Toggled::True),
            "{name}: radio selected"
        );
        assert_eq!(role("Volume"), Role::Slider, "{name}: slider");
        assert_eq!(
            harness
                .get_by_label("Volume")
                .accesskit_node()
                .numeric_value(),
            Some(0.25),
            "{name}: slider value"
        );
        assert_eq!(
            role("Delete item"),
            Role::Button,
            "{name}: icon button label"
        );
        assert_eq!(role("Directions"), Role::Button, "{name}: assist chip");
        assert_eq!(role("Vegan"), Role::CheckBox, "{name}: filter chip");
        assert_eq!(
            toggled("Vegan"),
            Some(Toggled::True),
            "{name}: filter chip selected"
        );
        assert_eq!(role("Compose"), Role::Button, "{name}: fab label");
        assert_eq!(role("Settings"), Role::Tab, "{name}: tab");
        assert_eq!(
            toggled("Settings"),
            Some(Toggled::True),
            "{name}: selected tab"
        );
        assert_eq!(
            toggled("Home"),
            Some(Toggled::False),
            "{name}: unselected tab"
        );
    }
}

#[test]
fn disabled_widgets_report_disabled_and_ignore_clicks() {
    let clicks = Cell::new(0);
    let mut checked = false;
    let mut harness = support::harness_for(
        egui_material3::MaterialTheme::baseline(false),
        egui::vec2(400.0, 200.0),
        |ui| {
            if ui
                .add(MaterialButton::filled("Disabled").enabled(false))
                .clicked()
            {
                clicks.set(clicks.get() + 1);
            }
            ui.add(MaterialCheckbox::new(&mut checked, "Locked").enabled(false));
        },
    );
    assert!(
        harness
            .get_by_label("Disabled")
            .accesskit_node()
            .is_disabled(),
        "button disabled"
    );
    assert!(
        harness
            .get_by_label("Locked")
            .accesskit_node()
            .is_disabled(),
        "checkbox disabled"
    );
    harness.get_by_label("Disabled").click();
    harness.get_by_label("Locked").click();
    harness.run();
    assert_eq!(
        clicks.get(),
        0,
        "a disabled button does not report clicked()"
    );
    drop(harness);
    assert!(!checked, "a disabled checkbox does not toggle");
}

#[test]
fn keyboard_reaches_and_activates_a_button() {
    let clicks = Cell::new(0);
    let mut harness = support::harness_for(
        egui_material3::MaterialTheme::baseline(false),
        egui::vec2(300.0, 120.0),
        |ui| {
            if ui.add(MaterialButton::filled("Go")).clicked() {
                clicks.set(clicks.get() + 1);
            }
        },
    );
    harness.key_press(egui::Key::Tab);
    harness.run();
    assert!(
        harness.get_by_label("Go").is_focused(),
        "Tab focuses the button"
    );
    harness.key_press(egui::Key::Enter);
    harness.run();
    harness.key_press(egui::Key::Space);
    harness.run();
    assert_eq!(clicks.get(), 2, "Enter and Space activate it");
}
