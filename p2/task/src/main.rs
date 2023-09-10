

struct FilterCondition {
    field: String,
    operator: String,
    value: String,
}

impl FilterCondition {
    fn new(field: String, operator: String, value: String) -> Self {
        Self {
            field,
            operator,
            value,
        }
    }

    fn is_match(&self, item: &str) -> bool {
        let item = item.to_lowercase();
        let value = self.value.to_lowercase();

        match self.operator.as_str() {
            "eq" => item == value,
            "ne" => item != value,
            "gt" => item > value,
            "lt" => item < value,
            "ge" => item >= value,
            "le" => item <= value,
            "like" => item.contains(&value),
            _ => false,
        }
    }

}



fn main() {
    
    let filter = FilterCondition::new("name".to_string(), "like".to_string(), "John".to_string());
    let data = vec!["John", "Jane", "Joe","Johnn"];

    let result = data
            .iter()
            .filter(|item| filter.is_match(item))
            .collect::<Vec<_>>();

    println!("{:?}", result);
}
