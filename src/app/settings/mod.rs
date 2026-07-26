use iced::Element;
use iced::widget::{column, text, button};

use crate::app::Screen;


#[derive(Debug, PartialEq)]
pub struct Settings {}

#[derive(Debug, Clone, PartialEq)]
pub enum SettingsMessage {
    SetScreen(Screen),
}

impl Settings {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<'_, SettingsMessage> {
        column![
            text("Settings Page").size(30),

            button("Go to Dashboard")
                .on_press(SettingsMessage::SetScreen(Screen::Dashboard)),
        ]
        .spacing(10)
        .into()
    }

    pub fn update(&mut self, message: SettingsMessage, screen: &mut Screen) {
        match message {
            SettingsMessage::SetScreen(scrn) => *screen = scrn,
        }
    }
}
