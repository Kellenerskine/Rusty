// fn main() {
//     let mut counter = 0;
//     loop {
//         println!("printed {counter} times");
//         counter += 1;
//         if counter == 5 {
//             break;
//         }
//     }
// }

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//         count += 1;
//     }
//     println!("end count = {count}");
// }

// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}");
//         number -= 1;
//     }
//     println!("liftoff?");
// }

// fn main() {
//     let a = [1, 2, 3, 4, 5, 6];
//     for element in a {
//         println!("the elem is {element}");
//     }
// }

//rev seems hella useful DONT FORGET
fn main() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("finished");
}
