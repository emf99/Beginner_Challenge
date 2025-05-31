// TODO: 1. Define a function that adds two integers and returns the result
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// TODO: 2. Define a function that calculates the area of a rectangle
fn calculate_rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}

// TODO: 3. Define a function that checks if a number is prime
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// TODO: 4. Define a function that converts Fahrenheit to Celsius
// Formula: C = (F - 32) * 5/9
fn fahrenheit_to_celsius(f: f64) -> f64 {
     (f - 32.0) * 5.0 / 9.0
}

fn main() {
    // TODO: Call the addition function with different values and print the results
    let sum1 = add(3,6);
    let sum2 = add(4, 7);
    
    // TODO: Calculate and print the area of rectangles with different dimensions
    let area1 = calculate_rectangle_area(2.5, 6.7);
    let area2 = calculate_rectangle_area(4.9, 6.9);
    
    // TODO: Test your prime number checker with several numbers
    let prime_check1 = is_prime(123);
    let prime_check2 = is_prime(7867);
    
    // TODO: Convert and print some temperatures from Fahrenheit to Celsius
    let celsius1 = fahrenheit_to_celsius(124.00);
    let celsius2 = fahrenheit_to_celsius(150.00);
    let g =123.89;
    // TODO: Print all results with appropriate labels
    println!("Sum of 3 and 6 is: {}", sum1);
    println!("Area of rectangle with width 2.5 and height 6.7 is: {} square units", area1);
    println!("Is 123 a prime number? {}", prime_check1);
    println!("{}°F is equivalent to {}°C", g, celsius1);
}