use abstract_factory_pattern::{GUIFactory, WindowsFactory, MacFactory};

fn main() {
    let windows_factory = WindowsFactory;
    let windows_button = windows_factory.create_button();
    let windows_checkbox = windows_factory.create_checkbox();

    windows_button.click();
    windows_checkbox.check();

    let mac_factory = MacFactory;
    let mac_button = mac_factory.create_button();
    let mac_checkbox = mac_factory.create_checkbox();

    mac_button.click();
    mac_checkbox.check();
}