static GLOBAL_VAR: i32 = 100;

fn main() {
    let stack_var: i32 = 50;
    println!("Global variable: {}", GLOBAL_VAR);
    println!("Stack variable: {}", stack_var);
}