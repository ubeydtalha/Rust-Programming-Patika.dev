trait Drawable {
    fn draw(&self);
}

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle with width {} and height {}", self.width, self.height);
    }
}

fn draw_object(d: &dyn Drawable){
    d.draw();
}


impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}





fn main() {
    
    let c = Circle { radius: 2.0 };
    let r = Rectangle { width: 2.0, height: 3.0 };

    draw_object(&c);
    draw_object(&r);

    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
    shapes.push(Box::new(
        Circle { radius: 2.0 }
    ));
    shapes.push(Box::new(
        Rectangle { width: 2.0, height: 3.0 }
    ));

    for shape in shapes{
        println!("Area: {}", shape.area());
    }



}
