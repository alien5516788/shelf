use std::sync::Arc;

use iced::{Element, Task};
use iced::widget::{column, text, button};
use sqlx::SqlitePool;

use crate::app::Screen;


#[derive(Debug)]
pub struct Settings {
    pool: Option<Arc<SqlitePool>>
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    LoadSettings(Arc<SqlitePool>),
    SetScreen(Screen),
}

impl Settings {
    pub fn new(pool: Arc<SqlitePool>) -> (Self, Task<SettingsMessage>) {
        (
            Self {
                pool: None,
            },
            Task::done(SettingsMessage::LoadSettings(pool)),
        )
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

    pub fn update(&mut self, message: SettingsMessage, screen: &mut Screen) -> Task<SettingsMessage> {
        match message {
            SettingsMessage::LoadSettings(pool) => {
                self.pool = Some(pool);
                Task::none()
            },
            SettingsMessage::SetScreen(scrn) => {
                *screen = scrn;
                Task::none()
            },
        }
    }
}
