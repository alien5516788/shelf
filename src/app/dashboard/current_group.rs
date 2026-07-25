use iced::font::{Family, Style};
use iced::widget::{Space, button, column, container, grid, row, text};
use iced::{Background, Border, Color, Element, Font, Length};

use crate::icon;
use super::{Dashboard, DashboardMessage};


impl Dashboard {
    pub fn current_group_view(&self) -> Element<'_, DashboardMessage> {
        fn command_card_view() -> Element<'static, DashboardMessage> {
            button("command").into()
        }

        fn script_card_view() -> Element<'static, DashboardMessage> {
            button("script").into()
        }

        container(
            column![
                // Fancy group name
                row![
                    text("user@shelf")
                        .color(Color::from_rgb(0.3, 0.9, 0.4)),
                    text(":")
                        .color(Color::from_rgb(1.0, 1.0, 1.0)),
                    text("~/")
                        .color(Color::from_rgb(0.3, 0.5, 1.0)),
                    text(&self.current_group.group_name)
                        .color(Color::from_rgb(0.3, 0.5, 1.0)),
                    text("$")
                        .color(Color::from_rgb(1.0, 1.0, 1.0)),
                ],

                // Group description
                container(
                    match &self.current_group.group_description {
                        Some(description) => text(description)
                            .color(Color::from_rgb(0.9, 0.9, 0.9)),
                        None => text("No description available")
                            .color(Color::from_rgb(0.5, 0.5, 0.5))
                            .font(Font {
                                family: Family::Monospace,
                                style: Style::Italic,
                                ..Default::default()
                            })
                    }
                )
                .width(Length::Fill)
                .padding(10)
                .style(|_| container::Style {
                    background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.3))),
                    ..Default::default()
                })
                ,
                
                // Group controls and filters
                row![
                    button(icon::square_terminal()
                        .size(20.0)
                        .color(Color::from_rgb(0.5, 0.9, 0.9)))
                        .style(|_, _| button::Style {
                            background: None,
                            ..Default::default()
                        }),
                    button(icon::code_xml()
                        .size(20.0)
                        .color(Color::from_rgb(0.5, 0.9, 0.9)))
                        .style(|_, _| button::Style {
                            background: None,
                            ..Default::default()
                        }),
                    button(icon::a_large_small()
                        .size(20.0)
                        .color(Color::from_rgb(0.5, 0.9, 0.9)))
                        .style(|_, _| button::Style {
                            background: None,
                            ..Default::default()
                        }),

                    Space::new().width(Length::Fill),

                    button(icon::pen()
                        .size(20.0)
                        .color(Color::from_rgb(0.5, 0.9, 0.9)))
                        .style(|_, _| button::Style {
                            background: None,
                            ..Default::default()
                        }),
                    button("Add command")
                        .style(|_, _| button::Style {
                            background: None,
                            ..Default::default()
                        }),
                    button("Add snippet")
                        .style(|_, _| button::Style {
                            background: None,
                            ..Default::default()
                        }),
                ],

                // Items
                grid!(
                    command_card_view(),
                    script_card_view(),
                )
            ]
            .spacing(10)
        )
        .height(Length::Fill)
        .width(Length::Fill)
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
