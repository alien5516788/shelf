mod navigator;
mod group_navigator;
mod group;
mod status_bar;

use iced::{Element, Theme};
use iced::widget::{column, row};

use navigator::{Navigator, NavigatorMessage};
use group_navigator::{GroupNavigator, GroupNavigatorMessage};
use group::{Group, GroupMessage};
use status_bar::{StatusBar, StatusBarMessage};

use super::Screen;


#[derive(Debug, PartialEq)]
pub struct Dashboard {
    pub current_group: GroupInfo,
    pub command_list: Vec<CommandInfo>,

    pub navigator: Navigator,
    pub group_navigator: GroupNavigator,
    pub group: Group,
    pub status_bar: StatusBar,
}

#[derive(Debug, PartialEq)]
pub struct CommandInfo {
    pub command_id: String,
    pub command_name: String,
    pub command_description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupInfo {
    pub group_id: String,
    pub group_name: String,
    pub group_description: Option<String>,
    pub item_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DashboardMessage {
    NavigatorMessage(NavigatorMessage),
    GroupNavigatorMessage(GroupNavigatorMessage),
    GroupMessage(GroupMessage),
    StatusBarMessage(StatusBarMessage),
}


impl Dashboard {
    pub fn new() -> Self {
        Self {

            // current group
            current_group: GroupInfo {
                group_id: "recent".to_string(),
                group_name: "Recent".to_string(),
                group_description: Some("This is recent group".to_string()),
                item_count: 2,
            },
            command_list: Vec::from([
                CommandInfo {
                    command_id: "tempCommandId".to_string(),
                    command_name: "tempCommandName".to_string(),
                    command_description: "tempCommandDescription".to_string(),
                },
                CommandInfo {
                    command_id: "tempCommandId".to_string(),
                    command_name: "tempCommandName".to_string(),
                    command_description: "tempCommandDescription".to_string(),
                }
            ]),
            navigator: Navigator::new(),
            group_navigator: GroupNavigator::new(),
            group: Group::new(),
            status_bar: StatusBar::new(),
        }
    }

    pub fn view(&self, title: &String) -> Element<'_, DashboardMessage> {
        column![
            self.navigator.view(title.clone()).map(|m| DashboardMessage::NavigatorMessage(m)),

            row![
                self.group_navigator.view().map(|m| DashboardMessage::GroupNavigatorMessage(m)),
                self.group.view(&self.current_group).map(|m| DashboardMessage::GroupMessage(m)),
            ],

            self.status_bar.view().map(|m| DashboardMessage::StatusBarMessage(m))
        ]
        .into()
    }

    pub fn update(&mut self, message: DashboardMessage, screen: &mut Screen, theme: &mut Theme) {
        match message {
            DashboardMessage::NavigatorMessage(navigator_m) => self.navigator.update(navigator_m, screen, theme),
            DashboardMessage::GroupNavigatorMessage(group_navigator_m) => self.group_navigator.update(group_navigator_m, &mut self.current_group),
            DashboardMessage::GroupMessage(group_m) => self.group.update(group_m),
            DashboardMessage::StatusBarMessage(status_bar_m) => self.status_bar.update(status_bar_m),
        }
    }
}
