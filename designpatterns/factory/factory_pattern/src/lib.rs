pub trait Widget {
    fn draw(&self);
}

pub struct Button {
    label: String,
}

impl Widget for Button {
    fn draw(&self) {
        println!("Drawing a button: {}", self.label);
    }
}

impl Button {
    pub fn new(label: &str) -> Self {
        Button { label: label.to_string(), }
    }
}

pub struct TextBox {
    placeholder: String,
}

impl Widget for TextBox {
    fn draw(&self) {
        println!("Drawing a text box with placeholder: {}", self.placeholder);
    }
}

impl TextBox {
    pub fn new(placeholder: &str) -> Self {
        TextBox { placeholder: placeholder.to_string(), }
    }
}

pub enum WidgetType {
    Button,
    TextBox,
}

pub struct WidgetFactory;

impl WidgetFactory  {
    pub fn create_widget(widget_type: WidgetType, text: &str) -> Box<dyn Widget> {
        match widget_type {
            WidgetType::Button => Box::new(Button::new(text)),
            WidgetType::TextBox => Box::new(TextBox::new(text)),
        }
    }
}
