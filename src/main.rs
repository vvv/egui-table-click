#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    eframe::run_native(
        "egui-table-click",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(egui_table_click::HelloApp::new(cc))),
    )
}
