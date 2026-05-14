use iced::Element;
use iced::widget::{column, text, button};

use crate::ui::types::Component;

#[derive(Debug, Clone, PartialEq)]
pub enum HomeMessage {
    GotoDashboard,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Home {}

impl Component for Home {
    type Message = HomeMessage;

    fn new() -> Self {
        Self {}
    }

    fn update(&mut self, message: HomeMessage) -> () {
        match message {
            HomeMessage::GotoDashboard => {},
        }
    }

    fn view(&self) -> Element<'_, HomeMessage> {
        column![
            text("Home Page").size(30),

            button("Go to Dashboard")
                .on_press(HomeMessage::GotoDashboard),
        ]
        .spacing(10)
        .into()
    }
}
