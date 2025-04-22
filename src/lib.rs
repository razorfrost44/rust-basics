pub mod option_usage;
use option_usage::*;
pub mod result_usage;
use result_usage::*;
pub mod vector_usage;
use vector_usage::*;
pub mod random_numbers_usage;
use random_numbers_usage::*;

pub fn run() {
    println!("START\n");

    // Option Usage
    let option_int = give_option_int(42);
    let option_string = give_option_string("Apple".to_string());
    print_option_int(option_int);
    print_option_string(option_string);
    println!();

    // Result Usage
    let result_int = give_result_int(51);
    let result_string = give_result_string("Banana".to_string());
    print_result_int(result_int);
    print_result_string(result_string);
    println!();

    // Vector Usage
    let vector = give_vector(10);
    print_vector(vector);
    println!();

    // Random Numbers Usage
    let random_number = get_random_i32();
    let random_number_range = get_random_i32_from_range(1, 100);
    let random_three = get_random_three_i32();
    let random_three_range = get_random_three_i32_from_range(1, 100);
    let random_with_step = generate_random_with_step(0, 20, 5);
    println!("Random number: {}", random_number);
    println!("Random number from range 1-100: {}", random_number_range);
    println!(
        "Random three numbers: ({}, {}, {})",
        random_three.0, random_three.1, random_three.2
    );
    println!(
        "Random three numbers from range 1-100: ({}, {}, {})",
        random_three_range.0, random_three_range.1, random_three_range.2
    );
    println!("Random number with step 4: {}", random_with_step);
    println!();

    println!("\nEND");
}
