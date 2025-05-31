use std::io;

fn main() {
    // Part 1: FizzBuzz Implementation
    println!("=== FizzBuzz Challenge ===");
    
    // TODO: Implement the FizzBuzz algorithm for numbers 1 to 20
    for i in 1..=20 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
    
    // Part 2: Menu-driven Calculator
    println!("\n=== Calculator ===");
    
    // TODO: Create a variable to control the calculator loop
    let mut running = true;
    
    // TODO: Implement the calculator loop
    while running {
        // TODO: Show the menu options
        println!("Choose an operation:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit");
        println!("Enter your choice (1-5): ");
        // TODO: Get the user's choice
        let mut choice = String::new();
        // TODO: Read user input
        io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input.");
        let choice = choice.trim();
        
        // TODO: Convert choice to a number (with error handling)

        let choice_int: u32 = match choice.parse() {
            Ok(num) => num, 
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 5.:");
                continue;
            }
        };

       
        
        // TODO: Exit if the user chose option 5
        if choice_int == 5 {
            running = false;
            // TODO: Set running to false to exit the loop
            break;
        }
        
        
        // TODO: Get the two input numbers from the user
        // First number
        // TODO: Read first number
        
        // Second number
        // TODO: Read second number
        
        // TODO: Perform the selected operation using match or if statements
        // match choice {
        //    1 => // Handle addition
        //    2 => // Handle subtraction
        //    3 => // Handle multiplication
        //    4 => // Handle division (remember to check for division by zero)
        //    _ => println!("Invalid option. Please try again."),
        // }
        match choice.trim(){
            "1" => perform_operation("+"),
            "2" => perform_operation("-"),
            "3" => perform_operation("*"),
            "4" => perform_operation("/"),
            
            _ => println!("Invalid option. Please try again"),
        }
        
        // TODO: Ask if the user wants to perform another calculation
        println!("Do you want to perform another calculation? (y/n): ");
        // TODO: Read user's response
        let mut choice2 = String::new();
        // TODO: Read user input
        io::stdin()
        .read_line(&mut choice2)
        .expect("Failed to read input.");
        if choice2.trim().to_lowercase() != "y" {
        // TODO: Set running to false if the user doesn't want to continue
            running = false;
            }
    
    println!("Thank you for using the calculator!");
    }
}
fn perform_operation(op: &str) {
    println!("Enter the first number: ");
        let mut first_num = String::new();
        // TODO: Read user input
        io::stdin()
        .read_line(&mut first_num)
        .expect("Failed to read input.");
        // TODO: Convert choice to a number (with error handling)
        let first_num_int : u32 = first_num.trim().parse().unwrap(); 
        println! ("Enter the second number: ");
       
        let mut second_num = String::new();
        // TODO: Read user input
        io::stdin()
        .read_line(&mut second_num)
        .expect("Failed to read input.");
        // TODO: Convert choice to a number (with error handling)
        let second_num_int : u32 = second_num.trim().parse().unwrap();
    

    let result = match op {
        "+" => { first_num_int + second_num_int }
        "-" => { first_num_int - second_num_int }
        "*" => { first_num_int * second_num_int }
        "/" => {
            if second_num_int == 0 {
                println!("Error: Cannot divide by zero.");
                return;
            } else {
                first_num_int / second_num_int
            }
        }
        _ => {
            println!("Unknown operation.");
            return;
        }
    };

    println!("Result: {:.2}", result);
}