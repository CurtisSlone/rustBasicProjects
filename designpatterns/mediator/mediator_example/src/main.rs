use mediator_pattern::{ChatRoomMediator, Mediator, User};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // Create the chat room (mediator) inside Rc<RefCell> for shared mutable access
    let chat_room = Rc::new(RefCell::new(ChatRoomMediator::new()));

    // Create users and register them to the chat room
    let user1 = Box::new(User::new("User1", Box::new(chat_room.clone())));
    let user2 = Box::new(User::new("User2", Box::new(chat_room.clone())));
    let user3 = Box::new(User::new("User3", Box::new(chat_room.clone())));

    // Register users to the chat room
    chat_room.borrow_mut().register(user1);
    chat_room.borrow_mut().register(user2);
    chat_room.borrow_mut().register(user3);

    // Simulate sending messages
    chat_room.borrow().send("User1", Some("User2"), "Hello, User2!");
    chat_room.borrow().send("User2", None, "Broadcast: Hello, everyone!");
}
