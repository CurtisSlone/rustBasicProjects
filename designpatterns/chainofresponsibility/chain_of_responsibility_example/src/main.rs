use chain_of_responsibility_pattern::{SupportAgent, Manager, Director, SupportHandler};

fn main() {
    // Create the chain of responsibility
    let director = Director;
    let manager = Manager::new(Some(Box::new(director)));
    let agent = SupportAgent::new(Some(Box::new(manager)));

    // Test the chain with different levels of requests
    println!("--- Request Level 1 ---");
    agent.handle_request(1, "Basic support question");

    println!("--- Request Level 2 ---");
    agent.handle_request(2, "Intermediate support question");

    println!("--- Request Level 3 ---");
    agent.handle_request(3, "Advanced support question");

    println!("--- Request Level 4 ---");
    agent.handle_request(4, "Critical system failure");
}
