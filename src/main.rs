#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    eframe::run_native(
        "egui-table-row-select-and-frame",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            let app: egui_table_click::HelloApp = cc
                .storage
                .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
                .unwrap_or_default();
            Box::new(app)
        }),
    )
}
