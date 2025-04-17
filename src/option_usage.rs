pub fn give_option_int(num: i32) -> Option<i32> {
    Some(num)
}

pub fn give_option_string(str: String) -> Option<String> {
    Some(str)
}

pub fn print_option_int(option: Option<i32>) {
    match option {
        Some(num) => println!("Option number: {}", num),
        None => println!("No number provided"),
    }
}

pub fn print_option_string(option: Option<String>) {
    match option {
        Some(str) => println!("Option string: {}", str),
        None => println!("No string provided"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_int_unwrap() {
        // Arrange
        let expected = 31;
        // Act
        let option1 = give_option_int(expected);
        let result = option1.unwrap_or(0);
        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn test_option_int_match() {
        // Arrange
        let expected = 98;
        // Act
        let option1 = give_option_int(expected);
        let result = match option1 {
            Some(num) => num,
            None => 0,
        };
        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn test_option_string_unwrap() {
        // Arrange
        let expected = String::from("Apple");
        // Act
        let option1 = give_option_string(expected.clone());
        let result = option1.unwrap_or(String::from("No string"));
        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn test_option_string_match() {
        // Arrange
        let expected = String::from("Banana");
        // Act
        let option1 = give_option_string(expected.clone());
        let result = match option1 {
            Some(str) => str,
            None => String::from("No string"),
        };
        // Assert
        assert_eq!(result, expected);
    }
}
