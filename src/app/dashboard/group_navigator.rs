use iced::{Alignment, Background, Border, Color, Element, Length};
use iced::widget::{Column, Space, button, column, container, row, text};

use crate::icon;
use super::GroupInfo;


#[derive(Debug, Clone, PartialEq)]
pub struct GroupNavigator {
    pub group_navigator_open: bool,
    pub group_list: Vec<GroupInfo>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GroupNavigatorMessage {
    ToggleGroupNavigatorOpen,
    SetCurrentGroup(GroupInfo),
}

impl GroupNavigator {
    pub fn new() -> Self {
        Self {
            group_navigator_open: true,
            group_list: Vec::from([
                GroupInfo {
                    group_id: "recent".to_string(),
                    group_name: "Recent".to_string(),
                    group_description: Some("This is recent group".to_string()),
                    item_count: 2,
                },
                GroupInfo {
                    group_id: "favorite".to_string(),
                    group_name: "Favorite".to_string(),
                    group_description: Some("This is favourite group".to_string()),
                    item_count: 0,
                },
                GroupInfo {
                    group_id: "cus2346245723tom".to_string(),
                    group_name: "Custom 1".to_string(),
                    group_description: Some("This is normal group".to_string()),
                    item_count: 0,
                },
                GroupInfo {
                    group_id: "custo2342662m2".to_string(),
                    group_name: "Custom custom custom custom".to_string(),
                    group_description: Some("wwwwwwwwwwwww wwwwwwwwwwwwwww wwwwwwwwwwwww wwwww w wwwww ww    wwwwwwwwwww wwwwwwwwww wwwwwww".to_string()),
                    item_count: 5,
                },
                GroupInfo {
                    group_id: "custo23sd42662m2".to_string(),
                    group_name: "wwwwwwwww wwwwwwwww".to_string(),
                    group_description: None,
                    item_count: 5,
                },
            ]),
        }
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
                Space::new()
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
                                    Space::new()
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
                .style(|_, _| button::Style {
                    border: Border {
                        color: Color::from_rgb(0.3, 0.9, 0.4),
                        width: 2.0,
                        radius: 4.into(),
                    },
                    ..Default::default()
                }),

                // Space
                Space::new()
                    .height(Length::Fixed(20.0)),

                // Group list
                self.group_list.iter().fold(
                    Column::new()
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
                            Space::new()
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

    pub fn update(&mut self, message: GroupNavigatorMessage, current_group: &mut GroupInfo) {
        match message {
            GroupNavigatorMessage::ToggleGroupNavigatorOpen => self.group_navigator_open = !self.group_navigator_open,
            GroupNavigatorMessage::SetCurrentGroup(currnt_grp) => *current_group = currnt_grp,
        }
    }

    pub fn _search_item(&mut self, _item_name: String) {
        todo!();
    }
}


fn clamp_name(name: &str, max_len: usize) -> String {
    let char_count = name.chars().count();

    if char_count <= max_len {
        name.to_string()  // Convert &str to String
    } else {
        let truncate_at = max_len.saturating_sub(3);
        name.chars().take(truncate_at).collect::<String>() + "..."
    }
}
