fn main() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    //Enum with variety of types.
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    //Just like structs, we can define methods using 'impl.'
    impl Message {
        fn call(&self) {
            //method body
        }
    }

    let m = Message::Write(String::from("Hola"));
    m.call();

    //Option: represents a value that is something or nothing (Rust has no Null.)
    enum Option<T> {
        None,
        Some(T),
    }

    //Example of using Option values.
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    //This wont work. you have to convert an Option<T> to a T before you can perform T operations with it.
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}
