fn main() {
    engine::launch(App);
}

#[derive(Default)]
pub struct App;

impl engine::State for App {
    fn update(
        &mut self,
        _service_bus: &mut engine::ServiceBus,
        ui_context: &engine::egui::Context,
    ) {
        engine::egui::Window::new("Doge").show(ui_context, |ui| {
            ui.heading("doge");
        });
    }
}
