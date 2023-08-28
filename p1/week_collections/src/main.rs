
use std::collections::HashMap;



fn main() {

    // Using the vec! macro
    let mut numbers = vec![1, 2, 3, 4, 5];

    // Using the new() method
    let mut names : Vec<String> = Vec::new();

    // Using the push() method
    names.push(String::from("John"));
    names.push(String::from("Doe"));

    let first_name = names.get(0);
    let second_name = &names[1];

    match first_name {
        Some(name) => println!("First name: {}", name),
        None => println!("No first name"),
    }

    println!("Second name: {:?}", second_name);

    // Iterating over a vector
    for number in &numbers {
        println!("Number: {}", number);
    }

    // Iterating over a vector and modifying it
    for number in &mut numbers {
        *number += 1;
    }

    // Iterating over a vector and modifying it
    for number in &numbers {
        println!("Number: {}", number);
    }

    // Using an enum to store multiple types
    enum Value {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let values = vec![
        Value::Int(3),
        Value::Float(10.0),
        Value::Text(String::from("Hello")),
    ];

    for value in &values {
        match value {
            Value::Int(number) => println!("Number: {}", number),
            Value::Float(number) => println!("Number: {}", number),
            Value::Text(text) => println!("Text: {}", text),
        }
    }

    //Slicing a vector
    let slice = &numbers[0..2];

    println!("Slice: {:?}", slice);

    // Using a HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    let blue_score = scores.get("Blue");

    match blue_score {
        Some(score) => println!("Blue score: {}", score),
        None => println!("No blue score"),
    }

    // Iterating over a HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating a HashMap
    scores.insert(String::from("Blue"), 30);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Only inserting a value if the key has no value
    scores.entry(String::from("Blue")).or_insert(50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating a value based on the old value
    let text = "hello world wonderful world";

    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", word_count);



}
