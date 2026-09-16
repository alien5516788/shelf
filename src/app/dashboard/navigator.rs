use iced::{Alignment, Background, Border, Element, Length, Padding, Task, Theme, color};
use iced::widget::{button, center_x, container, row, space, text, text_input};

use crate::data::settings::{AppScreen, AppTheme};
use crate::icon;
use crate::utils::font_size::{sv, sv_16, sv_20};

#[derive(Debug, Clone, PartialEq)]
pub struct Navigator {
    pub search_query: String,
    pub search_results: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NavigatorMessage {
    SetScreen(AppScreen),
    ToggleTheme,
    SetSearchQuery(Option<String>),
}

impl Navigator {
    pub fn new() -> (Self, Task<NavigatorMessage>) {
        (
            Self {
                search_query: String::new(),
                search_results: Vec::<String>::new(),
            },

            Task::none()
        )
    }

    pub fn view(&self, title: String, theme: &AppTheme) -> Element<'_, NavigatorMessage> {
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
                .padding(0)
                .on_press(NavigatorMessage::SetScreen(AppScreen::Home)),

                // Search bar
                center_x(
                    container(
                        row![
                            // Search button
                            button(icon::search().size(sv(15.0)))
                                .style(|theme, _| button::Style {
                                    text_color: theme.palette().primary,
                                    ..Default::default()
                                }),

                            // Input
                            text_input("Search", self.search_query.as_str())
                                .on_input(|query| NavigatorMessage::SetSearchQuery(Some(query)))
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
                                    .on_press(NavigatorMessage::SetSearchQuery(None))
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
                        .on_press(NavigatorMessage::SetScreen(AppScreen::Settings)),

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
                    .on_press(NavigatorMessage::ToggleTheme),
                ]
                .spacing(10),
            ]
            .align_y(Alignment::Center)
            .spacing(20)
        )
        .padding(Padding {
            top: 5.0,
            right: 20.0,
            bottom: 5.0,
            left: 20.0
        })
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

    pub fn update(&mut self, message: NavigatorMessage, screen: &mut AppScreen, theme: &mut AppTheme) -> Task<NavigatorMessage> {
        match message {
            NavigatorMessage::SetScreen(scrn) => {
                *screen = scrn;
                Task::none()
            },
            NavigatorMessage::ToggleTheme => match theme {
                AppTheme::Light => {
                    *theme = AppTheme::Dark;
                    Task::none()
                },
                AppTheme::Dark => {
                    *theme = AppTheme::Light;
                    Task::none()
                },
            },
            NavigatorMessage::SetSearchQuery(query) => {
                match query {
                    Some(q) => self.search_query = q,
                    None => self.search_query.clear(),
                }
                Task::none()
            },
        }
    }
}
