// fn main() {
//     let s = "hello";
//     let s_slice = &s[0..2];
//     let rand = print_my_str(s_slice);
// }

// fn print_my_str(s: &str) {
//     println!("this is s: {s}");
// }

fn main() {
    let mut s = String::from("Hello world");

    let word = first_word(&s);
    println!("word: {word}");
    s.clear();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
