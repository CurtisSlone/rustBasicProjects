// Define the Visitor trait
pub trait FileSystemVisitor {
    fn visit_file(&mut self, file: &File);
    fn visit_directory(&mut self, directory: &Directory);
}

// Define the Element trait for file system elements
pub trait FileSystemElement {
    fn accept(&self, visitor: &mut dyn FileSystemVisitor);
}

// Concrete Element: File
pub struct File {
    pub name: String,
    pub size: u64,
}

impl FileSystemElement for File {
    fn accept(&self, visitor: &mut dyn FileSystemVisitor) {
        visitor.visit_file(self);
    }
}

// Concrete Element: Directory
pub struct Directory {
    pub name: String,
    pub children: Vec<Box<dyn FileSystemElement>>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Directory {
            name: name.to_string(),
            children: Vec::new(),
        }
    }

    pub fn add(&mut self, element: Box<dyn FileSystemElement>) {
        self.children.push(element);
    }
}

impl FileSystemElement for Directory {
    fn accept(&self, visitor: &mut dyn FileSystemVisitor) {
        visitor.visit_directory(self);
    }
}

// Concrete Visitor: SizeCalculator
pub struct SizeCalculator {
    pub total_size: u64,
}

impl SizeCalculator {
    pub fn new() -> Self {
        SizeCalculator { total_size: 0 }
    }
}

impl FileSystemVisitor for SizeCalculator {
    fn visit_file(&mut self, file: &File) {
        println!("Calculating size for file: {}", file.name);
        self.total_size += file.size;
    }

    fn visit_directory(&mut self, directory: &Directory) {
        println!("Entering directory: {}", directory.name);
        for child in &directory.children {
            child.accept(self); // Traverse into the directory
        }
    }
}

// Concrete Visitor: FileLister
pub struct FileLister;

impl FileLister {
    pub fn new() -> Self {
        FileLister
    }
}

impl FileSystemVisitor for FileLister {
    fn visit_file(&mut self, file: &File) {
        println!("File: {}", file.name);
    }

    fn visit_directory(&mut self, directory: &Directory) {
        println!("Directory: {}", directory.name);
        for child in &directory.children {
            child.accept(self);
        }
    }
}
