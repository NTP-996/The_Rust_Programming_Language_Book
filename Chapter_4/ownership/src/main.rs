fn main() {
    //let s = "hello";
    // s is not valid here, it’s not yet declared
    //  s is valid from this point forward

    //    let mut s = String::from("hello");
    //
    //    s.push_str(", world!");
    //
    //    println!("{}", s);

    // Variables and Data Interacting with Move
    //    let x = 5;
    //    let y = x;
    //
    //    let s1 = String::from("hello");
    //    let s2 = s1;
    //
    //    println!("{}, world!", s1);

    // Variables and Data Interacting with Clone
    //    let s1 = String::from("hello");
    //    let s2 = s1.clone();
    //
    //    println!("s1 = {}, s2 = {}", s1, s2);

    // Stack-Only Data: Copy
    //    let x = 5;
    //    let y = x;
    //
    //    println!("x = {}, y = {}", x, y);

    // Ownership and Functions

    //    let s = String::from("hello");
    //
    //    takes_ownership(s);
    //
    //    let x = 5;
    //
    //    makes_copy(x);

    // Return Values and Scope

    //    let s1 = gives_ownership();
    //
    //    let s2 = String::from("hello");
    //
    //    let s3 = takes_and_gives_back(s2);

    // Tuple
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is '{}'.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

//fn gives_ownership() -> string {
//    let some_string = string::from("yours");
//    some_string
//}
//
//fn takes_and_gives_back(a_string: string) -> string {
//    a_string
//}

//fn takes_ownership(some_string: String) {
//    println!("{}", some_string);
//}
//
//fn makes_copy(some_integer: i32) {
//    println!("{}", some_integer);
//}
