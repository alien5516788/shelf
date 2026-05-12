use iced::Element;
use iced::widget::{column, text, button};

use super::types::Screen;
use crate::app::AppMessage;

#[derive(Debug, Clone, PartialEq)]
pub enum DashboardMessage {
    None
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dashboard {}

impl Dashboard {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: DashboardMessage) -> () {
        match message {
            DashboardMessage::None => {}
        }
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        column![
            text("Dashboard Page").size(30),

            button("Go to Home")
                .on_press(AppMessage::ChangeScreen(Screen::Home)),
        ]
        .spacing(10)
        .into()
    }
}
