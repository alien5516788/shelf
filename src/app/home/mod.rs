use iced::{Element, Task};
use iced::widget::{column, text, button};

use crate::data::settings::AppScreen;


#[derive(Debug, PartialEq)]
pub struct Home {}

#[derive(Debug, Clone, PartialEq)]
pub enum HomeMessage {
    SetScreen(AppScreen),
}

impl Home {
    pub fn new() -> (Self, Task<HomeMessage>) {
        (
            Self {},
            Task::none(),
        )
    }

    pub fn view(&self) -> Element<'_, HomeMessage> {
        column![
            text("Home Page").size(30),

            button("Go to Dashboard")
                .on_press(HomeMessage::SetScreen(AppScreen::Dashboard)),
        ]
        .spacing(10)
        .into()
    }

    pub fn update(&mut self, message: HomeMessage, screen: &mut AppScreen) -> Task<HomeMessage> {
        match message {
            HomeMessage::SetScreen(scrn) => {
                *screen = scrn;
                Task::none()
            },
        }
    }
}
