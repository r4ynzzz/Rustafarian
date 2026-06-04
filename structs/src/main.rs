//Create new User struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//Create tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//Define unit-like Struct (empty field)
//behave similarly to '()'
struct AlwaysEqual;

fn main() {
    //instantiating unit struct
    let subjects = AlwaysEqual;

    //Instance of new user
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("dude@coolbros.com"),
        sign_in_count: 1,
    };

    //Dot notation for access and changes of struct value
    user1.username = String::from("cooldude");

    //My own trying here. Look for better option if it exists.
    build_user("somerandoem@em.com".to_string(), "steve".to_string());

    //Create a new instance of a struct that includes most of the values
    //from another instance of the same type, but changes some of them.
    //Struct update syntax: '..'
    //..user1 must be last.
    //Since we took the String in username from user1 and moved it to user2,
    //user1 is out of scope. If we change the username and email, then user1,
    //does not go out of scope. because active and sign_in_count user the Copy
    //trait

    let user2 = User {
        email: String::from("coolestbro@coolbros.com"),
        ..user1
    };

    //tuple struct usage
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    //destructure tuple struct.
    //you have to name the type of struct when you destructure to access those values
    let Point(x, y, z) = origin;
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
