use eframe::egui;

#[derive(Debug)]
pub struct HelloApp;

impl eframe::App for HelloApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))]
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::F11)) {
            frame.set_fullscreen(!frame.info().window_info.fullscreen);
        }

        // TODO: Add the menu bar; see
        // <https://github.com/emilk/eframe_template/blob/4f613f5d6266f0f0888544df4555e6bc77a5d079/src/app.rs#L55-L65>

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::widgets::global_dark_light_mode_switch(ui);
            egui::warn_if_debug_build(ui);

            ui.separator();

            egui::ScrollArea::horizontal().show(ui, |ui| {
                self.table_ui(ui);
            });
        });
    }
}

impl HelloApp {
    fn table_ui(&mut self, ui: &mut egui::Ui) {
        use egui::Label;
        use egui_extras::{Column, TableBuilder};

        let row_height = ui.text_style_height(&egui::TextStyle::Body);
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .selectable(true)
            .frame(true)
            .column(Column::auto())
            .column(Column::initial(100.0).at_least(40.0).clip(true))
            .column(Column::remainder())
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("_Row");
                });
                header.col(|ui| {
                    ui.strong("Key");
                });
                header.col(|ui| {
                    ui.strong("Value");
                });
            })
            .body(|body| {
                body.rows(row_height, 10_000, |row_idx, mut row| {
                    row.col(|ui| {
                        ui.add(Label::new(row_idx.to_string()).wrap(false));
                    });
                    row.col(|ui| {
                        ui.add(Label::new("Thousands of rows of even height").wrap(false));
                    });
                    row.col(|ui| {
                        ui.add(Label::new(format!("Row {row_idx} has some long text that you may want to clip, or it will take up too much horizontal space")).wrap(false));
                    });
                });
            });
    }
}
