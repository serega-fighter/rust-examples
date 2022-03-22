
#[derive(Debug)]
struct User<'a> {
    active: bool,
    username: String,
    email: String,
    strRef: &'a str,
    sign_in_count: u64,
}

impl<'a> User<'a> {
    fn is_celeb(& self) -> bool {
        self.username.len() > 5
    }
}

fn main() {

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        strRef: "ABC",
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("User: {:?}", user2);
    println!("User is celeb: {:?}", user2.is_celeb())
}