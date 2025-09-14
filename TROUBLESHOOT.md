

```bash
error[E0308]: mismatched types
   --> bingtray/src/main.rs:58:24
    |
 58 |             load_fonts(&cc.egui_ctx);
    |             ---------- ^^^^^^^^^^^^ expected `egui::context::Context`, found `eframe::egui::Context`
    |             |
    |             arguments to this function are incorrect
    |
```