static mut MEMORY_POOL: [u8; 1024] = [0; 1024];
static mut NEXT_FREE: usize = 0;

fn allocate(size: usize) -> Option<&'static mut [u8]> {
    unsafe {
        if NEXT_FREE + size > MEMORY_POOL.len() {
            None
        } else {
            let start_idx = NEXT_FREE; // Rename start to avoid conflicts
            NEXT_FREE += size;
            Some(&mut MEMORY_POOL[start_idx..start_idx + size])
        }
    }
}

fn main() {
    if let Some(mem) = allocate(100) {
        // Display first 10 elements or the size of allocated memory
        println!("Allocated 100 bytes: {:?}", &mem[..10.min(mem.len())]);
    }
    
    if let Some(mem) = allocate(200) {
        println!("Allocated 200 bytes: {:?}", &mem[..10.min(mem.len())]);
    } else {
        println!("Not enough memory for allocation 2");
    }
}
