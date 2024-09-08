#[cfg(feature = "server")]
fn main() {
    simple_chat::server::main();
}

#[cfg(feature = "client")]
fn main() {
    simple_chat::client::main();
}
