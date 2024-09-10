use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

// Mediator trait defining the communication interface
pub trait Mediator {
    fn send(&self, sender_id: &str, receiver_id: Option<&str>, message: &str);
    fn register(&mut self, user: Box<dyn Colleague>);
}

// Colleague trait representing participants in the mediator pattern
pub trait Colleague {
    fn receive(&self, sender_id: &str, message: &str);
    fn get_id(&self) -> &str;
}

// Concrete Mediator: ChatRoomMediator
pub struct ChatRoomMediator {
    users: HashMap<String, Box<dyn Colleague>>,
}

impl ChatRoomMediator {
    pub fn new() -> Self {
        ChatRoomMediator {
            users: HashMap::new(),
        }
    }
}

impl Mediator for ChatRoomMediator {
    fn send(&self, sender_id: &str, receiver_id: Option<&str>, message: &str) {
        match receiver_id {
            Some(receiver) => {
                if let Some(user) = self.users.get(receiver) {
                    user.receive(sender_id, message);
                } else {
                    println!("User {} not found.", receiver);
                }
            }
            None => {
                for (id, user) in &self.users {
                    if id != sender_id {
                        user.receive(sender_id, message);
                    }
                }
            }
        }
    }

    fn register(&mut self, user: Box<dyn Colleague>) {
        let user_id = user.get_id().to_string();
        self.users.insert(user_id, user);
    }
}

// Implement Mediator for Rc<RefCell<ChatRoomMediator>>
impl Mediator for Rc<RefCell<ChatRoomMediator>> {
    fn send(&self, sender_id: &str, receiver_id: Option<&str>, message: &str) {
        self.borrow().send(sender_id, receiver_id, message);
    }

    fn register(&mut self, user: Box<dyn Colleague>) {
        self.borrow_mut().register(user);
    }
}

// Concrete Colleague: User
pub struct User {
    pub id: String,
    pub mediator: Box<dyn Mediator>,
}

impl User {
    pub fn new(id: &str, mediator: Box<dyn Mediator>) -> Self {
        User {
            id: id.to_string(),
            mediator,
        }
    }

    pub fn send(&self, receiver_id: Option<&str>, message: &str) {
        self.mediator.send(&self.id, receiver_id, message);
    }
}

impl Colleague for User {
    fn receive(&self, sender_id: &str, message: &str) {
        println!("{} received a message from {}: {}", self.id, sender_id, message);
    }

    fn get_id(&self) -> &str {
        &self.id
    }
}
