fn main() {
    //let guess = "42".parse().expect("Not a number!");

    // Floating point types
    //let x = 2.0;
    //let y: f32 = 3.0;

    // Numeric Operations
    // addition
    //let num = 5 + 10;

    // subtraction
    //let difference = 95.5 - 4.3;

    // multiplication
    //let product = 4 * 30;

    // division
    //let quotient = 56.7 / 32.2;
    //let truncated = -5 / 3; // Results in -1

    // remainder
    //let remainder = 43 % 5;

    // The boolean Type
    //let t = true;

    // let f: bool = false; // with explicit type annotation

    // The Character Type
    //let c = 'z';
    //let z: char = 'ℤ';

    //let heart_eyed_cat = '😻';

    // The Compound Type
    // The Tuple type

    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
}
