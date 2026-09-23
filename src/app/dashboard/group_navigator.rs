use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Theme};
use iced::widget::{Column, button, center_x, column, container, mouse_area, row, scrollable, space, text};

use super::GroupInfo;
use crate::app::dashboard::{Dashboard, DashboardMessage};
use crate::icon;
use crate::utils::font_size::{sv, sv_16, sv_20};
use crate::utils::formatting::clamp_name;


impl Dashboard {
    pub fn group_navigator_view(&self) -> Element<'_, DashboardMessage> {
        container(
            column![
                // Collapse side bar
                row![
                    match self.group_navigator_open {
                        true => space().width(Length::Fill),
                        false => space(),
                    },

                    button(match self.group_navigator_open {
                        true => icon::panel_left_close().size(sv(20.0)),
                        false => icon::panel_left_open().size(sv(20.0)),
                    })
                    .on_press(DashboardMessage::SetGroupNavigatoOpen)
                    .padding([0, 8])
                    .style(|theme, _| button::Style {
                        text_color: theme.palette().primary,
                        ..Default::default()
                    })
                ],


                space()
                    .height(10.0),

                // Add new group
                button(
                    center_x(
                        row![
                            icon::plus()
                                .size(sv(20.0))
                                .style(|theme| text::Style {
                                    color: Some(theme.palette().success),
                                    ..Default::default()
                                }),

                            text(match self.group_navigator_open {
                                true => "Group",
                                false => "",
                            })
                            .size(sv_16())
                            .style(|theme: &Theme| text::Style {
                                color: Some(theme.palette().success),
                                ..Default::default()
                            }),
                        ]
                        .align_y(Alignment::Center)
                        .spacing(10)
                    )
                )
                .on_press(DashboardMessage::OpenNewGroupBox)
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
                            |column, group| column.push(Self::group_card_view(group, group.id == self.group_selected.id, &self.group_navigator_open)),
                        )
                    )
                )
            ]
        )
        .height(Length::Fill)
        .width(match self.group_navigator_open {
            true => sv(250.0) as u32,
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

    fn group_card_view(group: &GroupInfo, selected: bool, open: &bool) -> Element<'static, DashboardMessage> {
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
                                .size(sv_20())
                                .color(Color::from_rgb(0.5, 0.9, 0.9)),
                            "Favourites" => icon::star()
                                .size(sv_20())
                                .color(Color::from_rgb(0.5, 0.9, 0.9)),
                            _ => icon::group_box()
                                .size(sv_20())
                                .color(Color::from_rgb(1.0, 0.7, 0.4)),
                        },

                        // Group details
                        match open {
                            true => row![
                                // Group title
                                text(clamp_name(name, 13))
                                    .size(sv_16())
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
                                                .size(sv(14.0))
                                        )
                                        .on_press(DashboardMessage::OpenDeleteGroupBox(group.clone()))
                                        .padding(0)
                                        .style(|theme, _| button::Style {
                                            text_color: theme.palette().danger,
                                            ..Default::default()
                                        })
                                    ),
                                    false => container(
                                        // Count
                                        text(item_count)
                                            .size(sv(15.0))
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
            .on_press(DashboardMessage::SetGroupSelected(group.clone()))
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
        .on_enter(DashboardMessage::SetGroupHovered(id))
        .on_exit(DashboardMessage::SetGroupHovered(id))
        .into()
    }
}
