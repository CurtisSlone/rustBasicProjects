struct Text { characters: String}

impl Text {
    fn from(text: &str) -> Text {
        Text { characters: text.to_string() }
    }

    fn draw(&self){
        print!("{}", self.characters)
    }
}

struct BoxedText {
    text: Text,
    first: char,
    last: char,
}

impl BoxedText {
    fn with_text_and_borders(text: &str, first: char, last: char) -> BoxedText {
        BoxedText {
            text: Text::from(text),
            first,
            last,
        }
    }
    fn draw(&self){
        print!("{}", self.first);
        self.text.draw();
        print!("{}", self.last);
    }
}

fn main() {
    let greeting = Text::from("Hello");
    greeting.draw();  // Output: Hello

    let boxed_greeting = BoxedText::with_text_and_borders("Hello", '*', '*');
    print!(",");
    boxed_greeting.draw();
}