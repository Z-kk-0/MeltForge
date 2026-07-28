use mf_core::message::Message;

pub fn print_messages(messages: Vec<Message>) {
    for message in messages {
        match message {
            Message::Info(text) => println!("[info]    {}", text),
            Message::Success(text) => println!("[success] {}", text),
            Message::Warning(text) => println!("[warning] {}", text),
            Message::Error(text) => eprintln!("[error]   {}", text),
        }
    }
}
