macro_rules! debug_print {
    ($val:expr) => {
        #[cfg(debug_assertions)]
        {
            println!("Debug: {:?} = {:?}", stringify!($val), $val);
        }
    };
}

fn main(){
    let x = 5;
    debug_print!(x + 2);
}