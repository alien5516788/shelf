use std::sync::Arc;

use iced::widget::text_editor::Content;
use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Task, Theme};
use iced::widget::{Column, button, center_x, column, container, mouse_area, row, scrollable, space, text};
use sqlx::SqlitePool;

use crate::icon;
use crate::services::group::{GroupRow, load_groups};
use crate::utils::formatting::clamp_name;
use super::{GroupInfo, ItemEditor, Dialog};


#[derive(Debug, Clone)]
pub struct GroupNavigator {
    pub open: bool,
    pub group_list: Vec<GroupInfo>,

    pub pool: Option<Arc<SqlitePool>>,
}

#[derive(Debug, Clone)]
pub enum GroupNavigatorMessage {
    SetPool(Arc<SqlitePool>),
    LoadGroupNavigator,
    SetCurrentGroup(i32),
    ReloadGroup,
    SetGroupList(Result<Vec<GroupRow>, String>),
    ToggleOpen,
    ToggleGroupHovered(i32),

    OpenNewGroupBox,
    OpenDeleteGroupBox(GroupInfo),
}

impl GroupNavigator {
    pub fn new(pool: Arc<SqlitePool>) -> (Self, Task<GroupNavigatorMessage>) {
        (
            Self {
                open: true,
                group_list: Vec::new(),
                pool: None,
            },

            Task::done(GroupNavigatorMessage::SetPool(pool))
        )
    }

    pub fn view(&self, current_group: &GroupInfo) -> Element<'_, GroupNavigatorMessage> {
        container(
            column![
                // Collapse side bar
                button(icon::menu().size(20.0))
                    .style(|theme, _| button::Style {
                        text_color: theme.palette().primary,
                        ..Default::default()
                    })
                    .on_press(GroupNavigatorMessage::ToggleOpen),

                space()
                    .height(10.0),

                // Add new group
                button(
                    center_x(
                        row![
                            icon::plus()
                                .size(20.0)
                                .style(|theme| text::Style {
                                    color: Some(theme.palette().success),
                                    ..Default::default()
                                }),

                            text("Group")
                                .style(|theme: &Theme| text::Style {
                                    color: Some(theme.palette().success),
                                    ..Default::default()
                                }),
                        ]
                        .align_y(Alignment::Center)
                        .spacing(10)
                    )
                )
                .on_press(GroupNavigatorMessage::OpenNewGroupBox)
                .style(|theme: &Theme, _| button::Style {
                    border: Border {
                        color: theme.palette().success,
                        width: 2.0,
                        radius: 4.into(),
                    },
                    ..Default::default()
                }),

                space()
                    .height(Length::Fixed(20.0)),

                // Group list
                scrollable(
                    container(
                        self.group_list.iter().fold(
                            Column::new()
                                .spacing(10),
                            |column, group| column.push(Self::group_card_view(group, group.id == current_group.id, &self.open)),
                        )
                    )
                )
            ]
        )
        .height(Length::Fill)
        .width(match self.open {
            true => 250,
            false => 70,
        })
        .padding(10)
        .style(|theme| container::Style {
            border: Border {
                color: theme.palette().primary,
                width: 0.5,
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
    }

    fn group_card_view(group: &GroupInfo, selected: bool, open: &bool) -> Element<'static, GroupNavigatorMessage> {
        let (id, name, item_count, hovered) = (
            group.id,
            group.name.as_str(),
            group.item_count,
            group.hovered,
        );

        mouse_area(
            button(
                container(
                    row![
                        // Group icon
                        match name {
                            "Recent" => icon::history()
                                .size(20.0)
                                .color(Color::from_rgb(0.5, 0.9, 0.9)),
                            "Favourites" => icon::star()
                                .size(20.0)
                                .color(Color::from_rgb(0.5, 0.9, 0.9)),
                            _ => icon::group_box()
                                .size(20.0)
                                .color(Color::from_rgb(1.0, 0.7, 0.4)),
                        },

                        // Group details
                        match open {
                            true => row![
                                // Group title
                                text(clamp_name(name, 13))
                                    .style(|theme: &Theme| text::Style {
                                        color: Some(theme.palette().text),
                                        ..Default::default()
                                    }),

                                space()
                                    .width(Length::Fill),

                                // Item count, Delete
                                match hovered &&
                                name != "Favourites" &&
                                name != "Recent" &&
                                name != "Default" {
                                    true => container(
                                        // Trash
                                        button(
                                            icon::trash()
                                                .size(15)
                                        )
                                        .on_press(GroupNavigatorMessage::OpenDeleteGroupBox(group.clone()))
                                        .padding(0)
                                        .style(|theme, _| button::Style {
                                            text_color: theme.palette().danger,
                                            ..Default::default()
                                        })
                                    ),
                                    false => container(
                                        // Count
                                        text(item_count)
                                            .style(|theme: &Theme| text::Style {
                                                color: Some(theme.palette().primary),
                                                ..Default::default()
                                            })
                                    ),
                                }
                            ],
                            false => row![],
                        },
                    ]
                    .align_y(Alignment::Center)
                    .padding(Padding {
                        top: 0.0,
                        right: 10.0,
                        bottom: 0.0,
                        left: 0.0,
                    })
                    .spacing(15)
                )
                .width(Length::Fill)
                .align_x(Alignment::Center)
            )
            .on_press(GroupNavigatorMessage::SetCurrentGroup(id))
            .width(Length::Fill)
            .padding(8)
            .style(move |theme, status| button::Style {
                background: if selected {
                    Some(Background::Color(theme.palette().primary.scale_alpha(0.1)))
                } else {
                    match status {
                        button::Status::Hovered | button::Status::Pressed =>
                            Some(Background::Color(theme.palette().primary.scale_alpha(0.1))),
                        _ => None,
                    }
                },
                ..Default::default()
            })
        )
        .on_enter(GroupNavigatorMessage::ToggleGroupHovered(id))
        .on_exit(GroupNavigatorMessage::ToggleGroupHovered(id))
        .into()
    }

    pub fn update(&mut self, message: GroupNavigatorMessage, current_group: &mut GroupInfo, item_editor: &mut Option<ItemEditor>) -> Task<GroupNavigatorMessage> {
        match message {
            GroupNavigatorMessage::SetPool(pool) => {
                self.pool = Some(pool.clone());
                Task::done(GroupNavigatorMessage::LoadGroupNavigator)
            },
            GroupNavigatorMessage::LoadGroupNavigator => {
                let pool = match self.pool.clone() {
                    Some(pool) => pool,
                    None => return Task::none(),
                };

                Task::perform(
                    load_groups(pool.clone()),
                    GroupNavigatorMessage::SetGroupList
                )
            },
            GroupNavigatorMessage::SetCurrentGroup(id) => {
                let group = match self.find_group(id) {
                    Some(group) => group,
                    None => return Task::none(),
                };
                *current_group = group.clone();
                Task::done(GroupNavigatorMessage::ReloadGroup)
            },
            GroupNavigatorMessage::ReloadGroup => {
                // Intercepted by the dashbaord
                // Group component must be ackknowledge of the reload content of the group
                Task::none()
            },
            GroupNavigatorMessage::SetGroupList(groups) => {
                match groups {
                    Ok(groups) => self.group_list = Self::group_row_to_group_info(groups),
                    Err(e) => {
                        eprintln!("Failed to load groups: {}", e);
                        self.group_list = Vec::new()
                    },
                }

                // Set current group
                let mut default_group_id = 0;

                for group in &self.group_list {
                    if group.id == current_group.id {
                        // Current group found, return it
                        return Task::done(GroupNavigatorMessage::SetCurrentGroup(group.id));
                    } else if group.name == "Default" {
                        // Default group found, store its id as fallback
                        default_group_id = group.id;
                    }
                }

                Task::done(GroupNavigatorMessage::SetCurrentGroup(default_group_id))
            },
            GroupNavigatorMessage::ToggleOpen =>{
                self.open = !self.open;
                Task::none()
            },
            GroupNavigatorMessage::ToggleGroupHovered(id) => {
                let group = match self.find_group(id) {
                    Some(group) => group,
                    None => return Task::none(),
                };
                group.hovered = !group.hovered;
                Task::none()
            },
            GroupNavigatorMessage::OpenNewGroupBox => {
                *item_editor = Some(
                    ItemEditor {
                        name: Some(String::new()),
                        description: Some(Content::new()),
                        dialog: Dialog::NewGroup,
                        ..Default::default()
                    }
                );
                Task::none()
            },
            GroupNavigatorMessage::OpenDeleteGroupBox(group) => {
                *item_editor = Some(
                    ItemEditor {
                        id: Some(group.id),
                        name: Some(group.name),
                        dialog: Dialog::DeleteGroup,
                        ..Default::default()
                    }
                );
                Task::none()
            },
        }
    }

    fn group_row_to_group_info(groups: Vec<GroupRow>) -> Vec<GroupInfo> {
        groups
            .into_iter()
            .map(|group| GroupInfo {
                id: group.id,
                name: group.name,
                description: group.description,
                item_count: group.item_count as usize,

                hovered: false,
            })
            .collect()
    }

    fn find_group(&mut self, id: i32) -> Option<&mut GroupInfo> {
        self.group_list.iter_mut().find(|group| group.id == id)
    }
}
