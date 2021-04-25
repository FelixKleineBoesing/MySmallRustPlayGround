fn main() {
    const ANSWER: u16 = 42;
    let mut product: u16 = 6;
    let result = ANSWER * product;
    println!("Result is: {}", result);
    product = 7;
    let result = ANSWER * product;
    println!("Result is: {}", result);
}