pub trait Coffee {
    fn cost(&self) -> f64;
    fn description(&self) -> String;
}

pub struct SimpleCoffee;

impl Coffee for SimpleCoffee {
    fn cost(&self) -> f64 {
        1.50
    }

    fn description(&self) -> String {
        "A simple coffee".to_string()
    }
}

pub struct MilkDecorator {
    coffee: Box<dyn Coffee>,
}

impl MilkDecorator {
    pub fn new(coffee: Box<dyn Coffee>) -> Self {
        MilkDecorator { coffee }
    }
}

impl Coffee for MilkDecorator {
    fn cost(&self) -> f64 {
        self.coffee.cost() + 0.25
    }

    fn description(&self) -> String {
        format!("{} with milk", self.coffee.description())
    }
}

pub struct SugarDecorator {
    coffee: Box<dyn Coffee>,
}

impl SugarDecorator {
    pub fn new(coffee: Box<dyn Coffee>) -> Self {
        SugarDecorator { coffee }
    }
}

impl Coffee for SugarDecorator {
    fn cost(&self) -> f64 {
        self.coffee.cost() + 0.15
    }

    fn description(&self) -> String {
        format!("{} with sugar", self.coffee.description())
    }
}