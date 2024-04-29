struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
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
    let mut user = User {
        username: String::from("someuser123"),
        email: String::from("someemail@example.com"),
        sign_in_count: 1,
        active: true,
    };
    let username = user.username;
    println!("User name is {}", username);

    let user2 = build_user(user.email, username);

    let user3 = User {
        email: String::from("example@example.com"),
        username: String::from("user3"),
        ..user
    };
}
