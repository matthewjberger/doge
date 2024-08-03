use contract::{
    EngineEvent, APP_COMMAND_TOPIC, APP_EVENT_TOPIC, ENGINE_COMMAND_TOPIC, ENGINE_EVENT_TOPIC,
};
use core::fmt;
use std::collections::HashMap;
use uuid::Uuid;

pub type Broker<C, E> = broker::Broker<contract::EngineMessage<C, E>>;

pub trait Service<C, E>
where
    C: Clone + fmt::Debug + 'static,
    E: Clone + fmt::Debug + 'static,
{
    fn update(&mut self, _broker: &mut Broker<C, E>);
}

/// Contains the main message broker
///
/// Services can be registered here
pub struct ServiceBus<C, E>
where
    C: Clone + fmt::Debug + 'static,
    E: Clone + fmt::Debug + 'static,
{
    broker: Broker<C, E>,
    services: HashMap<Uuid, Box<dyn Service<C, E>>>,
}

impl<C, E> Default for ServiceBus<C, E>
where
    C: Clone + fmt::Debug + 'static,
    E: Clone + fmt::Debug + 'static,
{
    fn default() -> Self {
        Self {
            broker: Broker::default(),
            services: HashMap::new(),
        }
    }
}

impl<C, E> ServiceBus<C, E>
where
    C: Clone + fmt::Debug + 'static,
    E: Clone + fmt::Debug + 'static,
{
    /// Registers a service with the bus
    pub fn register_service(&mut self, service: impl Service<C, E> + 'static) -> uuid::Uuid {
        let uuid = uuid::Uuid::new_v4();
        log::info!("[Register] Service registered: {uuid:?}");
        self.services.insert(uuid, Box::new(service));
        uuid
    }

    /// Unregisters a service with the bus
    pub fn unregister_service(&mut self, uuid: &uuid::Uuid) {
        log::info!("[Unregister] Service unregistered: {uuid:?}");
        self.services.remove(uuid);
    }

    /// Publish an engine message to the broker
    pub fn publish_engine_message(&mut self, topic: &str, message: contract::EngineMessage<C, E>) {
        log::info!("[Publish] Topic: {topic} Message: {message:?}");
        self.broker.publish(topic, message);
    }

    /// Publish an engine command message to the broker
    pub fn publish_engine_command(&mut self, command: contract::EngineCommand) {
        self.publish_engine_message(
            ENGINE_COMMAND_TOPIC,
            contract::EngineMessage::EngineCommand { command },
        );
    }

    /// Publish an engine event message to the broker
    pub fn publish_engine_event(&mut self, event: EngineEvent) {
        self.publish_engine_message(
            ENGINE_EVENT_TOPIC,
            contract::EngineMessage::EngineEvent { event },
        );
    }

    /// Publish an app command message to the broker
    pub fn publish_app_command(&mut self, command: C) {
        self.publish_engine_message(
            APP_COMMAND_TOPIC,
            contract::EngineMessage::AppCommand { command },
        );
    }

    /// Publish an app event message to the broker
    pub fn publish_app_event(&mut self, event: E) {
        self.publish_engine_message(APP_EVENT_TOPIC, contract::EngineMessage::AppEvent { event });
    }

    /// Called continually to update all services
    pub fn update(&mut self) {
        self.services.values_mut().for_each(|service| {
            service.update(&mut self.broker);
        });
    }
}
