use state_pattern::VendingMachine;

fn main() {
    let mut machine = VendingMachine::new(3); // Vending machine with 3 items

    // Try dispensing without money
    machine.press_button();
    machine.dispense();

    // Insert money and try dispensing
    machine.insert_money();
    machine.press_button();
    machine.dispense();

    // Insert money and try dispensing again
    machine.insert_money();
    machine.press_button();
    machine.dispense();

    // Insert money and try when there is no more stock
    machine.insert_money();
    machine.press_button();
    machine.dispense();
}
