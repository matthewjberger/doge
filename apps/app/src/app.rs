use crate::services::{Command, Event, NotificationService};

#[derive(Default)]
pub struct App {
    registered_services: bool,
}

impl engine::State<Command, Event> for App {
    fn update(
        &mut self,
        bus: &mut engine::ServiceBus<Command, Event>,
        ui_context: &engine::egui::Context,
    ) {
        if !self.registered_services {
            bus.register_service(NotificationService::default());
            self.registered_services = true;
        }

        engine::egui::Window::new("DOGE - Distribution Oriented Game Engine").show(
            ui_context,
            |ui| {
                ui.heading("Hello, world!");
                if ui.button("Click me!").clicked() {
                    let command = Command::Notify {
                        content: "Hello, world!".to_string(),
                    };
                    bus.publish_app_command(command);
                }
            },
        );
    }
}
