pub fn give_vector(count: i32) -> Vec<i32> {
    let mut vector = Vec::new();

    for i in 0..count {
        vector.push(i);
    }

    vector
}

pub fn print_vector(vector: Vec<i32>) {
    print!("Vector with size {}: ", vector.len());

    for num in vector {
        print!("{} ", num)
    }

    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_10() {
        // Arrange
        let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        // Act
        let result = give_vector(10);
        // Assert
        for i in 0..result.len() {
            assert_eq!(result[i], expected[i]);
        }
    }
}
