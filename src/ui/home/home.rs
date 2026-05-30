use iced::Element;
use iced::widget::{column, text, button};

use crate::ui::app::AppMessage;

// Home messages
#[derive(Debug, Clone, PartialEq)]
pub enum HomeMessage {
    GotoDashboard,
}

// Home state
#[derive(Debug, Clone, PartialEq)]
pub struct Home {}

impl Home {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        column![
            text("Home Page").size(30),

            button("Go to Dashboard")
                .on_press(AppMessage::HomeMessage(HomeMessage::GotoDashboard)),
        ]
        .spacing(10)
        .into()
    }
}
