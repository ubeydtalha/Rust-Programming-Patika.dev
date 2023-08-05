

fn find_square_root(number : f64) -> Option<f64> {
    if number < 0.0 {
        None
    } else {
        Some(number.sqrt())
    }
}

fn divide(a:f64, b:f64) -> Result<f64,String> {
    if b == 0.0 {
        Err("Division by zero is not allowed".to_string())
    } else {
        Ok(a/b)
    }
}


fn get_from_database(key : &str) -> Option<f64> {
    let database = vec![
        ("pi", Some(3.14)),
        ("e", Some(2.71)),
        ("phi", Some(1.61)),
        
    ];
    
    for (k,v) in database {
        if k == key {
            return v;
        }
    }
    None
}


fn calculate_tringle_area(base : Option<f64>,height : Option<f64>) -> Result<f64,String> {

    match (base,height) {
        (Some(b),Some(h)) => {
            if b < 0.0 || h < 0.0 {
                Err("Base and height must be positive".to_string())
            } else {
                Ok(0.5 * b * h)
            }
        },
        (None , _) => Err("Base is missing".to_string()),
        (_, None) => Err("Height is missing".to_string())
    }
    
}


fn main() {
    
    let number = 4.0;
    let square_root = find_square_root(number);

    match square_root {
        Some(x) => println!("Square root of {} is {}", number, x),
        None => println!("Cannot find square root of {}", number)
    }

    let a = 10.0;
    let b = 0.0;
    let result = divide(a,b);

    match result {
        Ok(x) => println!("{} divided by {} is {}", a, b, x),
        Err(e) => println!("Error: {}", e)
    }


    let base = get_from_database("pi");
    let height = get_from_database("phi");
    let area = calculate_tringle_area(base,height);

    match area {
        Ok(x) => println!("Area of the triangle is {}", x),
        Err(e) => println!("Error: {}", e)
    }

}
