pub trait TaxStrategy {
    fn calculate_tax(&self, amount: f64) -> f64;
}

pub struct USTax;

impl TaxStrategy for USTax {
    fn calculate_tax(&self, amount: f64) -> f64 {
        amount * 0.08
    }
}

pub struct CanadianTax;

impl TaxStrategy for CanadianTax {
    fn calculate_tax(&self, amount: f64) -> f64 {
        amount * 0.05
    }
}

pub struct UKTax;

impl TaxStrategy for UKTax {
    fn calculate_tax(&self, amount: f64) -> f64 {
        amount * 0.12
    }
}

pub struct TaxCalculator {
    tax_strategy: Box<dyn TaxStrategy>,
}

impl TaxCalculator {
    pub fn new(tax_strategy: Box<dyn TaxStrategy>) -> Self {
        TaxCalculator { tax_strategy }
    }

    pub fn set_strategy(&mut self, tax_strategy: Box<dyn TaxStrategy>) {
        self.tax_strategy = tax_strategy;
    }

    pub fn calculate_total_tax(&self, amount: f64) -> f64 {
        self.tax_strategy.calculate_tax(amount)
    }
}