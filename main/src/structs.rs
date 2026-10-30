#[derive(Debug)]

struct user {
    username: String,
    age: u32,
    active: bool,
}

fn main () {
    let user1 = user {
        username: String::from("sulfur"),
        age: 19,
        active: true,
    };
    println!("{:?}", user1);
    println!("{}", user1.username);
    println!("{}", user1.age);
    println!("{}", user1.active);
}

// basically objects