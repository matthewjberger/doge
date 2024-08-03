use crate::Client;
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};
use uuid::Uuid;

#[derive(Default)]
pub struct Broker<T: Clone> {
    subscribers: HashMap<String, Vec<Weak<RefCell<Client<T>>>>>,
}

impl<T: Clone> Broker<T> {
    pub fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, topic: &str, client: &Rc<RefCell<Client<T>>>) {
        let client_weak = Rc::downgrade(client);
        self.subscribers
            .entry(topic.to_string())
            .or_default()
            .push(client_weak);
    }

    pub fn unsubscribe(&mut self, topic: &str, client_id: Uuid) -> Result<(), &'static str> {
        if let Some(subscribers) = self.subscribers.get_mut(topic) {
            subscribers.retain(|subscriber| {
                if let Some(subscriber) = subscriber.upgrade() {
                    subscriber.borrow().id() != client_id
                } else {
                    false
                }
            });
            Ok(())
        } else {
            Err("TopicNotFound")
        }
    }

    pub fn publish(&mut self, topic: &str, message: T) {
        if let Some(subscribers) = self.subscribers.get_mut(topic) {
            // Use retain to filter out the expired weak references
            subscribers.retain(|subscriber_weak| {
                if let Some(subscriber_strong) = subscriber_weak.upgrade() {
                    let mut subscriber = subscriber_strong.borrow_mut();

                    // Access VecDeque methods by borrowing the RefCell
                    let ring_buffer_size = subscriber.ring_buffer_size();
                    if subscriber.event_queue().len() == ring_buffer_size {
                        subscriber.event_queue_mut().pop_front();
                    }
                    subscriber.event_queue_mut().push_back(message.clone());
                    true
                } else {
                    false // Drop the weak reference if it's no longer valid
                }
            });
            // Remove the topic entry if there are no subscribers left
            if subscribers.is_empty() {
                self.subscribers.remove(topic);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Broker, Client};

    #[derive(Debug, Clone, PartialEq)]
    pub struct Message {
        content: String,
    }

    impl Message {
        pub fn new(content: &str) -> Self {
            Self {
                content: content.to_string(),
            }
        }
    }

    #[test]
    fn test_single_client_receive_message() {
        let mut broker = Broker::new();
        let client1 = Client::new();
        broker.subscribe("topic1", &client1);
        broker.publish("topic1", Message::new("hello world"));
        assert_eq!(
            client1.borrow().next_message().unwrap().content,
            "hello world"
        );
    }

    #[test]
    fn test_multiple_subscribers_receive_message() {
        let mut broker = Broker::new();
        let client1 = Client::new();
        let client2 = Client::new();
        broker.subscribe("topic1", &client1);
        broker.subscribe("topic1", &client2);
        broker.publish("topic1", Message::new("hello world"));
        assert_eq!(
            client1.borrow().next_message().unwrap().content,
            "hello world"
        );
        assert_eq!(
            client2.borrow().next_message().unwrap().content,
            "hello world"
        );
    }

    #[test]
    fn test_unsubscribe() {
        let mut broker = Broker::new();
        let client1 = Client::new();
        let client2 = Client::new();
        broker.subscribe("topic1", &client1);
        broker.subscribe("topic1", &client2);
        broker.unsubscribe("topic1", client1.borrow().id()).unwrap();
        broker.publish("topic1", Message::new("hello world"));
        assert_eq!(client1.borrow().next_message(), None);
        assert_eq!(
            client2.borrow().next_message().unwrap().content,
            "hello world"
        );
    }

    #[test]
    fn test_multiple_topics() {
        let mut broker = Broker::new();
        let client = Client::new();
        broker.subscribe("topic1", &client);
        broker.subscribe("topic2", &client);
        broker.publish("topic1", Message::new("hello topic1"));
        broker.publish("topic2", Message::new("hello topic2"));
        assert_eq!(
            client.borrow().next_message().unwrap().content,
            "hello topic1"
        );
        assert_eq!(
            client.borrow().next_message().unwrap().content,
            "hello topic2"
        );
    }

    #[test]
    fn test_ring_buffer() {
        let mut broker = Broker::new();
        let client = Client::with_ring_buffer_size(2); // set ring buffer size to 2
        broker.subscribe("topic1", &client);
        broker.publish("topic1", Message::new("message1"));
        broker.publish("topic1", Message::new("message2"));
        broker.publish("topic1", Message::new("message3"));
        // Expecting the oldest message to be discarded due to ring buffer
        assert_eq!(client.borrow().next_message().unwrap().content, "message2");
        assert_eq!(client.borrow().next_message().unwrap().content, "message3");
    }

    #[test]
    fn usage_example() {
        // Create a new broker
        let mut broker = Broker::new();

        // Create a client with a specified ring buffer size
        let client = Client::with_ring_buffer_size(5);

        // Subscribe the client to a topic
        broker.subscribe("news", &client);

        // The broker publishes a message to the topic
        broker.publish("news", Message::new("Breaking news!"));

        // The client retrieves the message from its ring buffer
        assert_eq!(
            client.borrow().next_message().unwrap().content,
            "Breaking news!"
        );
    }

    #[test]
    fn test_weak_reference_cleanup() {
        let mut broker = Broker::new();

        // Subscribe a client to a topic
        {
            let client = Client::new();
            broker.subscribe("topic1", &client);

            // Ensure there's a subscriber for "topic1"
            assert!(broker.subscribers.contains_key("topic1"));

            // Simulating a message publish
            broker.publish("topic1", Message::new("Test"));

            // Ensure the client received the message
            assert_eq!(client.borrow().next_message().unwrap().content, "Test");
        } // Client goes out of scope here and should be dropped

        // The weak reference to the client should not be upgradeable now
        if let Some(subscribers) = broker.subscribers.get("topic1") {
            assert!(subscribers[0].upgrade().is_none());
        } else {
            panic!("Topic 'topic1' should still exist at this point.");
        }

        // Simulating another publish to trigger the weak reference cleanup
        broker.publish("topic1", Message::new("Test 2"));

        // Check if weak reference cleanup worked by checking the subscribers for "topic1"
        assert!(!broker.subscribers.contains_key("topic1"));
    }

    #[test]
    fn test_peek_message() {
        let mut broker = Broker::new();
        let client = Client::new();
        broker.subscribe("topic1", &client);
        broker.publish("topic1", Message::new("peek this"));

        // Peek the message
        assert_eq!(client.borrow().peek_message().unwrap().content, "peek this");

        // Check if the message is still in the queue
        assert_eq!(client.borrow().next_message().unwrap().content, "peek this");

        // Ensure the message queue is now empty after calling `next_message`
        assert!(client.borrow().next_message().is_none());
    }
}
