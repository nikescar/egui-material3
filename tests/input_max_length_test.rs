//! `MaterialInput::max_length` counts characters, not bytes.

use egui_material3::MaterialInput;

fn render_with_max_length(text: &mut String, max: usize) {
    let ctx = egui::Context::default();
    ctx.run_ui(Default::default(), |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.add(MaterialInput::outlined(text).max_length(max));
        });
    })
    .textures_delta
    .clear();
}

#[test]
fn multibyte_text_is_truncated_on_a_character_boundary() {
    let mut text = String::from("€€€");
    render_with_max_length(&mut text, 2);
    assert_eq!(text, "€€", "max_length 2 keeps two characters");
}

#[test]
fn text_within_the_limit_is_left_alone() {
    let mut text = String::from("ab€");
    render_with_max_length(&mut text, 3);
    assert_eq!(text, "ab€", "three characters fit a max_length of 3");
}
