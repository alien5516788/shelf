use iced::widget::{button, column, text};
use iced::{Element, Theme};

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .theme(Theme::Light)
        .centered()
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    Reset,
}

struct App {
    counter: i32,
}

impl App {
    fn new() -> Self {
        Self { counter: 0 }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.counter += 1,
            Message::Decrement => self.counter -= 1,
            Message::Reset => self.counter = 0,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            text(self.counter).size(40),

            button("+")
                .on_press(Message::Increment),

            button("-")
                .on_press(Message::Decrement),

            button("reset")
                .on_press(Message::Reset),
        ]
        .spacing(10)
        .into()
    }
}
