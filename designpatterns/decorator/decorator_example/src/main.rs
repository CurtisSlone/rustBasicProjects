use decorator_pattern::{Coffee, MilkDecorator, SugarDecorator, SimpleCoffee};

fn main(){
    let coffee = SimpleCoffee;
    println!("Coffee description: {}", coffee.description());
    println!("Coffee cost: ${:2}", coffee.cost());

    let coffee_with_milk = MilkDecorator::new(Box::new(coffee));
    println!("Coffee with milk description: {}", coffee_with_milk.description());
    println!("Coffee with milk cost: ${:2}", coffee_with_milk.cost());

    let coffee_with_sugar = SugarDecorator::new(Box::new(coffee_with_milk));
    println!("Coffee with milk and sugar description: {}", coffee_with_sugar.description());
    println!("Coffee with milk and sugar cost: ${:2}", coffee_with_sugar.cost());
}