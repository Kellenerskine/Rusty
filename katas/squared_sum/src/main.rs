// Complete the square sum function so that it squares each number passed into it and 
// then sums the results together.
// For example, for [1, 2, 2] it should return 9 because 1^2+2^2+2^2=9.


fn square_sum(vec: Vec<i32>) -> i32 {
    let mut results = 0;
    for num in vec{
        results += num * num;
    }
        println!("{}", results);

    results
}

fn main() {
    println!("{}", square_sum(vec![1, 2]));
}
