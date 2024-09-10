use std::fs::File;
use std::io::ErrorKind;

fn main(){
    let file_name = "non_existent_file.txt";

    let file = File::open(file_name);

    let _file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                eprintln!("The file {} does not exist.", file_name);
                File::create(file_name).unwrap_or_else(|_| {
                    panic!("Failed to create file: {}", file_name);
                })
            }
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}