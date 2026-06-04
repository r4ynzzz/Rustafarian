//Create new User struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    //Instance of new user
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("dude@coolbros.com"),
        sign_in_count: 1,
    };

    //Dot notation for access and changes of struct value
    user1.username = String::from("cooldude");

    build_user("somerandoem@em.com".to_string(), "steve".to_string());
}

//Function that returns 'User' Instance
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        //field init shorthand syntax for parameter names
        //that are the same.
        username,
        email,
        sign_in_count: 1,
    }
}
