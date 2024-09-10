rustc --crate-type=lib mylib.rs

rustc main.rs --extern mylib=libmylib.rlib
.\main.exe
