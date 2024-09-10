pub trait SupportHandler {
    fn handle_request(&self, level: u32, request: &str);
}

pub struct SupportAgent {
    next_handler: Option<Box<dyn SupportHandler>>,
}

impl SupportAgent {
    pub fn new(next_handler: Option<Box<dyn SupportHandler>>) -> Self {
        SupportAgent { next_handler }
    }
}

impl SupportHandler for SupportAgent {
    fn handle_request(&self, level: u32, request: &str) {
        if level <= 1 {
            println!("SupportAgent handling request: {}", request);
        } else if let Some(ref handler) = self.next_handler {
            handler.handle_request(level, request);
        } else {
            println!("SupportAgent: Request cannot be handled further.");
        }
    }
}

pub struct Manager {
    next_handler: Option<Box<dyn SupportHandler>>,
}

impl Manager {
    pub fn new(next_handler: Option<Box<dyn SupportHandler>>) -> Self {
        Manager { next_handler }
    }
}

impl SupportHandler for Manager {
    fn handle_request(&self, level: u32, request: &str) {
        if level <= 2 {
            println!("Manager handling request: {}", request);
        } else if let Some(ref handler) = self.next_handler {
            handler.handle_request(level, request);
        } else {
            println!("Manager: Request cannot be handled further.");
        }
    }
}

pub struct Director;

impl SupportHandler for Director {
    fn handle_request(&self, level: u32, request: &str) {
        if level <= 3 {
            println!("Director: Handling request: {}", request);
        } else {
            println!("Director: Request too complex to handle.");
        }
    }
}