use iced::{Alignment, Background, Border, Color, Element, Length};
use iced::widget::{Column, Space, button, column, container, row, text};

use crate::icon;
use super::{Dashboard, DashboardMessage, GroupInfo};


impl Dashboard {
    pub fn group_navigator_view(&self) -> Element<'_, DashboardMessage> {
        fn group_card_view(group_info: &GroupInfo, open: bool) -> Element<'_, DashboardMessage> {
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
                        match open {
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
            .on_press(DashboardMessage::SetCurrentGroup(group_info.clone()))
            .style(|_, status| button::Style {
                background: match status {
                    button::Status::Hovered => Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.3))),
                    _ => None,
                },
                ..Default::default()
            })
            .into()
        }


        container(
            column![
                // Add new group button
                button(
                    container(
                        row![
                            icon::plus()
                                .size(20.0)
                                .color(Color::from_rgb(0.3, 0.9, 0.4)),

                            match self.group_navigator_open {
                                true => container(
                                    text("New Group")
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
                    |column, group| column.push(group_card_view(&group, self.group_navigator_open)),
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
