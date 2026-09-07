mod navigator;
mod group_navigator;
mod group;
mod status_bar;

use std::sync::Arc;
use iced::{Border, Color, Element, Length, Task, Theme};
use iced::widget::{center, column, container, row, space, stack, text};
use sqlx::SqlitePool;

use navigator::{Navigator, NavigatorMessage};
use group_navigator::{GroupNavigator, GroupNavigatorMessage};
use group::{Group, GroupMessage};
use status_bar::{StatusBar, StatusBarMessage};

use crate::components::loading_screen::loading_screen_view;
use crate::components::modal::modal_view;

use super::Screen;


#[derive(Debug)]
pub struct Dashboard {
    pub current_group: GroupInfo,
    pub item_dialog: ItemDialog,

    pub navigator: Option<Navigator>,
    pub group_navigator: Option<GroupNavigator>,
    pub group: Option<Group>,
    pub status_bar: Option<StatusBar>,
}

#[derive(Debug, Clone)]
pub struct GroupInfo {
    pub group_id: i32,
    pub group_name: String,
    pub group_description: Option<String>,
    pub item_count: usize,
}

#[derive(Debug, Clone)]
pub enum ItemDialog {
    None,
    GroupNew,
    GroupEdit(i64),
    GroupDel(i64),
    CommandNew,
    CommandEdit(i64),
    CommandDel(i64),
    ScriptNew,
    ScriptEdit(i64),
    ScriptDel(i64),
}


#[derive(Debug, Clone)]
pub enum DashboardMessage {
    LoadDashboard(Arc<SqlitePool>),
    CloseItemDialog,

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
                    group_id: 0,
                    group_name: "Recent".to_string(),
                    group_description: Some("This is recent group".to_string()),
                    item_count: 2,
                },
                item_dialog: ItemDialog::None,
                navigator: None,
                group_navigator: None,
                group: None,
                status_bar: None,
            },

            Task::done(DashboardMessage::LoadDashboard(pool)),
        )
    }

    pub fn view(&self, title: &String) -> Element<'_, DashboardMessage> {
        // TODO: I know if-else, if-let . But not let-else. Tell the AI to explain this
        let Some(navigator) = &self.navigator else {
            return loading_screen_view();
        };
        let Some(group_navigator) = &self.group_navigator else {
            return loading_screen_view();
        };
        let Some(group) = &self.group else {
            return loading_screen_view();
        };
        let Some(status_bar) = &self.status_bar else {
            return loading_screen_view();
        };

        stack![
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
            ],

            // Popup to create/edit/delete items
            match self.item_dialog {
                ItemDialog::None => space().into(),
                _ => self.add_item_dialog_view(),
            },
        ]
        .into()
    }

    fn add_item_dialog_view(&self) -> Element<'_, DashboardMessage> {
        modal_view(
            container(
                center(
                    text("Hello there")
                )
            )
            .style(|_| container::Style {
                border: Border {
                    color: Color::from_rgb(0.4, 0.4, 0.4),
                    width: 0.5,
                    ..Default::default()
                },
                ..Default::default()
            })
            .height(Length::Fixed(200.0))
            .width(Length::Fixed(400.0)),

            DashboardMessage::CloseItemDialog,
        )
    }

    pub fn update(&mut self, message: DashboardMessage, screen: &mut Screen, theme: &mut Theme) -> Task<DashboardMessage> {
        match message {
            DashboardMessage::LoadDashboard(pool) => {
                let (navigator, navigator_task) = Navigator::new();
                self.navigator = Some(navigator);

                let (group_navigator, group_navigator_task) = GroupNavigator::new(pool);
                self.group_navigator = Some(group_navigator);

                let (group, group_task) = Group::new();
                self.group = Some(group);

                let (status_bar, status_bar_task) = StatusBar::new();
                self.status_bar = Some(status_bar);

                Task::batch([
                    navigator_task.map(|m| DashboardMessage::NavigatorMessage(m)),
                    group_navigator_task.map(|m| DashboardMessage::GroupNavigatorMessage(m)),
                    group_task.map(|m| DashboardMessage::GroupMessage(m)),
                    status_bar_task.map(|m| DashboardMessage::StatusBarMessage(m)),
                ])
            },
            DashboardMessage::CloseItemDialog => {
                self.item_dialog = ItemDialog::None;
                Task::none()
            },
            DashboardMessage::NavigatorMessage(navigator_m) => match &mut self.navigator {
                Some(navigator) => navigator.update(navigator_m, screen, theme).map(|m| DashboardMessage::NavigatorMessage(m)),
                None => Task::none(),
            },
            DashboardMessage::GroupNavigatorMessage(group_navigator_m) => match &mut self.group_navigator {
                Some(group_navigator) => group_navigator.update(group_navigator_m, &mut self.current_group, &mut self.item_dialog).map(|m| DashboardMessage::GroupNavigatorMessage(m)),
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
