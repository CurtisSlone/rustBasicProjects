/*
Some types, like file handles or network sockets, 
represent resources that should not be duplicated, 
hence they cannot implement the Clone trait.
*/

struct FileHandle {
    name: String,
}

impl FileHandle {
    fn new(name: &str) -> Self {
        FileHandle { name: String::from(name) }
    }
}

fn main() {
    let fh = FileHandle:::new("myfile.txt");
    let cloned_fh = fh.clone(); // This will fail because FileHandle does not implement Clone trait
}