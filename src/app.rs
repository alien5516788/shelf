use iced::Element;

use crate::screens::types::Screen;
use crate::screens::home::{Home, HomeMessage};
use crate::screens::dashboard::{Dashboard, DashboardMessage};

#[derive(Debug, Clone, PartialEq)]
pub enum AppMessage {
    ChangeScreen(Screen),
    HomeMessage(HomeMessage),
    DashboardMessage(DashboardMessage),
}

// App itself is the screenmanager
pub struct App {
    screen: Screen,
    home: Home,
    dashboard: Dashboard
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Home,
            home: Home::new(),
            dashboard: Dashboard::new()
        }
    }

    pub fn update(&mut self, message: AppMessage) -> () {
        match message {
            AppMessage::ChangeScreen(screen) => self.screen = screen,
            AppMessage::HomeMessage(message) => self.home.update(message),
            AppMessage::DashboardMessage(message) => self.dashboard.update(message)
        }
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        match self.screen {
            Screen::Home => self.home.view(),
            Screen::Dashboard => self.dashboard.view()
        }
    }
}
