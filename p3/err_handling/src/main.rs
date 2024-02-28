use std::collections::HashMap;


    fn get_ingredient(
        name: &str,
    ) -> Result<Option<String>, &'static str> {
        let mut pantry: HashMap<String, Option<String>> = HashMap::new();


        pantry.insert(
            "Eggs".to_string(),
            Some("10".to_string()),
        );


        match pantry.get(name) {
            Some(ingredient) => {
                match ingredient {
                    Some(details) => Ok(Some(details.to_string())),
                        None => Ok(None),
                }
            },
            None => Err("No such ingredient"),
        }
    }

    fn main() {
        let eggs = get_ingredient("Eggs");

        match eggs {
            Ok(Some(details)) => println!("Eggs: {}", details),
            Ok(None) => println!("No eggs"),
            Err(_) => println!("No such ingredient"),
        }
    }
