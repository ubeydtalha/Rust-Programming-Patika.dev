
struct Container<T>{
    value: T,
}

enum MagicalResult<T,E>{
    Success(T),
    Failure(E),
}

// Generic struct
struct FancyPair<T,U> {
    first: T,
    second: U,
}



fn main() {

    let int_container = Container{value: 5};
    let string_container = Container{value: "Hello, world!"};
    let float_container = Container{value: 3.14};

    println!("int_container.value = {}", int_container.value);
    println!("string_container.value = {}", string_container.value);
    println!("float_container.value = {}", float_container.value);


    let success_result : MagicalResult<i32,String> = MagicalResult::Success(42);
    let failure_result : MagicalResult<i32,String> = MagicalResult::Failure("Ooops!".to_string());

    match success_result {
        MagicalResult::Success(value) => println!("We have got the answer {}!", value),
        MagicalResult::Failure(message) => println!("Something went wrong: {}", message),
    }

    match failure_result {
        MagicalResult::Success(value) => println!("We have got the answer {}!", value),
        MagicalResult::Failure(message) => println!("Something went wrong: {}", message),
    }

    let int_str_pair = FancyPair{
        first: 42,
        second: "Hello, world!".to_string(),
    };
    

    println!("int_str_pair.first = {}", int_str_pair.first);
    println!("int_str_pair.second = {}", int_str_pair.second);





}
