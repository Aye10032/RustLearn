pub fn run() {
    // Float
    let x = 2.0;
    let y: f32 = 3.0;

    println!("x = {x}; y = {y}");

    // addition
    let sum = 5 + 10;
    println!("sum = {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference = {difference}");

    // multiplication
    let product = 4 * 30;
    println!("product = {product}");

    // division
    let quotient = 56.7 / 32.6;
    println!("quotient = {quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("truncated = {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder = {remainder}");

    // Boolean
    let t = true;

    let f: bool = false; // with explicit type annotation

    // Tuple Type

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let y = tup.1;
    println!("The value of y is: {y}");

    // Array Type
    let a: [i32; 5] = [1, 2, 3, 4, 5];
}