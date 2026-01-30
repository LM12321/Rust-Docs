


fn example_one(){
    enum IpAddrKind{
        V4,
        V6
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

}

fn example_two(){
    enum IpAddr {
        V4(String),
        V6(String),
    };

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}

fn example_three(){
    enum IpAddr {
        V4(u8,u8,u8,u8),
        V6(String),
    }
}

fn example_four(){
    enum Message {
        Quit,
        Move { x: i32, y: i32},
        Write (String),
        ChangeColor(i32, i32, i32),
    }
    impl Message { 
        fn call(&self){
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn example_five(){
    //Option Enum

    enum Option_example_not_shown<T>{   //Option<T> exists in Rust base
        None, 
        Some(T),
    }

    //encodes the concept of a value being present or absent
    //T is the generic type parameter

    let some_number = Some(5);
    let some_char = Some('e');

    let mut absent_number: Option<i32> = Some(5);
    //absent_number = Some(32);

    //you can't add an i32 to an Option<i32>
    // Check out Option<T> documentation
}

fn main() {
    //example_four();
    example_five();
}
