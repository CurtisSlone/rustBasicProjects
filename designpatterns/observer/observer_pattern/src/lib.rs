use std::sync::{Arc, Mutex};
use std::cell::RefCell;

pub trait Observer {
    fn update(&self, message: &str);
}

pub trait Subject {
    fn register_observer(&mut self, observer: Arc<Mutex<dyn Observer>>);
    fn notify_observers(&self, message: &str);
}

pub struct NewsChannel {
    observers: Vec<Arc<Mutex<dyn Observer>>>,
    latest_news: RefCell<String>,
}

impl NewsChannel {
    pub fn new() -> Self {
        NewsChannel {
            observers: Vec::new(),
            latest_news: RefCell::new(String::new()),
        }
    }

    pub fn publish_news(&self, news: &str) {
        *self.latest_news.borrow_mut() = news.to_string();
        self.notify_observers(news);
    }
}

impl Subject for NewsChannel {
    fn register_observer(&mut self, observer: Arc<Mutex<dyn Observer>>) {
        self.observers.push(observer);
    }

    fn notify_observers(&self, message: &str) {
        for observer in &self.observers {
            let objs = observer.lock().unwrap();
            objs.update(message);
        }
    }
}

pub struct User {
    name: String,
}

impl User {
    pub fn new(name: &str) -> Self {
        User { name: name.to_string(), }
    }
}

impl Observer for User {
    fn update(&self, message: &str) {
        println!("{} received news: {}", self.name, message);
    }
}