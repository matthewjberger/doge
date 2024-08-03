use contract::Message;

pub type MessageClient = broker::Client<Message>;
pub type MessageClientHandle = broker::ClientHandle<Message>;

pub struct Client {
    uuid: uuid::Uuid,
    handle: MessageClientHandle,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            uuid: uuid::Uuid::new_v4(),
            handle: MessageClient::new(),
        }
    }
}

impl Client {
    pub fn id(&self) -> uuid::Uuid {
        self.uuid
    }

    pub fn subscribe_to_topic(&mut self, topic: &str, broker: &mut broker::Broker<Message>) {
        broker.subscribe(topic, &self.handle);
    }

    pub fn publish(&mut self, topic: &str, message: Message, broker: &mut broker::Broker<Message>) {
        broker.publish(topic, message);
    }

    pub fn next_message(&mut self) -> Option<Message> {
        self.handle.borrow_mut().next_message()
    }

    pub fn peek_message(&mut self) -> Option<Message> {
        self.handle.borrow_mut().peek_message()
    }
}
