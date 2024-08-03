use engine::{
    contract::{EngineMessage, APP_COMMAND_TOPIC},
    log,
    service::{client::Client, Broker, Service},
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
