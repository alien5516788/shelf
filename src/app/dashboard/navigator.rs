use iced::{Alignment, Border, Color, Element, Length, Padding, Task, Theme};
use iced::widget::{button, center_x, container, row, text, text_input};

use crate::app::Screen;
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

    pub fn view(&self, title: String, theme: &Theme) -> Element<'_, NavigatorMessage> {
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
                center_x(
                    container(
                        text_input("Search...", self.search_query.as_str())
                            .on_input(|s| NavigatorMessage::SetSearchQuery(s))
                            .padding(10)
                    )
                    .width(600)
                    .align_y(Alignment::Center)
                    .padding(5)
                )
                .width(Length::Fill),

                // Shortcuts
                row![
                    button(
                        icon::settings()
                            .size(20.0)
                            .color(Color::from_rgb(0.5, 0.9, 0.9))
                    )
                    .style(|_, _| button::Style {
                        background: None,
                        ..Default::default()
                    })
                    .on_press(NavigatorMessage::SetScreen(Screen::Settings)),

                    button(match theme {
                        Theme::Dark => icon::moon()
                            .size(20.0)
                            .color(Color::from_rgb(0.5, 0.9, 0.9)),
                        _ => icon::sun()
                            .size(20.0)
                            .color(Color::from_rgb(0.5, 0.5, 0.5)),
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
        .style(|theme: &Theme| container::Style {
            border: Border {
                color: Color::from_rgb(0.4, 0.4, 0.45),
                width: 0.5,
                radius: 0.0.into(),
            },
            background: Some(iced::Background::Color(theme.palette().background)),
            ..Default::default()
        })
        .padding(Padding {
            top: 5.0,
            right: 20.0,
            bottom: 5.0,
            left: 20.0
        })
        .width(Length::Fill)
        .into()
    }

    pub fn update(&mut self, message: NavigatorMessage, screen: &mut Screen, theme: &mut Theme) -> Task<NavigatorMessage> {
        match message {
            NavigatorMessage::SetScreen(scrn) => {
                *screen = scrn;
                Task::none()
            },
            NavigatorMessage::ToggleTheme => match theme {
                Theme::Light => {
                    *theme = Theme::Dark;
                    Task::none()
                },
                Theme::Dracula => {
                    *theme = Theme::Light;
                    Task::none()
                },
                _ => {
                    *theme = Theme::Light;
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
