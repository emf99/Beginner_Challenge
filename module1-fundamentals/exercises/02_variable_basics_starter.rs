fn main() {
    // TODO: 1. Declare an immutable integer variable
    
    
    
    let my_integer = 10;



    // TODO: 2. Declare a mutable float variable and modify it later
    let mut my_float = 10.2;
    let z =my_float;
    // TODO: Modify the float value
    my_float = 20.5;
    // TODO: 3. Declare a boolean variable using type inference
    let is_rust_fun:bool = true;
    
    // TODO: 4. Declare a character variable with explicit type annotation
    let my_char: char ='R';
    // TODO: 5. Perform arithmetic operations with the numeric variables
    let sum = my_integer as f32 + my_float;
    let product = my_integer as f32 * my_float;
    
    // TODO: 6. Print all variables and calculation results with appropriate labels
    println!("Integer value: {}", my_integer);
    println!("Original float value: {}", z);
    println!("Modified float value: {}", my_float);
    println!("Boolean value: {}", is_rust_fun);
    println!("Character value: {}", my_char);
    println!("Addition result: {}", sum);
    println!("Multiplication result: {}", product);
}