
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

struct AlwaysEqual; //unit-like struct
//similar to (), the unit type

fn main() {

    let user1: User = build_user(String::from("example@example.org"), String::from("Basdf"));
    let user2: User = User{
        email: String::from("another@example.com"),
        ..user1
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    let subject = AlwaysEqual;
}
