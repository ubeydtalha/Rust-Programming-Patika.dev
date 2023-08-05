
enum Operation {

    Add(f64,f64),
    Subtract(f64,f64),
    Multiply(f64,f64),
    Divide(f64,f64),
}

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add( x,y ) => x + y,
        Operation::Subtract(x,y ) => x - y,
        Operation::Multiply(x,y ) => x * y,
        Operation::Divide(x,y ) => x / y,
    }
}

fn main() {
    
    // user input from keyboard first number
    let mut first_number = String::new();
    println!("Enter first number: ");
    std::io::stdin().read_line(&mut first_number).expect("Failed to read line");
    let first_number: f64 = first_number.trim().parse().expect("Please type a number!");

    // user input from keyboard second number
    let mut second_number = String::new();
    println!("Enter second number: ");
    std::io::stdin().read_line(&mut second_number).expect("Failed to read line");
    let second_number: f64 = second_number.trim().parse().expect("Please type a number!");

    // user input from keyboard operation
    let mut operation = String::new();
    println!("Enter operation: ");
    std::io::stdin().read_line(&mut operation).expect("Failed to read line");
    let operation: char = operation.trim().parse().expect("Please type a number!");

    // match operation
    let result = match operation {
        '+' => calculate(Operation::Add(first_number,second_number)),
        '-' => calculate(Operation::Subtract(first_number,second_number)),
        '*' => calculate(Operation::Multiply(first_number,second_number)),
        '/' => calculate(Operation::Divide(first_number,second_number)),
        _ => panic!("Invalid operation"),
    };

    println!("Result: {}", result);




}

