#[derive(Debug)]
struct Point {
    x : i64,
    y : i64,
}


fn main() {
    println!("Hello, world!");
    let mut point = Point{
        x : 26,
        y : 23,
    };
    println!(
        "x:{},y:{}" , point.x,point.y
    );

    println!("Alice's details: {:?}", point);
}
