#[derive(Debug)]
enum List {
	Cons(i32, Box<List>),
	Nil,
}
 
use List::{Cons, Nil};
 
trait PartyTrick {
    fn perform(&self);
}

struct Magician {
    name: String,
}

impl PartyTrick for Magician {
    fn perform(&self) {
        println!("{} pulls a rabbit from his hat!", self.name);
    }
}

fn main() {

    let b = Box::new(5);
    println!("b = {}", *b);


    {
        let c = Box::new(6);
        println!("c = {}", *c);
    }

    // println!("c = {}", *c); // error: use of undeclared variable `c`
    let list = Cons(1,
    	Box::new(Cons(2,
        	Box::new(Cons(3,
            	Box::new(Nil))))));

    println!("list = {:?}", list);

    let my_trait_object : Box <dyn PartyTrick> = Box::new(Magician { name: "Gandalf".to_string() });

    my_trait_object.perform();
}
