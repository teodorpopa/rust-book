struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {

    let mut user = User {
        active: true,
        username: String::from("tcoder"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user.email = String::from("new_email@example.com");

    let user2 = User {
        username: String::from("username"),
            ..user
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}
