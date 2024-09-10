trait Speak {
    fn speak(&self);
}

#[derive(Clone)]
struct Dog;
#[derive(Clone)]
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

fn speak_static<T: Speak>(animal: T){
    animal.speak();
}

fn speak_dynamic(animal: &dyn Speak) {
    animal.speak();
}

fn main() {
    let dog = Dog;
    let cat = Cat;
    speak_static(dog.clone());
    speak_static(cat.clone());
    speak_dynamic(&dog);
    speak_dynamic(&cat);
}