
struct Empty;

fn main() {
    let empty_instance = Empty;
    empty_instance.greet();
    println!("Hello, world!");
}


impl Empty {

    fn greet(&self) {
        println!("Hello");
    }
}