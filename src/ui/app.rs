use iced::{Element, Font, Theme};

use super::AppMessage;
use super::home::{Home, HomeMessage};
use super::dashboard::{Dashboard, DashboardMessage};

/*
 * App is the global entity that holds the state of the entire application
 */

#[derive(Debug, PartialEq)]
pub struct App {
    theme: Theme,
    font: Font,
    screen: Screen,

    home: Home,
    dashboard: Dashboard,
}

#[derive(Debug, PartialEq)]
pub enum Screen {
    Home,
    Dashboard,
    Settings,
}

impl App {
    pub fn new() -> Self {
        Self {
            theme: Theme::Dracula,
            font: Font::MONOSPACE,
            screen: Screen::Dashboard,

            home: Home::new(),
            dashboard: Dashboard::new(),
        }
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        match self.screen {
            Screen::Home => self.home.view().map(|m| AppMessage::HomeMessage(m)),
            Screen::Dashboard => self.dashboard.view().map(|m| AppMessage::DashboardMessage(m)),
            Screen::Settings => self.dashboard.view().map(|m| AppMessage::DashboardMessage(m)), // TODO: implement settings
        }
    }

    pub fn update(&mut self, message: AppMessage) -> () {
        match message {
            AppMessage::HomeMessage(home_m) => match home_m {
                HomeMessage::GotoDashboard => self.screen = Screen::Dashboard,
            },
            AppMessage::DashboardMessage(dashboard_m) => match dashboard_m {
                DashboardMessage::GotoHome => self.screen = Screen::Home,
                DashboardMessage::GotoSettings => self.screen = Screen::Settings,
                DashboardMessage::ToggleDarkMode => self.toggle_theme(),
                DashboardMessage::SearchChanged(query) => {
                    let navbar = &mut self.dashboard.navbar;
                    navbar.search_query = query;
                    navbar.search_results = navbar.search();
                }
            }
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
