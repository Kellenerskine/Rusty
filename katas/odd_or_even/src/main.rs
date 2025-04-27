fn even_or_odd(number: i32) -> &'static str {
    if number % 2 == 0{
        "Even"
    }else{
        "Odd"
    }
}


fn main() {
    println!("{}", even_or_odd(25555550));
}
