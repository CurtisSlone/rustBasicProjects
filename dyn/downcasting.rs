use std::any::Any;

trait Speak: Any {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

fn print_speak(s: &dyn Speak) {
    // Box the reference to perform downcasting
    let s_boxed: Box<dyn Speak> = Box::new(s);
    
    if let Some(dog) = s_boxed.as_any().downcast_ref::<Dog>() {
        dog.speak();
    } else if let Some(cat) = s_boxed.as_any().downcast_ref::<Cat>() {
        cat.speak();
    } else {
        println!("Unknown type");
    }
}

// Helper function to allow downcasting
impl dyn Speak {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main(){
    let dog = Dog;
    let cat = Cat;

    let speakable_things: Vec<Box<dyn Speak>> = vec![Box::new(dog), Box::new(cat)];
    for obj in speakable_things {
        print_speak(&*obj);
    }
}
