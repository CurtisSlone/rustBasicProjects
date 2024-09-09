trait Cloneable {
    fn clone_box(&self) -> Box<dyn Cloneable>;
}

impl<T> Cloneable for T
where
    T: 'static + Clone,
    {
        fn clone_box(&self) -> Box<dyn Cloneable> {
            Box::new(self.clone())
        }
    }

    #[derive(Clone)]
    struct MyStruct {
        value: i32,
    }

    fn main() {
        let my_struct = MyStruct { value: 10 };
        let cloned1: Box<dyn Cloneable> = my_struct.clone_box();
        let cloned2: Box<dyn Cloneable> = cloned1.clone_box();

    }