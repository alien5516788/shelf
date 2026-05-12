use iced::Element;
use iced::widget::{column, text, button};

use super::types::Screen;
use crate::app::AppMessage;

#[derive(Debug, Clone, PartialEq)]
pub enum HomeMessage {
    None
}

#[derive(Debug, Clone, PartialEq)]
pub struct Home {}

impl Home {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: HomeMessage) -> () {
        match message {
            HomeMessage::None => {}
        }
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        column![
            text("Home Page").size(30),

            button("Go to Dashboard")
                .on_press(AppMessage::ChangeScreen(Screen::Dashboard)),
        ]
        .spacing(10)
        .into()
    }
}
