use std::{
    cell::{Ref, RefCell},
    collections::VecDeque,
    rc::Rc,
};
use uuid::Uuid;

pub type ClientHandle<T> = Rc<RefCell<Client<T>>>;

pub struct Client<T: Clone> {
    id: Uuid,
    event_queue: RefCell<VecDeque<T>>,
    ring_buffer_size: usize,
}

impl<T: Clone> Default for Client<T> {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            event_queue: RefCell::new(VecDeque::new()),
            ring_buffer_size: 100,
        }
    }
}

impl<T: Clone> Client<T> {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::default()))
    }

    pub fn event_queue(&mut self) -> Ref<VecDeque<T>> {
        self.event_queue.borrow()
    }

    pub fn event_queue_mut(&mut self) -> std::cell::RefMut<VecDeque<T>> {
        self.event_queue.borrow_mut()
    }

    pub fn with_ring_buffer_size(size: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            ring_buffer_size: size,
            ..Default::default()
        }))
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn ring_buffer_size(&self) -> usize {
        self.ring_buffer_size
    }

    pub fn next_message(&self) -> Option<T> {
        self.event_queue.borrow_mut().pop_front()
    }

    pub fn peek_message(&self) -> Option<T> {
        self.event_queue.borrow().front().cloned()
    }
}
