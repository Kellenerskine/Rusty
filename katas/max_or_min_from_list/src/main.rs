fn minimum(numbers: &[i32]) -> i32 {
    *numbers.iter().min().expect("List is empty")
}

fn maximum(numbers: &[i32]) -> i32 {
    *numbers.iter().max().expect("List is empty")
}
// just getting in a commit 
fn main() {
    let numbers = vec![42, 17, 8, 99, 23];

    println!("Minimum: {}", minimum(&numbers));
    println!("Maximum: {}", maximum(&numbers));
}

