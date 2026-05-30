use iced::widget::{Column, Space, button, column, container, row, text};
use iced::{Alignment, Background, Border, Color, Element, Length};

use crate::icon;
use super::DashboardMessage;


#[derive(Debug, PartialEq)]
pub struct Groups {
    group_list: Vec<Group>,
    pub search_query: String,
    pub search_results: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
enum GroupName {
    Favorite,
    Recent,
    Custom(String)
}

#[derive(Debug, PartialEq)]
struct Group {
    group_id: String,
    group_name: GroupName,
    item_count: usize,
}


impl Groups {
    pub fn new() -> Self {
        let group_list: Vec<Group> = Vec::from([
            Group {
                group_id: "favorite".to_string(),
                group_name: GroupName::Favorite,
                item_count: 0,
            },
            Group {
                group_id: "recent".to_string(),
                group_name: GroupName::Recent,
                item_count: 2,
            },
            Group {
                group_id: "custom".to_string(),
                group_name: GroupName::Custom("Custom".to_string()),
                item_count: 0,
            },
            Group {
                group_id: "custom2".to_string(),
                group_name: GroupName::Custom("Custom".to_string()),
                item_count: 5,
            },
        ]);

        Self {
            group_list: group_list,
            search_query: String::new(),
            search_results: Vec::new(),
        }
    }

    pub fn view(&self) -> Element<'_, DashboardMessage> {
        container(column![
            // Collapse side bar
            button(
                icon::menu()
                    .color(Color::from_rgb(0.3, 0.4, 0.6))
            )
            .style(|_, _| button::Style {
                background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.0))),
                ..Default::default()
            }),

            Space::new()
                .height(Length::Fixed(20.0)),

            // Add a new group
            button(
                container(
                    row![
                        icon::plus()
                            .color(Color::from_rgb(0.3, 0.9, 0.4)),
                        text("New Group")
                            .color(Color::from_rgb(0.3, 0.9, 0.4))
                    ]
                    .spacing(10)
                )
                .align_x(Alignment::Center)
            )
            .width(200)
            .style(|_, _| button::Style {
                background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.0))),
                border: Border {
                    color: Color::from_rgb(0.3, 0.9, 0.4),
                    width: 2.0,
                    radius: 4.into(),
                },
                ..Default::default()
            }),

            // Group list
            self.group_list.iter().fold(
                Column::new(),
                |column, group| column.push(GroupCard::view(group.group_name.clone(), group.item_count)),
            ),
        ])
        .height(Length::Fill)
        .padding(4)
        .style(|_| container::Style {
            border: Border {
                color: Color::from_rgb(0.4, 0.4, 0.4),
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
    }
}


struct GroupCard;

impl GroupCard {
    fn view(group_name: GroupName, item_count: usize) -> Element<'static, DashboardMessage> {
        // Convert group name to string
        let group_name = match group_name {
            GroupName::Favorite => "Favorite".to_string(),
            GroupName::Recent => "Recent".to_string(),
            GroupName::Custom(name) => name,
        };

        row![
            button(text(group_name)),
            text(item_count),
        ].into()
    }
}
