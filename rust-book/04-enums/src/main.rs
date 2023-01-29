fn main() {
    let msg_quit = Message::Quit;
    msg_quit.call();

    let msg_move = Message::Move { x: 3, y: 8 };
    msg_move.call();

    let msg_write = Message::Write(String::from("text"));
    msg_write.call();

    let msg_change_color = Message::ChangeColor(232, 22, 80);
    msg_change_color.call();
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move {} {}", x, y),
            Message::Write(s) => println!("Write {}", s),
            Message::ChangeColor(r, g, b) => println!("ChangeColor {} {} {}", r, g, b)
        }
    }
}
