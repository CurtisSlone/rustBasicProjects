use factory_pattern::{WidgetFactory, WidgetType};

fn main(){
    let button = WidgetFactory::create_widget(WidgetType::Button, "Click me");
    button.draw();

    let text_box = WidgetFactory::create_widget(WidgetType::TextBox, "Enter your text");
    text_box.draw();
}