rustc fibonacci.rs -C opt-level=0   # No optimization
./fibonacci

rustc fibonacci.rs -C opt-level=3   # High optimization
./fibonacci
