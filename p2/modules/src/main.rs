
mod fruits {

    pub mod apple {
        pub fn print_apple() {
            println!("Apple");
        }
    }

    pub mod mango {
        pub fn print_mango() {
            println!("Mango");
        }
    }

}

use fruits::mango;

fn main() {
    
    // Absolute path
    crate::fruits::apple::print_apple();

    // Relative path
    fruits::mango::print_mango();

    // Using use
    mango::print_mango();
}
