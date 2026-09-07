mod home;
mod dashboard;
mod settings;

use std::sync::Arc;
use iced::{Element, Task, Theme};
use sqlx::{Pool, Sqlite};

use home::{Home, HomeMessage};
use dashboard::{Dashboard, DashboardMessage};
use settings::{Settings, SettingsMessage};

use crate::components::loading_screen::loading_screen_view;
use crate::data::db::init_db;


#[derive(Debug)]
pub struct App {
    title: String,
    theme: Theme,
    screen: Screen,

    home: Option<Home>,
    dashboard: Option<Dashboard>,
    settings: Option<Settings>,
}

#[derive(Debug)]
pub enum AppMessage {
    PoolLoaded(Result<Arc<Pool<Sqlite>>, String>),
    LoadApp(Arc<Pool<Sqlite>>),

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

                home: None,
                dashboard: None,
                settings: None,
            },

            Task::perform(
                init_db(),
                |pool| AppMessage::PoolLoaded(pool),
            ),
        )
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        match self.screen {
            Screen::Home => match &self.home {
                Some(home) => home.view().map(|m| AppMessage::HomeMessage(m)),
                None => loading_screen_view(),
            },
            Screen::Dashboard => match &self.dashboard {
                Some(dashboard) => dashboard.view(&self.title).map(|m| AppMessage::DashboardMessage(m)),
                None => loading_screen_view(),
            },
            Screen::Settings => match &self.settings {
                Some(settings) => settings.view().map(|m| AppMessage::SettingsMessage(m)),
                None => loading_screen_view(),
            },
        }
    }

    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::PoolLoaded(pool) => match pool {
                Ok(p) => {
                    println!("Shelf: Database pool loaded successfully");
                    Task::done(AppMessage::LoadApp(p))
                },
                Err(e) => {
                    eprintln!("Shelf: Failed to create database pool: {}", e);
                    Task::none()
                },
            },
            AppMessage::LoadApp(pool) => {
                let (home, home_task) = Home::new();
                self.home = Some(home);

                let (dashboard, dashboard_task) = Dashboard::new(pool.clone());
                self.dashboard = Some(dashboard);

                let (settings, settings_task) = Settings::new(pool);
                self.settings = Some(settings);

                Task::batch([
                    home_task.map(|m| AppMessage::HomeMessage(m)),
                    dashboard_task.map(|m| AppMessage::DashboardMessage(m)),
                    settings_task.map(|m| AppMessage::SettingsMessage(m)),
                ])
            },
            AppMessage::HomeMessage(home_m) => match &mut self.home {
                Some(home) => home.update(home_m, &mut self.screen).map(|m| AppMessage::HomeMessage(m)),
                None => Task::none(),
            },
            AppMessage::DashboardMessage(dashboard_m) => match &mut self.dashboard {
                Some(dashboard) => dashboard.update(dashboard_m, &mut self.screen, &mut self.theme).map(|m| AppMessage::DashboardMessage(m)),
                None => Task::none(),
            },
            AppMessage::SettingsMessage(settings_m) => match &mut self.settings {
                Some(settings) => settings.update(settings_m, &mut self.screen).map(|m| AppMessage::SettingsMessage(m)),
                None => Task::none(),
            },
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
