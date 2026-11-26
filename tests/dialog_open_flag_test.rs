//! The dialog hands its open flag to the content callback.

use egui_material3::MaterialDialog;

fn show_dialog(open: &mut bool, close_from_content: bool) {
    let ctx = egui::Context::default();
    ctx.run_ui(Default::default(), |ui| {
        MaterialDialog::new(egui::Id::unique("dialog_open_flag"), "Title", open)
            .content(move |ui, open| {
                ui.label("Body");
                if close_from_content {
                    *open = false;
                }
            })
            .show(ui.ctx());
    })
    .textures_delta
    .clear();
}

#[test]
fn content_can_close_the_dialog() {
    let mut open = true;
    show_dialog(&mut open, true);
    assert!(
        !open,
        "content set the flag to false, so the dialog is closed"
    );
}

#[test]
fn an_untouched_dialog_stays_open() {
    let mut open = true;
    show_dialog(&mut open, false);
    assert!(open, "nothing asked the dialog to close");
}
