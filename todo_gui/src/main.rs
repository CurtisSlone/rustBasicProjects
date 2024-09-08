use druid::widget::{Button, Flex, Label, List, TextBox};
use druid::{AppLauncher, Color, Data, Env, Lens, Widget, WidgetExt, WindowDesc};
use std::sync::Arc;

#[derive(Clone, Data, Lens)]
struct AppState {
    todo_items: Arc<Vec<String>>, // Wrap Vec<String> in Arc
    new_item: String,
}

fn build_ui() -> impl Widget<AppState> {
    let add_button = Button::new("Add Item")
        .on_click(|_ctx, data: &mut AppState, _env| {
            if !data.new_item.is_empty() {
                let mut items = (*data.todo_items).clone();
                items.push(data.new_item.clone());
                data.todo_items = Arc::new(items);
                data.new_item.clear();
            }
        });

    let new_item_textbox = TextBox::new()
        .with_placeholder("Enter new todo item")
        .lens(AppState::new_item);

    let todo_list = List::new(|| {
        Label::new(|item: &String, _env: &Env| item.clone())
            .padding(5.0)
            .background(Color::BLACK)
            .border(Color::BLACK, 1.0)
    });

    Flex::column()
        .with_child(new_item_textbox)
        .with_child(add_button)
        .with_child(todo_list.lens(AppState::todo_items))
        .padding(10.0)
        .background(Color::rgb8(0, 0, 0))
}

fn main() {
    let main_window = WindowDesc::new(|| build_ui())
        .title("Todo List")
        .window_size((400.0, 600.0));

    let initial_state = AppState {
        todo_items: Arc::new(Vec::new()),
        new_item: String::new(),
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(initial_state)
        .expect("Failed to launch application");
}
