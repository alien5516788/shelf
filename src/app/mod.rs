mod home;
mod dashboard;
mod settings;

use std::sync::Arc;
use sqlx::{Pool, Sqlite, SqlitePool};
use iced::{Element, Theme, Task};

use home::{Home, HomeMessage};
use dashboard::{Dashboard, DashboardMessage};
use settings::{Settings, SettingsMessage};

use crate::data::db::init_db;


#[derive(Debug)]
pub struct App {
    title: String,
    theme: Theme,
    screen: Screen,

    home: Home,
    dashboard: Dashboard,
    settings: Settings,

    pool: Option<Arc<SqlitePool>>,
}

#[derive(Debug)]
pub enum AppMessage {
    PoolLoaded(Result<Arc<Pool<Sqlite>>, String>),

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
    pub fn new() -> (Self, Task<AppMessage>) {
        (
            Self {
                title: String::from("Shelf"),
                theme: Theme::Dracula,
                screen: Screen::Dashboard,

                home: Home::new(),
                dashboard: Dashboard::new(),
                settings: Settings::new(),

                pool: None,
            },
            Task::perform(init_db(), |result| AppMessage::PoolLoaded(result)),
        )
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        match self.screen {
            Screen::Home => self.home.view().map(|m| AppMessage::HomeMessage(m)),
            Screen::Dashboard => self.dashboard.view(&self.title).map(|m| AppMessage::DashboardMessage(m)),
            Screen::Settings => self.settings.view().map(|m| AppMessage::SettingsMessage(m)),
        }
    }

    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::PoolLoaded(result) => match result {
                Ok(pool) => {
                    self.pool = Some(pool);
                    println!("Shelf: Database pool loaded successfully");
                    Task::none()
                },
                Err(e) => {
                    eprintln!("Shelf: Failed to create database pool: {e}");
                    Task::none()
                }
            },
            AppMessage::HomeMessage(home_m) => {
                self.home.update(home_m, &mut self.screen);
                Task::none()
            },
            AppMessage::DashboardMessage(dashboard_m) => {
                self.dashboard.update(dashboard_m, &mut self.screen, &mut self.theme);
                Task::none()
            },
            AppMessage::SettingsMessage(settings_m) => {
                self.settings.update(settings_m, &mut self.screen);
                Task::none()
            }
        }
    }

    pub fn title(&self) -> String {
        match self.screen {
            Screen::Home => self.title.clone(),
            Screen::Dashboard => String::from(format!("{} - Dashboard", self.title)),
            Screen::Settings => String::from(format!("{} - Settings", self.title))
        }
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
