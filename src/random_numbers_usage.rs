use rand::prelude::*;

pub fn get_random_i32() -> i32 {
    let mut rng = rand::rng();
    rng.random()
}

pub fn get_random_i32_from_range(min: i32, max: i32) -> i32 {
    let mut rng = rand::rng();
    rng.random_range(min..=max)
}

pub fn get_random_three_i32() -> (i32, i32, i32) {
    let mut rng = rand::rng();
    (rng.random(), rng.random(), rng.random())
}

pub fn get_random_three_i32_from_range(min: i32, max: i32) -> (i32, i32, i32) {
    let mut rng = rand::rng();
    (
        rng.random_range(min..=max),
        rng.random_range(min..=max),
        rng.random_range(min..=max),
    )
}

pub fn generate_random_with_step(min: i32, max: i32, step: i32) -> i32 {
    let mut rng = rand::rng();
    let total_steps = (max - min) / step + 1;
    let random_index = rng.random_range(0..total_steps);
    min + random_index * step
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

    #[test]
    fn get_3() {
        // Arrange
        // Act
        let (a, b, c) = get_random_three_i32();
        // Assert
        assert!(a >= i32::MIN && a <= i32::MAX);
        assert!(b >= i32::MIN && b <= i32::MAX);
        assert!(c >= i32::MIN && c <= i32::MAX);
    }

    #[test]
    fn get_3_from_range() {
        // Arrange
        let min = 1;
        let max = 10;
        // Act
        let (a, b, c) = get_random_three_i32_from_range(min, max);
        // Assert
        assert!(a >= min && a <= max);
        assert!(b >= min && b <= max);
        assert!(c >= min && c <= max);
    }

    #[test]
    fn get_random_with_steps() {
        // Arrange
        let min = 0;
        let max = 10;
        let step = 2;
        let possible = vec![0, 2, 4, 6, 8, 10];
        // Act
        let result = generate_random_with_step(min, max, step);
        // Assert
        assert!(result >= min && result <= max);
        assert!(possible.contains(&result));
    }
}
