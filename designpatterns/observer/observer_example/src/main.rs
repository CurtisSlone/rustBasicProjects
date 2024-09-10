use observer_pattern::{NewsChannel, User, Subject, Observer};
use std::sync::{Arc, Mutex};

fn main() {
    let mut channel = NewsChannel::new();

    let user1: Arc<Mutex<dyn Observer>> = Arc::new(Mutex::new(User::new("Alice")));
    let user2: Arc<Mutex<dyn Observer>> = Arc::new(Mutex::new(User::new("Bob")));
    

    channel.register_observer(Arc::clone(&user1));
    channel.register_observer(Arc::clone(&user2));

    channel.publish_news("Breaking news: World War III is about to begin!");
    channel.publish_news("International news: Ukraine is under attack!");

}