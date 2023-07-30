fn main() {
    
    let str1 = String::from("Hello");
    let str2 = String::from("World");
    
    let msg = concatenate_strings(str1, str2);

    println!("{}",msg);
}


fn concatenate_strings(
    str1 : String,
    str2 : String
)-> String{
    let mut res: String = String::new();
    res.push_str(&str1);
    res.push_str(&str2);
    return res;
}