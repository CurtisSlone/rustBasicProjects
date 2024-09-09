use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

fn main() {
    let layout = Layout::new::<u64>();
    unsafe{
        let ptr = alloc(layout) as *mut u64;
        if ptr.is_null() {
            panic!(" Memory allocation failed");
        }

        ptr::write(ptr,42);
        println!("Value at ptr: {}:{:p}", *ptr,ptr);

        dealloc(ptr as *mut u8, layout);
    }
}