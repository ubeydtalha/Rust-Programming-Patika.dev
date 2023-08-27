trait Display {
    fn display(&self) -> String;
}

trait Printable {
    fn print(&self);
}


struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Display for Circle {
    fn display(&self) -> String {
        format!(
            "Circle of radius {}",
            self.radius
        )
    }
}


impl Display for Rectangle {
    fn display(&self) -> String {
        format!(
            "Rectangle of width {} and height {}",
            self.width,
            self.height
        )
    }
}

impl Display for String {

    fn display(&self) -> String {
        self.clone()
    }
    
}

impl Printable for Circle {
    fn print(&self) {
        println!("Circle of radius {}", self.radius);
    }
}

impl Printable for Rectangle {
    fn print(&self) {
        println!("Rectangle of width {} and height {}", self.width, self.height);
    }
}

impl Printable for String {
    fn print(&self) {
        println!("{}", self);
    }
}


fn print_something<T: Printable>(item : &T){
    item.print();
}



trait  Animal {
    fn speak(&self);
}

trait Mammal : Animal {
    fn walk(&self);
}

struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Mammal for Dog {
    fn walk(&self) {
        println!("Walking");
    }
}









fn main() {
    
    let c = Circle { radius: 6.0 };
    let r = Rectangle { width: 4.0, height: 5.0 };
    let s = String::from("Hello World");
    
    println!("{}", c.display());
    println!("{}", r.display());
    println!("{}", s.display());
    
    print_something(&c);
    print_something(&r);
    print_something(&s);
    
    let d = Dog;
    d.speak();
    d.walk();



}
