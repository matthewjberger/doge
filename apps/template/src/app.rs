use engine::{
    contract::{EngineMessage, APP_COMMAND_TOPIC},
    log,
    services::{client::Client, Broker, Service},
};

#[derive(Debug, Clone)]
pub enum Command {
    Notify { content: String },
}

#[derive(Default, Debug, Clone)]
pub enum Event {
    #[default]
    Empty,
}

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

        #[cfg(not(target_arch = "wasm32"))]
        let title = "Rust/Wgpu";

        #[cfg(feature = "webgpu")]
        let title = "Rust/Wgpu/Webgpu";

        #[cfg(feature = "webgl")]
        let title = "Rust/Wgpu/Webgl";

        engine::egui::Window::new(title).show(ui_context, |ui| {
            ui.heading("Hello, world!");
            if ui.button("Click me!").clicked() {
                let command = Command::Notify {
                    content: "Hello, world!".to_string(),
                };
                bus.publish_app_command(command);
            }
        });
    }
}

#[derive(Default)]
pub struct NotificationService {
    client: Client<Command, Event>,
    subscribed: bool,
}

impl Service<Command, Event> for NotificationService {
    fn update(&mut self, broker: &mut Broker<Command, Event>) {
        if !self.subscribed {
            log::info!("[Initialize] Notification service initialized");
            self.client.subscribe_to_topic(APP_COMMAND_TOPIC, broker);
            self.subscribed = true;
        }
        if let Some(EngineMessage::AppCommand {
            command: Command::Notify { content },
        }) = self.client.next_message()
        {
            log::info!("[Notify] {content}");
        }
    }
}
