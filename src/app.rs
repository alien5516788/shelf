use iced::{Element, Theme};

use crate::screens::types::Screen;
use crate::screens::home::{Home, HomeMessage};
use crate::screens::dashboard::{Dashboard, DashboardMessage};
use crate::components::navbar::NavbarMessage;

// Message order must be sent or recieved for all message enums (including sub components)
#[derive(Debug, Clone, PartialEq)]
pub enum AppMessage {
    // recieved
    HomeMessage(HomeMessage),
    DashboardMessage(DashboardMessage),
}

// App itself is the screen manager
pub struct App {
    theme: Theme,
    screen: Screen,
    home: Home,
    dashboard: Dashboard
}

impl App {
    // Initial states
    pub fn new() -> Self {
        Self {
            theme: Theme::Light,
            screen: Screen::Home,
            home: Home::new(),
            dashboard: Dashboard::new()
        }
    }

    // Messages intended for current component must be handled
    // Messages intended for sub components must be passed through update handler of sub component
    pub fn update(&mut self, message: AppMessage) -> () {
        let m = &message;

        match m {
            AppMessage::HomeMessage(home_m) => match home_m {
                HomeMessage::GotoDashboard => self.screen = Screen::Dashboard,
            },
            AppMessage::DashboardMessage(dashboard_m) => match dashboard_m {
                DashboardMessage::NavbarMessage(navbar_m) => match navbar_m {
                    NavbarMessage::GotoHome => self.screen = Screen::Home,
                    NavbarMessage::GotoSettings => {/* TODO */},
                    NavbarMessage::ToggleDarkMode => { self.toggle_theme() },
                    _ => self.dashboard.update(dashboard_m)
                }
            }
        }
    }

    // Messages recieved from sub components are mapped as if they were sent through the main component
    // This allows subcomponents to avoid deeper parent paths to access AppMessage enum
    pub fn view(&self) -> Element<'_, AppMessage> {
        match self.screen {
            Screen::Home => self.home.view().map(|m| AppMessage::HomeMessage(m)),
            Screen::Dashboard => self.dashboard.view().map(|m| AppMessage::DashboardMessage(m))
        }
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    pub fn toggle_theme(&mut self) {
        match self.theme {
            Theme::Light => self.theme = Theme::Dark,
            Theme::Dark => self.theme = Theme::Light,
            _ => self.theme = Theme::Light
        }
    }
}
