pub mod option_usage;
use option_usage::*;
pub mod result_usage;
use result_usage::*;

pub fn run() {
    println!("START\n");

    // Option Usage
    let option_int = give_option_int(42);
    let option_string = give_option_string("Apple".to_string());
    print_option_int(option_int);
    print_option_string(option_string);
    println!("");

    // Result Usage
    let result_int = give_result_int(51);
    let result_string = give_result_string("Banana".to_string());
    print_result_int(result_int);
    print_result_string(result_string);
    println!("");

    println!("\nEND");
}
