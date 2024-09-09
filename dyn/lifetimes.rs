trait Describable{
    fn describe(&self) -> String;
}

struct Book {
    title: String,
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!("The book is called '{}'.", self.title)
    }
}

fn print_description(item: &dyn Describable) {
    println!("{}", item.describe());
}

fn main() {
    let book = Book { title: String::from("To Kill a Mockingbird") };
    print_description(&book);
}