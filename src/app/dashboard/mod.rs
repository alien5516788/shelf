mod navigator;
mod group_navigator;
mod group;
mod status_bar;

use std::sync::Arc;

use iced::{Element, Task, Theme};
use iced::widget::{column, row, text};

use navigator::{Navigator, NavigatorMessage};
use group_navigator::{GroupNavigator, GroupNavigatorMessage};
use group::{Group, GroupMessage};
use sqlx::SqlitePool;
use status_bar::{StatusBar, StatusBarMessage};

use super::Screen;


#[derive(Debug)]
pub struct Dashboard {
    pub current_group: GroupInfo,

    pub navigator: Option<Navigator>,
    pub group_navigator: Option<GroupNavigator>,
    pub group: Option<Group>,
    pub status_bar: Option<StatusBar>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupInfo {
    pub group_id: String,
    pub group_name: String,
    pub group_description: Option<String>,
    pub item_count: usize,
}

#[derive(Debug, Clone)]
pub enum DashboardMessage {
    LoadDashboard(Arc<SqlitePool>),

    NavigatorMessage(NavigatorMessage),
    GroupNavigatorMessage(GroupNavigatorMessage),
    GroupMessage(GroupMessage),
    StatusBarMessage(StatusBarMessage),
}


impl Dashboard {
    pub fn new(pool: Arc<SqlitePool>) -> (Self, Task<DashboardMessage>) {
        (
            Self {
                current_group: GroupInfo {
                    group_id: "recent".to_string(),
                    group_name: "Recent".to_string(),
                    group_description: Some("This is recent group".to_string()),
                    item_count: 2,
                },
                navigator: None,
                group_navigator: None,
                group: None,
                status_bar: None,
            },

            Task::done(DashboardMessage::LoadDashboard(pool)),
        )
    }

    pub fn view(&self, title: &String) -> Element<'_, DashboardMessage> {
        let Some(navigator) = &self.navigator else {
            return text!("Loading...").into();
        };
        let Some(group_navigator) = &self.group_navigator else {
            return text!("Loading...").into();
        };
        let Some(group) = &self.group else {
            return text!("Loading...").into();
        };
        let Some(status_bar) = &self.status_bar else {
            return text!("Loading...").into();
        };

        column![
            navigator
                .view(title.clone())
                .map(DashboardMessage::NavigatorMessage),

            row![
                group_navigator
                    .view()
                    .map(DashboardMessage::GroupNavigatorMessage),

                group
                    .view(&self.current_group)
                    .map(DashboardMessage::GroupMessage),
            ],

            status_bar
                .view()
                .map(DashboardMessage::StatusBarMessage),
        ]
        .into()
    }

    pub fn update(&mut self, message: DashboardMessage, screen: &mut Screen, theme: &mut Theme) -> Task<DashboardMessage> {
        match message {
            DashboardMessage::LoadDashboard(pool) => {
                let (navigator, navigator_task) = Navigator::new();
                self.navigator = Some(navigator);

                let (group_navigator, group_navigator_task) = GroupNavigator::new(pool);
                self.group_navigator = Some(group_navigator);

                let (group, task) = Group::new();
                self.group = Some(group);

                let (status_bar, status_bar_task) = StatusBar::new();
                self.status_bar = Some(status_bar);

                Task::batch([
                    navigator_task.map(|m| DashboardMessage::NavigatorMessage(m)),
                    group_navigator_task.map(|m| DashboardMessage::GroupNavigatorMessage(m)),
                    task.map(|m| DashboardMessage::GroupMessage(m)),
                    status_bar_task.map(|m| DashboardMessage::StatusBarMessage(m)),
                ])
            },
            DashboardMessage::NavigatorMessage(navigator_m) => match &mut self.navigator {
                Some(navigator) => navigator.update(navigator_m, screen, theme).map(|m| DashboardMessage::NavigatorMessage(m)),
                None => Task::none(),
            },
            DashboardMessage::GroupNavigatorMessage(group_navigator_m) => match &mut self.group_navigator {
                Some(group_navigator) => group_navigator.update(group_navigator_m, &mut self.current_group).map(|m| DashboardMessage::GroupNavigatorMessage(m)),
                None => Task::none(),
            },
            DashboardMessage::GroupMessage(group_m) => match &mut self.group {
                Some(group) => group.update(group_m).map(|m| DashboardMessage::GroupMessage(m)),
                None => Task::none(),
            },
            DashboardMessage::StatusBarMessage(status_bar_m) => match &mut self.status_bar {
                Some(status_bar) => status_bar.update(status_bar_m).map(|m| DashboardMessage::StatusBarMessage(m)),
                None => Task::none(),
            },
        }
    }
}
