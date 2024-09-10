use visitor_pattern::{File, Directory, FileSystemElement, SizeCalculator, FileLister};

fn main() {
    // Create a file system structure
    let mut root = Directory::new("root");
    let mut home = Directory::new("home");
    let mut user = Directory::new("user");

    // Add files to directories
    user.add(Box::new(File {
        name: "file1.txt".to_string(),
        size: 1200,
    }));
    user.add(Box::new(File {
        name: "file2.txt".to_string(),
        size: 3400,
    }));

    home.add(Box::new(user));
    root.add(Box::new(home));

    // Create a visitor to calculate the total size
    let mut size_calculator = SizeCalculator::new();
    root.accept(&mut size_calculator);
    println!("Total size: {} bytes", size_calculator.total_size);

    // Create a visitor to list all files
    let mut file_lister = FileLister::new();
    root.accept(&mut file_lister);
}
