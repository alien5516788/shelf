use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Task, Theme};
use iced::widget::{button, center_x, container, row, text, text_input};

use crate::app::{AppTheme, Screen};
use crate::icon;

#[derive(Debug, Clone, PartialEq)]
pub struct Navigator {
    pub search_query: String,
    pub search_results: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NavigatorMessage {
    SetScreen(Screen),
    ToggleTheme,
    SetSearchQuery(String),
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
                button(text(title)
                    .size(24))
                    .style(|theme: &Theme, status: button::Status| {
                        button::Style {
                            background: None,
                            border: Border::default(),
                            text_color: match status {
                                button::Status::Hovered => Color::from_rgb(0.3, 0.6, 1.0),
                                _ => theme.palette().text,
                            },
                            ..Default::default()
                        }
                    })
                    .padding(0)
                    .on_press(NavigatorMessage::SetScreen(Screen::Home)),

                // Search bar
                // ISSUE: search bar is fucked. fix this.
                center_x(
                    container(
                        text_input("Search...", self.search_query.as_str())
                            .on_input(|s| NavigatorMessage::SetSearchQuery(s))
                            .padding(10)
                            .style(|theme: &Theme, _| text_input::Style {
                                background: Background::Color(theme.extended_palette().secondary.base.color).scale_alpha(0.0),
                                border: Border {
                                    color: theme.extended_palette().secondary.base.color,
                                    width: 1.0,
                                    ..Default::default()
                                },
                                icon: theme.extended_palette().secondary.base.color,
                                placeholder: theme.extended_palette().secondary.base.color,
                                value: theme.extended_palette().secondary.base.color,
                                selection: theme.extended_palette().secondary.base.color,
                            })
                    )
                    .width(600)
                    .align_y(Alignment::Center)
                    .padding(5)
                    .style(|theme: &Theme| container::Style {
                        border: Border {
                            color: theme.extended_palette().secondary.base.color,
                            width: 1.0,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                )
                .width(Length::Fill),

                // Shortcuts
                row![
                    button(
                        icon::settings()
                            .size(20.0)
                    )
                    .style(|theme, _| button::Style {
                        background: None,
                        text_color: theme.extended_palette().secondary.base.color,
                        ..Default::default()
                    })
                    .on_press(NavigatorMessage::SetScreen(Screen::Settings)),

                    button(match theme {
                        AppTheme::Dark => icon::moon()
                            .size(20.0)
                            .color(Color::from_rgb(0.9, 0.9, 0.5)),
                        AppTheme::Light => icon::sun()
                            .size(20.0)
                            .color(Color::from_rgb(1.0, 0.7, 0.4)),
                    })
                    .style(|_, _| button::Style {
                        background: None,
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
                color: theme.extended_palette().secondary.weak.color,
                width: 0.5,
                ..Default::default()
            },
            ..Default::default()
        })
        .width(Length::Fill)
        .into()
    }

    pub fn update(&mut self, message: NavigatorMessage, screen: &mut Screen, theme: &mut AppTheme) -> Task<NavigatorMessage> {
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
                self.search_query = query;
                Task::none()
            },
        }
    }
}
