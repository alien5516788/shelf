mod app;
mod home;
mod dashboard;
mod settings;

use std::sync::Arc;
use sqlx::{Pool, Sqlite};

use super::data::settings::{AppScreen, AppTheme};
use home::{Home, HomeMessage};
use dashboard::{Dashboard, DashboardMessage};
use settings::{Settings, SettingsMessage};


#[derive(Debug)]
pub struct App<'a: 'static> {
    title: &'a str,
    theme: AppTheme,
    screen: AppScreen,

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
