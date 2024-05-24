// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit(String),
    Echo(String),
    Move(String),
    ChangeColor(String),
}

fn main() {
    println!("{:?}", Message::Quit(String::from("Out")));
    println!("{:?}", Message::Echo(String::from("hello")));
    println!("{:?}", Message::Move(String::from("your turn")));
    println!("{:?}", Message::ChangeColor(String::from("blue")));
}
