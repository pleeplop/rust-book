
#[derive(Debug)]
struct Color (i32, i32, i32);

#[derive(Debug)]
struct Point (i32, i32, i32);

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let mut user1 = User {
        email: String::from("someon@example.com"),
        username: String::from("someoneusername123"),
        sign_in_count: 1,
        active: true,
    };

    user1.username = String::from("anotherusername");

    let user2 = build_user(String::from("user2"), String::from("user2@example.com"));
    update_email(&mut user1, String::from("anotheremail@example.com"));

    let user3 = User {
        username: String::from("user3"),
        ..user1
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    println!("Hello, world {:?} {:?}!", black, origin);
}

fn build_user(username: String, email: String) -> User {
    User {
        username: username,
        email: email,
        sign_in_count: 1,
        active: true,
    }
}

fn update_email(user: &mut User, email: String) {
    user.email = email;
}
