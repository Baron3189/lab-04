use std::io;

fn main() {
    let weight: f64;
    let total: f64;
    let mut input = String::new();

    println!("Enter the weight:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    weight = input.trim().parse().expect("weight-Not a number");

    if weight <= 1.0 {
        total = 5.0;
        print!("less than 1: {total}")
    } else if weight <= 5.0 {
        total = 12.0;
        print!("less than 12: {total}");
    } else if weight <= 20.0 {
        total = 25.0;
        print!("less than 25: {total}");
    } else if weight > 20.0 {
        total = 50.0;
        print!("greater than 50: {total}");
    }

    let mut destination = String::new();
    println!("Enter destination: 1. Domestic 2. International:");
    io::stdin()
        .read_line(&mut destination)
        .expect("Failed to read line");
    let destination: i32 = destination
        .trim()
        .parse()
        .expect("Input is not a valid integer");

    let mut tier = String::new();
    println!("Express Shipping? (y/n):");
    io::stdin()
        .read_line(&mut tier)
        .expect("Failed to read line");

    let mut customer_tier = String::new();
    println!("Enter customer tier: 1. standard 2. member 3. premium:");
    io::stdin()
        .read_line(&mut customer_tier)
        .expect("Failed to read line");
    let destination: i32 = customer_tier
        .trim()
        .parse()
        .expect("Input is not a valid integer");
}
