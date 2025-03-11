use iced::Theme;
use iced::widget::{Column, button, column, text};

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

fn update(value: &mut i64, message: Message) {
    match message {
        Message::Increment => *value += 1,
        Message::Decrement => *value -= 1,
    }
}

fn view(value: &i64) -> Column<Message> {
    column![
        text(value),
        button("+").on_press(Message::Increment),
        button("-").on_press(Message::Decrement),
        text("hello_world_iced"),
    ]
}

pub fn main() -> iced::Result {
    iced::application("A counter", update, view)
        .theme(|_| Theme::Dark)
        .centered()
        .run()
}
