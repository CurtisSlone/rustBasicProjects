struct Book<'a> {
    title: &'a str,
    author: &'a str,
}

trait Describable {
    fn describe(&self) -> String;
}

impl<'a> Describable for Book<'a> {
    fn describe(&self) -> String {
        format!("The book '{}' by {}.", self.title, self.author)
    }
}

fn main() {
    let book = Book {
        title: "To Kill a Mockingbird",
        author: "Harper Lee",
    };

    println!("{}", book.describe());
}
