fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let had_greater_than_3 = v.iter().any(|x| *x > 3);
    println!("Greater than 3: {}", had_greater_than_3);

    let all_greater_than_3 = v.iter().all(|x| *x > 3);
    println!("All greater than 3: {}", all_greater_than_3);

    let count = v.iter().count();
    println!("Count: {}", count);

    let sum: i32 = v.iter().sum();
    println!("Sum: {}", sum);

    let min = v.iter().min();
    println!("Min: {:?}", min);

    let max = v.iter().max();
    println!("Max: {:?}", max);

    let collected: Vec<_> = v.iter().collect();
    println!("Collected: {:?}", collected);

}