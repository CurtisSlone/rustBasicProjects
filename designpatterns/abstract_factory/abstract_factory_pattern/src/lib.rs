pub trait Button {
    fn click(&self);
}

pub trait Checkbox {
    fn check(&self);
}

pub trait GUIFactory{
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}

pub struct WindowsButton;

impl Button for WindowsButton {
    fn click(&self) {
        println!("Windows button clicked!");
    }
}

pub struct WindowsCheckbox;

impl Checkbox for WindowsCheckbox {
    fn check(&self) {
        println!("Windows checkbox checked!");
    }
}

pub struct WindowsFactory;

impl GUIFactory for WindowsFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WindowsCheckbox)
    }
}

pub struct MacButton;

impl Button for MacButton {
    fn click(&self){
        println!("MacOS button clicked!");
    }
}

pub struct MacCheckbox;

impl Checkbox for MacCheckbox {
    fn check(&self) {
        println!("MacOS checkbox checked!");
    }
}

pub struct MacFactory;

impl GUIFactory for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacCheckbox)
    }
}