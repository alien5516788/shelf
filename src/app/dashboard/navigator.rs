use iced::{Alignment, Background, Border, Element, Length, Theme, color};
use iced::widget::{button, center_x, container, row, space, text, text_input};

use super::{Dashboard, DashboardMessage};
use crate::data::settings::{AppScreen, AppTheme};
use crate::icon;
use crate::utils::font_size::{sv, sv_16, sv_20};


impl Dashboard {
    pub fn navigator_view(&self, title: &'static str, theme: &AppTheme) -> Element<'_, DashboardMessage> {
        container(
            row![
                // App name
                button(
                    text(title)
                        .size(sv(18.0)))
                        .style(|theme: &Theme, _| {
                            button::Style {
                                border: Border::default(),
                                text_color: theme.palette().text,
                                ..Default::default()
                            }
                        }
                )
                .padding([0, 15])
                .on_press(DashboardMessage::SetScreen(AppScreen::Home)),

                // Search bar
                center_x(
                    container(
                        row![
                            // Search button
                            button(icon::search().size(sv(15.0)))
                                .on_press(DashboardMessage::Search)
                                .style(|theme, _| button::Style {
                                    text_color: theme.palette().primary,
                                    ..Default::default()
                                }),

                            // Input
                            text_input("Search", &self.search_query)
                                .on_input(|query| DashboardMessage::SetSearchQuery(Some(query)))
                                .size(sv_16())
                                .style(|theme: &Theme, _| text_input::Style {
                                    background: Background::Color(theme.palette().background).scale_alpha(0.0),
                                    border: Border {
                                        ..Default::default()
                                    },
                                    icon: theme.palette().primary,
                                    placeholder: theme.palette().primary,
                                    value: theme.palette().text,
                                    selection: theme.palette().primary.scale_alpha(0.3),
                                }),

                            // Clear button
                            match self.search_query.as_str() {
                                "" => container(space()),
                                _ => container(button(icon::x().size(sv(15.0)))
                                    .on_press(DashboardMessage::SetSearchQuery(None))
                                    .style(|theme, _| button::Style {
                                        text_color: theme.palette().primary,
                                        ..Default::default()
                                    })),
                            }
                        ]
                        .align_y(Alignment::Center)
                    )
                    .width(sv(600.0))
                    .padding(2)
                    .align_y(Alignment::Center)
                    .style(|theme: &Theme| container::Style {
                        border: Border {
                            color: theme.palette().primary,
                            width: 1.0,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                )
                .width(Length::Fill)
                .padding(3),

                // Shortcuts
                row![
                    // Settings
                    button(icon::settings().size(sv_20()))
                        .style(|theme, _| button::Style {
                            text_color: theme.palette().primary,
                            ..Default::default()
                        })
                        .on_press(DashboardMessage::SetScreen(AppScreen::Settings)),

                    // Switch theme
                    button(match theme {
                        AppTheme::Dark => icon::moon()
                            .size(sv_20())
                            .color(color!(0xF1FA8C)),
                        AppTheme::Light => icon::sun()
                            .size(sv_20())
                            .color(color!(0xE77B00)),
                    })
                    .style(|_, _| button::Style {
                        ..Default::default()
                    })
                    .on_press(DashboardMessage::SetTheme),
                ]
                .spacing(10),
            ]
            .align_y(Alignment::Center)
            .spacing(20)
        )
        .padding([5, 20])
        .style(|theme| container::Style {
            border: Border {
                color: theme.palette().primary,
                width: 0.5,
                ..Default::default()
            },
            ..Default::default()
        })
        .width(Length::Fill)
        .into()
    }
}
