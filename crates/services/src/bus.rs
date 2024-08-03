use client::Client;
use contract::{Command, Message};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default)]
pub struct NotificationService {
    client: Client,
    subscribed: bool,
}

impl Service for NotificationService {
    fn update(&mut self, broker: &mut Broker) {
        if !self.subscribed {
            log::info!("Notification service initialized");
            self.client
                .subscribe_to_topic(Message::COMMAND_TOPIC, broker);
            self.subscribed = true;
        }
        if let Some(Message::Command {
            command: Command::Notify { content },
        }) = self.client.next_message()
        {
            log::info!("NotificationService::update: {content}");
        }
    }
}

pub type Broker = broker::Broker<contract::Message>;

pub trait Service {
    fn update(&mut self, _broker: &mut Broker);
}

/// Contains the main message broker
///
/// Services can be registered here
#[derive(Default)]
pub struct ServiceBus {
    broker: Broker,
    services: HashMap<Uuid, Box<dyn Service>>,
}

impl ServiceBus {
    pub fn new() -> Self {
        let mut bus = Self::default();
        bus.register_service(NotificationService::default());
        bus
    }
}

impl ServiceBus {
    /// Registers a service with the bus
    pub fn register_service(&mut self, service: impl Service + 'static) -> uuid::Uuid {
        let uuid = uuid::Uuid::new_v4();
        self.services.insert(uuid, Box::new(service));
        uuid
    }

    /// Unregisters a service with the bus
    pub fn unregister_service(&mut self, uuid: &uuid::Uuid) {
        self.services.remove(uuid);
    }

    /// Publish a message to the broker
    pub fn publish_message(&mut self, topic: &str, message: contract::Message) {
        log::info!("Published message. topic: {topic} {message:?}");
        self.broker.publish(topic, message);
    }

    /// Publish a command message to the broker
    pub fn publish_command(&mut self, command: contract::Command) {
        self.publish_message(
            contract::Message::COMMAND_TOPIC,
            contract::Message::Command { command },
        );
    }

    /// Called continually to update all services
    pub fn update(&mut self) {
        self.services.values_mut().for_each(|service| {
            service.update(&mut self.broker);
        });
    }
}
