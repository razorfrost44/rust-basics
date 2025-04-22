use rand::prelude::*;

pub fn get_random_i32() -> i32 {
    let mut rng = rand::rng();
    rng.random()
}

pub fn get_random_i32_from_range(min: i32, max: i32) -> i32 {
    let mut rng = rand::rng();
    rng.random_range(min..=max)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_1() {
        // Arrange
        // Act
        let random_number = get_random_i32();
        // Assert
        assert!(random_number >= i32::MIN && random_number <= i32::MAX);
    }

    #[test]
    fn get_1_from_range() {
        // Arrange
        // Act
        let random_number = get_random_i32_from_range(1, 10);
        // Assert
        assert!(random_number >= 1 && random_number <= 10);
    }
}
