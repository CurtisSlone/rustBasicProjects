use std::process;

fn main() {
    let success = true;
    if success {
        process::exit(0);
    } else {
        process::exit(1);
    }
}