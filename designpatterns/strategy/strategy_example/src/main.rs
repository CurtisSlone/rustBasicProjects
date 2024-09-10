use strategy_pattern::{TaxCalculator, TaxStrategy, USTax, CanadianTax, UKTax};

fn main() {
    let mut calculator = TaxCalculator::new(Box::new(USTax));

    let amount = 1000.0;

    println!("Tax for U.S.: {}", calculator.calculate_total_tax(amount));

    calculator.set_strategy(Box::new(CanadianTax));
    println!("Tax for Canada: {}", calculator.calculate_total_tax(amount));

    calculator.set_strategy(Box::new(UKTax));
    println!("Tax for UK: {}", calculator.calculate_total_tax(amount));
}