use iced::font::{Family, Style};
use iced::widget::tooltip::Position;
use iced::widget::{Space, button, column, container, grid, row, text, tooltip};
use iced::{Alignment, Background, Border, Color, Element, Font, Length};

use crate::icon;
use super::GroupInfo;

#[derive(Debug, Clone, PartialEq)]
pub struct Group {

}

#[derive(Debug, Clone, PartialEq)]
pub enum GroupMessage {

}

impl Group {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn view(&self, current_group: &GroupInfo) -> Element<'_, GroupMessage> {
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
                    text(current_group.group_name.clone())
                        .color(Color::from_rgb(0.3, 0.5, 1.0)),
                    text("$")
                        .color(Color::from_rgb(1.0, 1.0, 1.0)),
                ],

                // Group description and edit button
                row![
                    // Description
                    container(
                        match current_group.group_description.clone() {
                            Some(description) => text(description)
                                .color(Color::from_rgb(0.9, 0.9, 0.9)),
                            None => text("No description available")
                                .color(Color::from_rgb(0.5, 0.5, 0.5))
                                .font(Font {
                                    family: Family::Monospace,
                                    style: Style::Italic,
                                    ..Default::default()
                                }),
                        }
                    )
                    .width(Length::Fill)
                    .padding(10)
                    .style(|_| container::Style {
                        background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.3))),
                        ..Default::default()
                    }),

                    // Space
                    Space::new()
                        .width(10.0),

                    // Edit group
                    tooltip(
                        button(
                            column![
                                Space::new()
                                    .height(5.0),
    
                                icon::pen()
                                    .size(15.0)
                                    .color(Color::from_rgb(0.5, 0.9, 0.9))
                            ]
                            .height(Length::Fill)
                        )
                        .height(40.0)
                        .padding(0)
                        .style(|_, _| button::Style {
                            background: None,
                            ..Default::default()
                        }),
                        
                        text("Edit Group")
                            .size(15.0)
                            .color(Color::from_rgb(0.8, 0.8, 0.8)),
                        
                        Position::Top
                    ),
                ],

                // Group controls and filters
                row![
                    //Filters
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

                    // Space
                    Space::new().width(Length::Fill),

                    // Add new command
                    button(
                        row![
                            icon::plus()
                                .size(20.0)
                                .color(Color::from_rgb(0.3, 0.9, 0.4)),

                            text("Command")
                                .color(Color::from_rgb(0.3, 0.9, 0.4)),
                        ]
                        .align_y(Alignment::Center)
                        .spacing(10)
                    )
                    .style(|_, _| button::Style {
                        background: None,
                        border: Border {
                            color: Color::from_rgb(0.3, 0.9, 0.4),
                            width: 2.0,
                            radius: 4.into(),
                        },
                        ..Default::default()
                    }),

                    // Space
                    Space::new()
                        .width(Length::Fixed(10.0)),

                    // Add new script
                    button(
                        row![
                            icon::plus()
                                .size(20.0)
                                .color(Color::from_rgb(0.3, 0.9, 0.4)),

                            text("Script")
                                .color(Color::from_rgb(0.3, 0.9, 0.4)),
                        ]
                        .align_y(Alignment::Center)
                        .spacing(10)
                    )
                    .style(|_, _| button::Style {
                        background: None,
                        border: Border {
                            color: Color::from_rgb(0.3, 0.9, 0.4),
                            width: 2.0,
                            radius: 4.into(),
                        },
                        ..Default::default()
                    }),

                ],

                // Items
                grid!(
                    self.command_card_view(),
                    self.script_card_view(),
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
                width: 0.5,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
    }

    fn command_card_view(&self) -> Element<'_, GroupMessage> {
        button("command").into()
    }

    fn script_card_view(&self) -> Element<'_, GroupMessage> {
        button("script").into()
    }

    pub fn update(&mut self, message: GroupMessage) {
        match message {

        }
    }
}
