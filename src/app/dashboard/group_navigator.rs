use std::sync::Arc;

use iced::widget::text_editor::Content;
use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Task};
use iced::widget::{button, column, container, mouse_area, row, scrollable, space, text};
use sqlx::SqlitePool;

use crate::icon;
use crate::services::group::{GroupRow, load_groups};
use crate::utils::formatting::clamp_name;
use super::{GroupInfo, ItemBox, ItemForm};


#[derive(Debug, Clone)]
pub struct GroupNavigator {
    pub group_navigator_open: bool,
    pub group_list: Vec<GroupInfo>,
    pub group_hovered: Option<i32>,
    pub group_opened: Option<i32>,

    pub pool: Option<Arc<SqlitePool>>,
}

#[derive(Debug, Clone)]
pub enum GroupNavigatorMessage {
    SetPool(Arc<SqlitePool>),
    LoadGroupNavigator,
    SetGroupList(Result<Vec<GroupRow>, String>),
    SetGroupHovered(Option<i32>),

    ToggleGroupNavigatorOpen,
    SetCurrentGroup(GroupInfo),
    ReloadGroup,

    OpenNewGroupBox,
    OpenDeleteGroupBox(GroupInfo),
}

impl GroupNavigator {
    pub fn new(pool: Arc<SqlitePool>) -> (Self, Task<GroupNavigatorMessage>) {
        (
            Self {
                group_navigator_open: true,
                group_list: Vec::new(),
                group_opened: None,
                group_hovered: None,
                pool: None,
            },

            Task::done(GroupNavigatorMessage::SetPool(pool))
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
                .on_press(GroupNavigatorMessage::OpenNewGroupBox)
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
                scrollable(
                    container(
                        self.group_list.iter().fold(
                            column([])
                                .spacing(10),
                            |column, group| column.push(self.group_card_view(group)),
                        )
                    )
                    .padding(Padding {
                        top: 0.0,
                        right: 10.0,
                        bottom: 0.0,
                        left: 0.0,
                    })
                )
            ]
        )
        .height(Length::Fill)
        .width(
            match self.group_navigator_open {
                true => Length::Fixed(250.0),
                false => Length::Fixed(60.0),
            }
        )
        .padding(Padding {
            top: 10.0,
            right: 0.0, // Filled by the scrollable
            bottom: 10.0,
            left: 10.0,
        })
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
        let opened = match self.group_opened {
            Some(id) => id == group_info.id,
            None => false,
        };

        let hovered = match self.group_hovered {
            Some(id) => id == group_info.id,
            None => false,
        };

        mouse_area(
            button(
                container(
                    row![
                        // Card icon
                        match group_info.name.as_str() {
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
                                text(clamp_name(&group_info.name, 16))
                                    .style(|_| text::Style {
                                        color: Some(Color::from_rgb(1.0, 1.0, 1.0)),
                                        ..Default::default()
                                    }),

                                // Space
                                space()
                                    .width(Length::Fill),

                                // Card item count
                                match hovered {
                                    true => container(
                                        button(
                                            icon::trash()
                                                .size(13.0)
                                                .color(Color::from_rgb(0.9, 0.1, 0.1))
                                        )
                                        .on_press(GroupNavigatorMessage::OpenDeleteGroupBox(group_info.clone()))
                                        .padding(0)
                                        .style(|_, _| button::Style {
                                            background: None,
                                            ..Default::default()
                                        })
                                    ),
                                    false => container(
                                        text(&group_info.item_count)
                                            .style(|_| text::Style {
                                                color: Some(Color::from_rgb(1.0, 1.0, 1.0)),
                                                ..Default::default()
                                            })
                                    ),
                                }
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
            .on_press(GroupNavigatorMessage::SetCurrentGroup(group_info.clone()))
            .width(Length::Fill)
            .padding(8)
            .style(move |_, status| button::Style {
                background: if opened {
                    Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.3)))
                } else {
                    match status {
                        button::Status::Hovered | button::Status::Pressed => Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.3))),
                        _ => None,
                    }
                },
                ..Default::default()
            })
        )
        .on_enter(GroupNavigatorMessage::SetGroupHovered(Some(group_info.id)))
        .on_exit(GroupNavigatorMessage::SetGroupHovered(None))
        .into()
    }

    pub fn update(&mut self, message: GroupNavigatorMessage, current_group: &mut GroupInfo, item_box: &mut ItemBox, item_form: &mut ItemForm) -> Task<GroupNavigatorMessage> {
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
            GroupNavigatorMessage::SetGroupList(groups) => {
                match groups {
                    Ok(groups) => self.group_list = Self::group_row_to_group_info(groups),
                    Err(e) => {
                        eprintln!("Failed to load groups: {}", e);
                        self.group_list = Vec::new()
                    },
                }

                // Set current group

                // Check if current group is not deleted in new list
                // TODO: Two iterations, try optimizing this
                let current_group_match = self.group_list
                    .iter()
                    .find(|group| current_group.id == group.id);

                match current_group_match {
                    Some(group) => Task::done(GroupNavigatorMessage::SetCurrentGroup(group.clone())),
                    None => match self.group_list.first() {
                        // Current group is deleted in new list
                        // Setting first group as current group
                        Some(first) => Task::done(GroupNavigatorMessage::SetCurrentGroup(first.clone())),
                        // Group list is empty
                        None => Task::done(GroupNavigatorMessage::SetCurrentGroup(GroupInfo::default()))
                    }
                }
            },
            GroupNavigatorMessage::SetGroupHovered(hovered) => {
                self.group_hovered = hovered;
                Task::none()
            },
            GroupNavigatorMessage::ToggleGroupNavigatorOpen =>{
                self.group_navigator_open = !self.group_navigator_open;
                Task::none()
            },
            GroupNavigatorMessage::SetCurrentGroup(current_g) => {
                self.group_opened = Some(current_g.id);
                *current_group = current_g;
                Task::done(GroupNavigatorMessage::ReloadGroup)
            },
            GroupNavigatorMessage::ReloadGroup => {
                // Intercepted by the dashbaord
                Task::none()
            },
            GroupNavigatorMessage::OpenNewGroupBox => {
                *item_box = ItemBox::NewGroup;
                *item_form = ItemForm {
                    name: Some(String::new()),
                    description: Some(Content::new()),
                    ..Default::default()
                };
                Task::none()
            },
            GroupNavigatorMessage::OpenDeleteGroupBox(group) => {
                *item_box = ItemBox::DeleteGroup;
                *item_form = ItemForm {
                    id: Some(group.id),
                    name: Some(group.name),
                    ..Default::default()
                };
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
            })
            .collect()
    }
}
