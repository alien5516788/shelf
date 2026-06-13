use iced::{Alignment, Background, Border, Color, Element, Length};
use iced::widget::{Column, Space, button, column, container, row, text};

use crate::icon;
use super::{Dashboard, DashboardMessage, GroupInfo};


impl Dashboard {
    pub fn group_navigator_view(&self) -> Element<'_, DashboardMessage> {
        fn group_card_view(group_info: &GroupInfo, open: bool) -> Element<'_, DashboardMessage> {
            button(
                row![
                    // Card icon
                    group_icon_view(&group_info.group_name),
                    Space::new()
                        .width(Length::Fixed(10.0)),

                    // Card details
                    match open {
                        true => row![
                            // Card title
                            text(&group_info.group_name)
                                .style(|_| text::Style {
                                    color: Some(Color::from_rgb(1.0, 1.0, 1.0)),
                                    ..Default::default()
                                }),

                            Space::new()
                                .width(Length::Fill),

                            // Card item count
                            text(&group_info.item_count)
                                .style(|_| text::Style {
                                    color: Some(Color::from_rgb(1.0, 1.0, 1.0)),
                                    ..Default::default()
                                }),
                        ],
                        false => row![
                            Space::new().width(0)
                        ],
                    },
                ]
            )
            .on_press(DashboardMessage::ToggleTheme)
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

        fn group_icon_view(group_name: &str) -> Element<'static, DashboardMessage> {
            let icon = match group_name {
                "Favorite" => icon::star()
                    .size(20.0)
                    .color(Color::from_rgb(0.5, 0.9, 0.9)),
                "Recent" => icon::history()
                    .size(20.0)
                    .color(Color::from_rgb(0.5, 0.9, 0.9)),
                _ => icon::group_box()
                    .size(20.0)
                    .color(Color::from_rgb(1.0, 0.7, 0.4)),
            };

            icon.into()
        }


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
                })
                .on_press(DashboardMessage::ToggleGroupNavigatorOpen),

                Space::new()
                    .height(Length::Fixed(20.0)),

                // Add a new group
                button(
                    container(
                        row![
                            icon::plus()
                                .color(Color::from_rgb(0.3, 0.9, 0.4)),

                            match self.group_navigator_open {
                                true => container(text("New Group")
                                    .color(Color::from_rgb(0.3, 0.9, 0.4))),
                                false => container(Space::new()).width(0),
                            }
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
                    |column, group| column.push(group_card_view(&group, self.group_navigator_open)),
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
