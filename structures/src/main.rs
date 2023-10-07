struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user() -> User {
    User {
        email: String::from("prajwal.praveen@gmail.com"),
        active: true,
        username: String::from("prajwal"),
        sign_in_count: 10,
    }
}

fn main() {
    // A struct, or structure,
    // is a custom data type that lets you package together and
    // name multiple related values that make up a meaningful group.
    let user = build_user();
    let user1 = User {
        email: String::from("new user"),
        ..user
    };
    println!("{}", user.active);
}
