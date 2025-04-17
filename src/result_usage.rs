pub fn give_result_int(num: i32) -> Result<i32, String> {
    if num > 10 {
        Ok(num)
    } else {
        Err(String::from("Number is too small"))
    }
}

pub fn give_result_string(str: String) -> Result<String, String> {
    if str.len() > 5 {
        Ok(str)
    } else {
        Err(String::from("String is too short"))
    }
}

pub fn print_result_int(result: Result<i32, String>) {
    match result {
        Ok(num) => println!("Result number: {}", num),
        Err(err) => println!("Error: {}", err),
    }
}

pub fn print_result_string(result: Result<String, String>) {
    match result {
        Ok(str) => println!("Result string: {}", str),
        Err(err) => println!("Error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_int() {
        // Arrange
        let expected = 67;
        // Act
        let result = give_result_int(expected);
        // Assert
        assert!(result.is_ok());
        assert!(result.unwrap() == expected);
    }

    #[test]
    fn result_string() {
        // Arrange
        let expected = "Hello, world!".to_string();
        // Act
        let result = give_result_string(expected.clone());
        // Assert
        assert!(result.is_ok());
        assert!(result.unwrap() == expected);
    }
}
