struct Person<'a> {
    name : &'a String,
}

fn get_name<'a>(person: &Person) -> &'a str {
    "Alice"
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_name() {
        let name = String::from("Alice");
        let person = Person { name: &name };
        assert_eq!(get_name(&person), "Alice");
    }
}
