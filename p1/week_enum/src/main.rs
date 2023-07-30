use std::fmt::Write;

#[derive(Debug)]
enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    Snowy,
}

enum Message {
    Quit , 
    Move {
            x: i32,
            y: i32,
        },
    Write(String),
    ChangeColor(i32,i32,i32),
}




fn main() {
    let current_weather = Weather::Sunny;
    println!("{:#?}", current_weather);
    
    let msg = Message::Write(
        String::from(
            "Hello Rust!"
        )
    );

    process_message(msg);

    let received_text = Message::Move { x: 12, y: 12 };
    // Message::Write(
    //     String::from("Hello".to_string())
    // );

    if let Message::Write(text) = received_text.copy() {
        println!("hello");
    }

    else {
        println!("Something went wrong");
    }

    received_text.call();

}


fn process_message(
    msg : Message
) {

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data.")
        }
        Message::Move { x, y } => {
            println!("Move te coordinates x: {} , y:{}",x,y)
        }
        Message::Write(text) => {
            println!("Text message {}",text);
        }
        Message::ChangeColor(r,g ,b ) => {
            println!("Color r({}) g({}) b({})",r,g,b);
        }
    }
}


impl Message {
    
    // return new copy of message
    fn copy(&self) -> Message {
        match self {
            Message::Quit => {
                println!("The Quit variant has no data.");
                Message::Quit
            }
            Message::Move { x, y } => {
                println!("Move te coordinates x: {} , y:{}",x,y);
                Message::Move { x: *x, y: *y }
            }
            Message::Write(text) => {
                println!("Text message {}",text);
                Message::Write(text.clone())
            }
            Message::ChangeColor(r,g ,b ) => {
                println!("Color r({}) g({}) b({})",r,g,b);
                Message::ChangeColor(*r,*g,*b)
            }
        }
    }



    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(s) => println!("Write: {}", s),
            Message::ChangeColor(r, g, b) => println!("Change color to R: {}, G: {}, B: {}", r, g, b),
        }
    }

    


}
