fn main() {
    let user1 = User {
        email: String::from("test@email.com"),
        username: String::from("username1"),
        active: true,
        sign_in_count: 1,
    };
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}