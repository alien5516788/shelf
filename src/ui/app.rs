use iced::{Element, Font, Theme};

use super::types::Component;
use super::types::Screen;
use super::home::{Home, HomeMessage};
use super::dashboard::{Dashboard, DashboardMessage};
use super::dashboard::components::NavbarMessage;


#[derive(Debug, Clone, PartialEq)]
pub enum AppMessage {
    HomeMessage(HomeMessage),
    DashboardMessage(DashboardMessage),
}

#[derive(Debug, Clone, PartialEq)]
pub struct App {
    theme: Theme,
    font: Font,
    screen: Screen,
    home: Home,
    dashboard: Dashboard,
}

impl Component for App {
    type Message = AppMessage;

    fn new() -> Self {
        Self {
            theme: Theme::Dracula,
            font: Font::MONOSPACE,
            screen: Screen::Home,
            home: Home::new(),
            dashboard: Dashboard::new(),
        }
    }

    fn update(&mut self, message: AppMessage) -> () {

        match message {
            AppMessage::HomeMessage(home_m) => match home_m {
                HomeMessage::GotoDashboard => self.screen = Screen::Dashboard,
            },
            AppMessage::DashboardMessage(dashboard_m) => match &dashboard_m {
                DashboardMessage::NavbarMessage(navbar_m) => match navbar_m {
                    NavbarMessage::GotoHome => self.screen = Screen::Home,
                    NavbarMessage::GotoSettings => /* self.screen = Screen::Settings */ {},
                    NavbarMessage::ToggleDarkMode => self.toggle_theme(),
                    _ => self.dashboard.update(dashboard_m),
                }
            },
        }
    }

    fn view(&self) -> Element<'_, AppMessage> {
        match self.screen {
            Screen::Home => self.home.view().map(|m| AppMessage::HomeMessage(m)),
            Screen::Dashboard => self.dashboard.view().map(|m| AppMessage::DashboardMessage(m)),
        }
    }
}

impl App {
    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    pub fn toggle_theme(&mut self) {
        match self.theme {
            Theme::Light => self.theme = Theme::Dracula,
            Theme::Dracula => self.theme = Theme::Light,
            _ => self.theme = Theme::Light,
        }
    }
}
