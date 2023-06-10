use eframe::egui;

#[derive(Debug)]
pub struct HelloApp {
    /// Total number of rows in the table.
    num_rows: usize,
    /// Index of the selected row, if any.
    selected_row: Option<usize>,
}

impl HelloApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // TODO: Load previous app state; see
        // <https://github.com/emilk/eframe_template/blob/4f613f5d6266f0f0888544df4555e6bc77a5d079/src/app.rs#L29-L33> and
        // <https://github.com/emilk/eframe_template/blob/4f613f5d6266f0f0888544df4555e6bc77a5d079/src/app.rs#L40-L43>.

        Self {
            num_rows: 10_000,
            selected_row: None,
        }
    }
}

impl eframe::App for HelloApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))]
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::F11)) {
            frame.set_fullscreen(!frame.info().window_info.fullscreen);
        }

        // TODO: Add the menu bar; see
        // <https://github.com/emilk/eframe_template/blob/4f613f5d6266f0f0888544df4555e6bc77a5d079/src/app.rs#L55-L65>

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.selected_row.map_or_else(
                || "Click on any row to select it".to_owned(),
                |row_idx| format!("Selected row: {row_idx}"),
            ));
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
        use egui::{Color32, Label, RichText};
        use egui_extras::{Column, TableBuilder};

        let row_height = ui.text_style_height(&egui::TextStyle::Body);
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
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
                body.rows(row_height, self.num_rows, |row_idx, mut row| {
                    // Add `_Row` cell.
                    let (_rect, response) = row.col(|ui| {
                        let text = RichText::new(row_idx.to_string());
                        let text = if self.selected_row == Some(row_idx) {
                            text.background_color(Color32::LIGHT_BLUE)
                        } else {
                            text
                        };
                        ui.add(Label::new(text).wrap(false));
                    });
                    if response.clicked() {
                        self.selected_row = Some(row_idx);
                    }

                    // Add `Key` cell.
                    let (_rect, response) = row.col(|ui| {
                        let text = RichText::new("Thousands of rows of even height");
                        let text = if self.selected_row == Some(row_idx) {
                            text.background_color(Color32::LIGHT_BLUE)
                        } else {
                            text
                        };
                        ui.add(Label::new(text).wrap(false));
                    });
                    if response.clicked() {
                        self.selected_row = Some(row_idx);
                    }

                    // Add `Value` cell.
                    let (_rect, response) = row.col(|ui| {
                        let text = RichText::new(format!("Row {row_idx} has some long text that you may want to clip, or it will take up too much horizontal space"));
                        let text = if self.selected_row == Some(row_idx) {
                            text.background_color(Color32::LIGHT_BLUE)
                        } else {
                            text
                        };
                        ui.add(Label::new(text).wrap(false));
                    });
                    if response.clicked() {
                        self.selected_row = Some(row_idx);
                    }
                });
            });
    }
}
