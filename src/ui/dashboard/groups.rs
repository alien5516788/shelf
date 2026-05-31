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

#[derive(Debug, PartialEq)]
struct Group {
    group_id: String,
    group_name: String,
    item_count: usize,
}

impl Groups {
    pub fn new() -> Self {
        let group_list: Vec<Group> = Vec::from([
            Group {
                group_id: "favorite".to_string(),
                group_name: "Favorite".to_string(),
                item_count: 0,
            },
            Group {
                group_id: "recent".to_string(),
                group_name: "Recent".to_string(),
                item_count: 2,
            },
            Group {
                group_id: "cus2346245723tom".to_string(),
                group_name: "Custom 1".to_string(),
                item_count: 0,
            },
            Group {
                group_id: "custo2342662m2".to_string(),
                group_name: "Custom 2".to_string(),
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
        container(
            column![
                // Collapse side bar
                button(
                    icon::menu()
                        .size(20.0)
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
                .padding(8)
                .width(220)
                .style(|_, _| button::Style {
                    background: None,
                    border: Border {
                        color: Color::from_rgb(0.3, 0.9, 0.4),
                        width: 2.0,
                        radius: 4.into(),
                    },
                    ..Default::default()
                }),

                Space::new()
                    .height(Length::Fixed(20.0)),

                // Group list
                self.group_list.iter().fold(
                    Column::new().spacing(10),
                    |column, group| column.push(GroupCard::view(group.group_name.clone(), group.item_count)),
                ),
            ]
        )
        .height(Length::Fill)
        .padding(10)
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
    fn view(group_name: String, item_count: usize) -> Element<'static, DashboardMessage> {
        button(
            row![
                // Card icon
                match group_name.as_str() {
                    "Favorite" => icon::star()
                        .size(20.0)
                        .color(Color::from_rgb(0.5, 0.9, 0.9)),
                    "Recent" => icon::history()
                        .size(20.0)
                        .color(Color::from_rgb(0.5, 0.9, 0.9)),
                    _ => icon::group_box()
                        .size(20.0)
                        .color(Color::from_rgb(1.0, 0.7, 0.4)),
                },
                Space::new()
                    .width(Length::Fixed(10.0)),
                
                // Card title
                text(group_name)
                    .style(|_| text::Style {
                        color: Some(Color::from_rgb(1.0, 1.0, 1.0)),
                        ..Default::default()
                    }),
                Space::new()
                    .width(Length::Fill),
                
                // Card item count
                text(item_count)
                    .style(|_| text::Style {
                        color: Some(Color::from_rgb(1.0, 1.0, 1.0)),
                        ..Default::default()
                    }),
            ]
        )
        .on_press(DashboardMessage::ToggleDarkMode)
        .padding(8)
        .width(220)
        .style(|_, status| button::Style {
            background: match status {
                button::Status::Hovered => Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.3))),
                _ => None,
            },
            ..Default::default()
        })
        .into()
    }
}
