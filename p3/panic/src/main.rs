use std::fs;

fn read_number_from_file() -> Result<i32, std::io::Error> {
    let contents = fs::read_to_string("C:\\Users\\utabj\\Rust\\p3\\panic\\my_number.txt")?;
    let number = contents.trim().parse::<i32>();
    println!("Successfully read number from file");
    Ok(number.unwrap())
}


fn main() {
    println!("Hello, world!");

    read_number_from_file().unwrap();
}
