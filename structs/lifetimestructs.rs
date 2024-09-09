struct StringWrapper<'a> {
    data: &'a str,
}

fn main() {
    let text = String::from("Hello, Rust!");
    let wrapper = StringWrapper { data: &text };
    println!("Wrapped string: {}", wrapper.data);
}
