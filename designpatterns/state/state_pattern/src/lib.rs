// Define the trait for State
pub trait State {
    fn insert_money(self: Box<Self>, machine: &mut VendingMachine) -> Box<dyn State>;
    fn press_button(self: Box<Self>, machine: &mut VendingMachine) -> Box<dyn State>;
    fn dispense(self: Box<Self>, machine: &mut VendingMachine) -> Box<dyn State>;
}

// The VendingMachine struct
pub struct VendingMachine {
    pub item_count: u32,
    pub has_money: bool,
    state: Option<Box<dyn State>>,
}

impl VendingMachine {
    pub fn new(item_count: u32) -> Self {
        VendingMachine {
            item_count,
            has_money: false,
            state: Some(Box::new(IdleState {})),
        }
    }

    pub fn insert_money(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.insert_money(self));
        }
    }

    pub fn press_button(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.press_button(self));
        }
    }

    pub fn dispense(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.dispense(self));
        }
    }
}

// Concrete state: IdleState
pub struct IdleState;

impl State for IdleState {
    fn insert_money(self: Box<Self>, machine: &mut VendingMachine) -> Box<dyn State> {
        println!("Money inserted.");
        machine.has_money = true;
        Box::new(HasMoneyState {})
    }

    fn press_button(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn State> {
        println!("You need to insert money first.");
        self
    }

    fn dispense(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn State> {
        println!("No money inserted. Can't dispense.");
        self
    }
}

// Concrete state: HasMoneyState
pub struct HasMoneyState;

impl State for HasMoneyState {
    fn insert_money(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn State> {
        println!("Money already inserted.");
        self
    }

    fn press_button(self: Box<Self>, machine: &mut VendingMachine) -> Box<dyn State> {
        if machine.item_count > 0 {
            println!("Button pressed. Dispensing item.");
            Box::new(DispensingState {})
        } else {
            println!("No items left to dispense.");
            machine.has_money = false;
            Box::new(IdleState {})
        }
    }

    fn dispense(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn State> {
        println!("Press the button first.");
        self
    }
}

// Concrete state: DispensingState
pub struct DispensingState;

impl State for DispensingState {
    fn insert_money(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn State> {
        println!("Please wait. Dispensing in progress.");
        self
    }

    fn press_button(self: Box<Self>, _machine: &mut VendingMachine) -> Box<dyn State> {
        println!("Already dispensing.");
        self
    }

    fn dispense(self: Box<Self>, machine: &mut VendingMachine) -> Box<dyn State> {
        if machine.item_count > 0 {
            println!("Item dispensed.");
            machine.item_count -= 1;
            machine.has_money = false;
            if machine.item_count > 0 {
                Box::new(IdleState {})
            } else {
                println!("Machine is out of items.");
                Box::new(IdleState {}) // Could transition to an "Out of Stock" state.
            }
        } else {
            println!("No items to dispense.");
            Box::new(IdleState {})
        }
    }
}
