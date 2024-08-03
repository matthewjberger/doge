fn main() {
    engine::launch(App::default());
}

#[derive(Default)]
pub struct App {}

impl engine::State for App {
    fn update(&mut self, bus: &mut engine::ServiceBus, ui_context: &engine::egui::Context) {
        #[cfg(not(target_arch = "wasm32"))]
        let title = "Rust/Wgpu";

        #[cfg(feature = "webgpu")]
        let title = "Rust/Wgpu/Webgpu";

        #[cfg(feature = "webgl")]
        let title = "Rust/Wgpu/Webgl";

        engine::egui::Window::new(title).show(ui_context, |ui| {
            ui.heading("Hello, world!");
            if ui.button("Click me!").clicked() {
                engine::log::info!("Button clicked!");
                let command = engine::contract::Command::Notify {
                    content: "Hello, world!".to_string(),
                };
                bus.publish_command(command);
            }
        });
    }
}
