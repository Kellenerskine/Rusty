// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition false");
//     }
// }

fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("the value of the number is: {number}");
}
