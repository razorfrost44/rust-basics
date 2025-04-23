fn main() {
    rust_basics::run();
    // Stop terminal from closing
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
