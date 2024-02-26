


fn longest<'a>(s1:&'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    }else {
        s2
    }
}

struct Data<'a> {
    value : &'a str,
}




fn main() {
    println!("Hello, world!");
}
