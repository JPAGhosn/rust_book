pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    pub fn call(&self) {
        match self {
            Message::Quit => println!("Quiting the application"),
            Message::Move { x, y } => println!("Moving to {x} {y}"),
            Message::Write(text) => println!("Writing to {text}"),
            Message::ChangeColor(r, g, b) => println!("Changing color to {r} {g} {b}"),
            _ => println!("Message Type not defined")
        }
    }
}
