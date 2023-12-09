fn main() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 12 };

    match msg {
        Message::Hello { id: 1..=9 } => {
            println!("Found 1 to 9")
        }

        Message::Hello { id: ida @ 0..=100 } => {
            println!("Found to 10 to 100: {}", ida)
        }

        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
