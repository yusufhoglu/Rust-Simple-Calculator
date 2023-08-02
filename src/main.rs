use std::io;
fn main() {
    let mut input = String::new();

    // Get the first number
    println!("Enter the first number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let x: f64 = input.trim().parse().expect("Invalid input. Please enter a valid number.");
    // Get the operation
    input.clear();
    println!("Enter the operation (+, -, *, /):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let operation_type:i32 = match input.trim().chars().next() {
        Some('+') => 1,
        Some('-') => 2,
        Some('*') => 3,
        Some('/') => 4,
        _ => {
            println!("Invalid operation. Exiting...");
            return;
        }
    };

    // Get the second number
    input.clear();
    println!("Enter the second number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let y: f64 = input.trim().parse().expect("Invalid input. Please enter a valid number.");
    // Calculate and print the result 
    let result = match operation_type {
        1 => Operation::Add { x: (x), y: (y) },
        2 => Operation::Subtract{ x: (x), y: (y) },
        3 => Operation::Multiply { x: (x), y: (y) },
        4 => Operation::Divide { x: (x), y: (y) },
        _ => {
            println!("Invalid operation. Exiting...");
            return;
        }
    };
    print!("{}",result.calculate());


}

enum Operation {
    Add {x:f64,y:f64},
    Subtract {x:f64,y:f64},
    Multiply {x:f64,y:f64},
    Divide {x:f64,y:f64}
}

impl Operation {
    fn calculate(&self) -> f64 {
        match self {
            Operation::Add {x, y} => x+y,           
            Operation::Divide { x, y } =>x/y,
            Operation::Multiply { x, y } => x*y,
            Operation::Subtract { x, y } => x-y
        }
    }
}

