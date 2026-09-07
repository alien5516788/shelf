use std::sync::Arc;

use iced::{Alignment, Background, Border, Color, Element, Length, Task};
use iced::widget::{space, button, column, container, row, text};
use sqlx::SqlitePool;

use crate::app::dashboard::ItemDialog;
use crate::icon;
use crate::services::group::{load_groups};
use crate::utils::formatting::clamp_name;
use super::GroupInfo;


#[derive(Debug, Clone)]
pub struct GroupNavigator {
    pub group_navigator_open: bool,
    pub group_list: Vec<GroupInfo>,
}

#[derive(Debug, Clone)]
pub enum GroupNavigatorMessage {
    LoadGroupNavigator(Arc<SqlitePool>),
    SetGroupList(Vec<GroupInfo>),
    ToggleGroupNavigatorOpen,
    SetCurrentGroup(GroupInfo),
    OpenItemDialog(ItemDialog)
}

impl GroupNavigator {
    pub fn new(pool: Arc<SqlitePool>) -> (Self, Task<GroupNavigatorMessage>) {
        (
            Self {
                group_navigator_open: true,
                group_list: Vec::from([]),
            },

            Task::done(GroupNavigatorMessage::LoadGroupNavigator(pool))
        )
    }

    pub fn view(&self) -> Element<'_, GroupNavigatorMessage> {
        container(
            column![
                // Collapse side bar
                button(
                    icon::menu()
                        .size(20.0)
                        .color(Color::from_rgb(0.3, 0.4, 0.6))
                )
                .style(|_, _| button::Style {
                    background: None,
                    ..Default::default()
                })
                .on_press(GroupNavigatorMessage::ToggleGroupNavigatorOpen),

                // Space
                space()
                    .height(Length::Fixed(10.0)),

                // Add new group button
                button(
                    container(
                        row![
                            icon::plus()
                                .size(20.0)
                                .color(Color::from_rgb(0.3, 0.9, 0.4)),

                            match self.group_navigator_open {
                                true => container(
                                    text("Group")
                                        .color(Color::from_rgb(0.3, 0.9, 0.4))
                                ),
                                false => container(
                                    space()
                                )
                                .width(0),
                            }
                        ]
                        .spacing(20)
                        .align_y(Alignment::Center)
                    )
                    .width(Length::Fill)
                    .align_x(Alignment::Center)
                )
                .on_press(GroupNavigatorMessage::OpenItemDialog(ItemDialog::GroupNew))
                .style(|_, _| button::Style {
                    border: Border {
                        color: Color::from_rgb(0.3, 0.9, 0.4),
                        width: 2.0,
                        radius: 4.into(),
                    },
                    ..Default::default()
                }),

                // Space
                space()
                    .height(Length::Fixed(20.0)),

                // Group list
                self.group_list.iter().fold(
                    column([])
                        .spacing(10),
                    |column, group| column.push(self.group_card_view(group)),
                ),
            ]
        )
        .height(Length::Fill)
        .width(
            match self.group_navigator_open {
                true => Length::Fixed(250.0),
                false => Length::Fixed(60.0),
            }
        )
        .padding(10)
        .style(|_| container::Style {
            border: Border {
                color: Color::from_rgb(0.4, 0.4, 0.4),
                width: 0.5,
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
    }

    fn group_card_view(&self, group_info: &GroupInfo) -> Element<'_, GroupNavigatorMessage> {
        button(
            container(
                row![
                    // Card icon
                    match group_info.group_name.as_str() {
                        "Recent" => icon::history()
                            .size(20.0)
                            .color(Color::from_rgb(0.5, 0.9, 0.9)),
                        "Favorite" => icon::star()
                            .size(20.0)
                            .color(Color::from_rgb(0.5, 0.9, 0.9)),
                        _ => icon::group_box()
                            .size(20.0)
                            .color(Color::from_rgb(1.0, 0.7, 0.4)),
                    },

                    // Card details
                    match self.group_navigator_open {
                        true => row![
                            // Card title
                            text(clamp_name(&group_info.group_name, 16))
                                .style(|_| text::Style {
                                    color: Some(Color::from_rgb(1.0, 1.0, 1.0)),
                                    ..Default::default()
                                }),

                            // Space
                            space()
                                .width(Length::Fill),

                            // Card item count
                            text(&group_info.item_count)
                                .style(|_| text::Style {
                                    color: Some(Color::from_rgb(1.0, 1.0, 1.0)),
                                    ..Default::default()
                                }),
                        ],
                        false => row![],
                    },
                ]
                .align_y(Alignment::Center)
                .spacing(10)
            )
            .width(Length::Fill)
            .align_x(Alignment::Center)
        )
        .width(Length::Fill)
        .padding(8)
        .on_press(GroupNavigatorMessage::SetCurrentGroup(group_info.clone()))
        .style(|_, status| button::Style {
            background: match status {
                button::Status::Hovered => Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.3))),
                _ => None,
            },
            ..Default::default()
        })
        .into()
    }

    pub fn update(&mut self, message: GroupNavigatorMessage, current_group: &mut GroupInfo, item_dialog: &mut ItemDialog) -> Task<GroupNavigatorMessage> {
        match message {
            GroupNavigatorMessage::LoadGroupNavigator(pool) => {
                Task::perform(
                    load_groups(pool),
                    |groups| match groups {
                        Ok(groups) => GroupNavigatorMessage::SetGroupList(
                            groups.into_iter().map(|group| GroupInfo {
                                group_id: group.id,
                                group_name: group.name,
                                group_description: group.description,
                                item_count: group.item_count as usize,
                            }).collect()
                        ),
                        Err(e) => {
                            eprintln!("Failed to load groups: {}", e);
                            GroupNavigatorMessage::SetGroupList(Vec::from([]))
                        },
                    }
                )
            },
            GroupNavigatorMessage::SetGroupList(groups) => {
                self.group_list = groups;

                if let Some(first) = self.group_list.first() {
                    *current_group = first.clone();
                }

                Task::none()
            },
            GroupNavigatorMessage::ToggleGroupNavigatorOpen =>{
                self.group_navigator_open = !self.group_navigator_open;
                Task::none()
            },
            GroupNavigatorMessage::SetCurrentGroup(currnt_grp) => {
                *current_group = currnt_grp;
                Task::none()
            },
            GroupNavigatorMessage::OpenItemDialog(item_dlg) => {
                *item_dialog = item_dlg;
                Task::none()
            },
        }
    }
}
