struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn structfn() {
    //this is only accessible but not writable
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    //accessible and writeable
    let mut user2 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user2.email = String::from("anotheremail@example.com");

    let _user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // let _user4 = User {
    //     email: String::from("anotherexample.com"),
    //     ..user1
    // };
}
