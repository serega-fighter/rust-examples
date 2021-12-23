
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn is_celeb(&self) -> bool {
        self.username.len() > 5
    }
}

fn main() {

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("User: {:?}", user2);
    println!("User is celeb: {:?}", user2.is_celeb())
}