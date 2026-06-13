mod home;
mod dashboard;

use iced::{Element, Font, Theme};

use home::{Home, HomeMessage};
use dashboard::{Dashboard, DashboardMessage};

/*
 * App state, message and view are divided into screens except app update
 */

#[derive(Debug, PartialEq)]
pub struct App {
    title: String,
    theme: Theme,
    font: Font,
    screen: Screen,

    home: Home,
    dashboard: Dashboard,
}

#[derive(Debug, PartialEq)]
enum Screen {
    Home,
    Dashboard,
    Settings,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppMessage {
    HomeMessage(HomeMessage),
    DashboardMessage(DashboardMessage),
}

impl App {
    pub fn new() -> Self {
        Self {
            title: "Shelf".to_string(),
            theme: Theme::Dracula,
            font: Font::MONOSPACE,
            screen: Screen::Dashboard,

            home: Home::new(),
            dashboard: Dashboard::new("Shelf"),
        }
    }

    // App view
    pub fn view(&self) -> Element<'_, AppMessage> {
        match self.screen {
            Screen::Home => self.home.view().map(|m| AppMessage::HomeMessage(m)),
            Screen::Dashboard => self.dashboard.view().map(|m| AppMessage::DashboardMessage(m)),
            Screen::Settings => self.dashboard.view().map(|m| AppMessage::DashboardMessage(m)), // TODO: implement settings
        }
    }

    fn change_screen(&mut self, screen: Screen) {
        self.screen = screen;
    }

    fn toggle_theme(&mut self) {
        match self.theme {
            Theme::Light => self.theme = Theme::Dracula,
            Theme::Dracula => self.theme = Theme::Light,
            _ => self.theme = Theme::Light,
        }
    }
}


impl App {
    pub fn update(&mut self, message: AppMessage) -> () {
        match message {
            AppMessage::HomeMessage(home_m) => match home_m {
                HomeMessage::GotoDashboard => self.change_screen(Screen::Dashboard),
            },
            AppMessage::DashboardMessage(dashboard_m) => match dashboard_m {
                DashboardMessage::GotoHome => self.change_screen(Screen::Home),
                DashboardMessage::GotoSettings => self.change_screen(Screen::Settings),
                DashboardMessage::ToggleTheme => self.toggle_theme(),
                DashboardMessage::ToggleGroupNavigatorOpen => {
                    self.dashboard.toggle_group_navigator_open();
                }
                DashboardMessage::SearchItem(query) => {
                    self.dashboard.search_item(query);
                }
            }
        }
    }
}


impl App {
    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
