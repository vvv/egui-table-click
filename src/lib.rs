use eframe::egui;

#[derive(Debug, Default)]
struct State;

#[derive(Debug)]
pub struct HelloApp {
    state: State,
}

impl HelloApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // TODO: Load previous app state; see
        // <https://github.com/emilk/eframe_template/blob/4f613f5d6266f0f0888544df4555e6bc77a5d079/src/app.rs#L29-L33> and
        // <https://github.com/emilk/eframe_template/blob/4f613f5d6266f0f0888544df4555e6bc77a5d079/src/app.rs#L40-L43>.

        Self {
            state: State::default(),
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
            ui.heading("XXX-hello-egui-table");
            egui::warn_if_debug_build(ui);
        });
    }
}
