pub mod option_usage;
use option_usage::*;

pub fn run() {
    println!("START\n");

    // Option Usage
    let option_int = give_option_int(42);
    let option_string = give_option_string("Apple".to_string());
    print_option_int(option_int);
    print_option_string(option_string);

    println!("\nEND");
}
