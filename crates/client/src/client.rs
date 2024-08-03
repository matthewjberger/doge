use contract::EngineMessage;
use std::fmt;

pub type MessageClient<C, E> = broker::Client<EngineMessage<C, E>>;
pub type MessageClientHandle<C, E> = broker::ClientHandle<EngineMessage<C, E>>;

pub struct Client<C, E>
where
    C: Clone + fmt::Debug + 'static,
    E: Clone + fmt::Debug + 'static,
{
    uuid: uuid::Uuid,
    handle: MessageClientHandle<C, E>,
}

impl<C, E> Default for Client<C, E>
where
    C: Clone + fmt::Debug + 'static,
    E: Clone + fmt::Debug + 'static,
{
    fn default() -> Self {
        Self {
            uuid: uuid::Uuid::new_v4(),
            handle: MessageClient::new(),
        }
    }
}

impl<C, E> Client<C, E>
where
    C: Clone + fmt::Debug + 'static,
    E: Clone + fmt::Debug + 'static,
{
    pub fn id(&self) -> uuid::Uuid {
        self.uuid
    }

    pub fn subscribe_to_topic(
        &mut self,
        topic: &str,
        broker: &mut broker::Broker<EngineMessage<C, E>>,
    ) {
        broker.subscribe(topic, &self.handle);
    }

    pub fn publish(
        &mut self,
        topic: &str,
        message: EngineMessage<C, E>,
        broker: &mut broker::Broker<EngineMessage<C, E>>,
    ) {
        broker.publish(topic, message);
    }

    pub fn next_message(&mut self) -> Option<EngineMessage<C, E>> {
        self.handle.borrow_mut().next_message()
    }

    pub fn peek_message(&mut self) -> Option<EngineMessage<C, E>> {
        self.handle.borrow_mut().peek_message()
    }
}
