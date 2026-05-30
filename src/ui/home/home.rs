use iced::Element;
use iced::widget::{column, text, button};

use super::HomeMessage;

#[derive(Debug, PartialEq)]
pub struct Home {}

impl Home {
    pub fn new() -> Self {
        Self {}
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
