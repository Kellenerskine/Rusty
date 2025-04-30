// Write a function to convert a name into initials.
// This kata strictly takes two words with one space in between them.
// The output should be two capital letters with a dot separating them.

fn abbrev_name(name: &str) -> String {
    let parts: Vec<&str> = name.split_whitespace().collect();
    let first_initial = parts[0].chars().next().unwrap().to_ascii_uppercase();
    let second_initial = parts[1].chars().next().unwrap().to_ascii_uppercase();
    format!("{}.{}", first_initial, second_initial)
}



fn main() {
    println!("{}", abbrev_name("Martin Greene"));
}
