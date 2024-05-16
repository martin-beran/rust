#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct UnitStruct;

fn main() {
    let mut user1 = build_user(
        String::from("someusername123"),
        "someone@example.com".to_string(),
    );
    print_user(&user1);
    user1.email = String::from("anotheremail@example.com");
    print_user(&user1);
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    print_user(&user2);
    let user2 = dbg!(user2);
    dbg!(&user2);
    println!("{:?}", user2);
    println!("{:#?}", user2);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    //let p: Point = black;
    let u1 = UnitStruct;
    let u2 = UnitStruct{};
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!(
        "username={1} active={0} email={2} cnt={3}",
        user.active, user.username, user.email, user.sign_in_count
    );
}
