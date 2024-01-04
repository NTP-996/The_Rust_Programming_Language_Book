//struct User {
//    active: bool,
//    username: String,
//    email: String,
//    sign_in_count: u64,
//}
//
//fn main() {
//    let mut user1 = User {
//        active: true,
//        username: String::from("someusername123"),
//        email: String::from("someone@example.com"),
//        sign_in_count: 1,
//    };
//
//    user1.email = String::from("anotheremail@example.com");
//}
//
//fn build_user(email: String, username: String) -> User {
//    User {
//        active: true,
//        username: username,
//        email: email,
//        sign_in_count: 1,
//    }
//}

//fn main() {
//    let width1 = 30;
//    let height1 = 50;
//
//    println!(
//        "The area of the rectangle is {} square pixels.",
//        area(width1, height1)
//    );
//}
//
//fn area(width: u32, height: u32) -> u32 {
//    width * height
//}

// Refactoring with Tuples

//fn main() {
//    let rect1 = (30, 50);
//
//    println!(
//        "The area of the rectangle is {} square pixels.",
//        area(rect1)
//    );
//}
//
//fn area(dimensions: (u32, u32)) -> u32 {
//    dimensions.0 * dimensions.1
//}
//
// Refactoring with Structs: Adding More Meaning

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
