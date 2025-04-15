fn main() {
    let s = String::from("Hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
    // ...therefore no longer valid here
    let x = 5;

    makes_copy(x);
} // here, x goes out of scope, then s. however, because s's...
// ... was moved, nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} //some_string goes out of scope, freed

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // here some_integer goes out of scope, but drop is not called?
