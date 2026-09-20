use iced::{Element, Task};
use iced::widget::{button, center, column, text};

use crate::data::settings::AppScreen;
use crate::utils::font_size::{sv, sv_20};


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
        center(
            column![
                text("Shelf").size(sv(30.0)),
    
                button(text("Explore the shelf").size(sv_20()))
                    .on_press(HomeMessage::SetScreen(AppScreen::Dashboard)),
            ]
            .spacing(10)
        )
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
