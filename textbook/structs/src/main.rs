struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("relzwit"),
        email: String::from("relz@gmail.com"),
        sign_in_count: 4,
    };
    user1.email = String::from("randemail@gmail.com");

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("rand@gmail.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // the above code has the same result as below:
    let user2 = User {
        email: String::from("rand@gmail.com"),
        ..user1
    };
}
