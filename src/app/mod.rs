mod home;
mod dashboard;
mod settings;

use iced::{Element, Font, Theme};

use home::{Home, HomeMessage};
use dashboard::{Dashboard, DashboardMessage};
use settings::{Settings, SettingsMessage};


#[derive(Debug, PartialEq)]
pub struct App {
    title: String,
    theme: Theme,
    font: Font,
    screen: Screen,

    home: Home,
    dashboard: Dashboard,
    settings: Settings,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppMessage {
    HomeMessage(HomeMessage),
    DashboardMessage(DashboardMessage),
    SettingsMessage(SettingsMessage),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Home,
    Dashboard,
    Settings,
}

impl App {
    pub fn new() -> Self {
        Self {
            title: "Shelf".to_string(),
            theme: Theme::Dracula,
            font: Font::MONOSPACE,
            screen: Screen::Dashboard,

            home: Home::new(),
            dashboard: Dashboard::new(),
            settings: Settings::new(),
        }
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        match self.screen {
            Screen::Home => self.home.view().map(|m| AppMessage::HomeMessage(m)),
            Screen::Dashboard => self.dashboard.view(&self.title).map(|m| AppMessage::DashboardMessage(m)),
            Screen::Settings => self.settings.view().map(|m| AppMessage::SettingsMessage(m)),
        }
    }

    pub fn update(&mut self, message: AppMessage) -> () {
        match message {
            AppMessage::HomeMessage(home_m) => self.home.update(home_m, &mut self.screen),
            AppMessage::DashboardMessage(dashboard_m) => self.dashboard.update(dashboard_m, &mut self.screen, &mut self.theme),
            AppMessage::SettingsMessage(settings_m) => self.settings.update(settings_m, &mut self.screen)
        }
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
