use std::collections::HashMap;


fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let doubled : Vec<i32> = 
        numbers
            .iter()
            .map(|x| x*2)
            .collect();

    println!("{:?}", doubled);

    let numbers = vec![1, 2, 3, 4, 5];

    let even_numbers : Vec<i32> = 
        numbers
            .into_iter()
            .filter(|x| x % 2 == 0)
            .collect();

    println!("{:?}", even_numbers);

    let numbers = vec![1, 2, 3, 4, 5];

    let sum : i32 = 
        numbers
            .iter()
            .fold(0, |acc, x| acc + x);
    
    println!("{}", sum);

    // Chained iterators

    let words = vec!["hello", "world", "rust", "is", "cool"];

    let result : Vec<String> = 
        words
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .map(|(i, word)| format!("{}: {}", i + 1, word.to_uppercase()))
            .collect();

    println!("{:?}", result);

    // Hashmap

    let numbers = vec![1, 2, 3, 4, 5];
    let number_squares : HashMap<_,_> = 
        numbers
            .iter()
            .map(|x| (x, x*x))
            .collect();

    println!("{:?}", number_squares);

}
