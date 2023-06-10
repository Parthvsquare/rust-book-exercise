enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn main() {
    let msg = Message::ChangeColor(0, 39, 200);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction by {} & y in direction {} ", x, y)
        }
        Message::Write(text) => println!("Text Message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to {} = R {} = G {} = B", r, g, b)
        }
    }
}
