use iced::Element;
use iced::widget::{column, text, button};

#[derive(Debug, Clone, PartialEq)]
pub enum HomeMessage {
    // sent
    GotoDashboard,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Home {}

impl Home {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: &HomeMessage) -> () {
        match message {
            _ => {}
        }
    }

    pub fn view(&self) -> Element<'_, HomeMessage> {
        column![
            text("Home Page").size(30),

            button("Go to Dashboard")
                .on_press(HomeMessage::GotoDashboard),
        ]
        .spacing(10)
        .into()
    }
}
