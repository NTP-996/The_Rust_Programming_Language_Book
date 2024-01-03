//fn main() {
//    let mut s = String::from("hello word");
//
//    let word = first_word(&s);
//
//    s.clear();
//}
//
//fn first_word(s: &String) -> usize {
//    let bytes = s.as_bytes();
//
//    for (i, &item) in bytes.iter().enumerate() {
//        if item == b' ' {
//            return i;
//        }
//    }
//
//    s.len()
//}
//
//fn second_word(s: &String) -> (usize, usize) {}

// String Slices
fn main() {
    //    let s = String::from("hello world");

    //    let hello = &s[0..5];
    //    let world = &s[6..1];

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
}
