fn main() {
    println!("Hello, world!");
    
    let mut user1 = User {
        active: true,
        username: String::from("Toni"),
        email: String::from("toni@bonboni"),
        sign_in_count: 1,
    };

    // tuple structs
    struct Color(i32, i32, i32);

    // unit-like structs
    struct AlwaysEqual;

    println!("test: {0}", user1.username);

    let mut user2 = build_user(String::from("test@test.com"), String::from("test"));

    user2.username = String::from("UserChanged");

    println!("test: {0}", user2.username);

    let mut user3 = build_user_init(String::from("email"), String::from("username"));

    println!("test: {0}", user3.username);

    let user4 = User {
        email: String::from("test@test1"),
        ..user2
    };

    println!("TEST {0}", user4.username);

}

fn build_user_init(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
