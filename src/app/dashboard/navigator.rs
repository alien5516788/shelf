use iced::{Alignment, Border, Color, Element, Length, Task, Theme};
use iced::widget::{button, container, row, text, text_input};

use crate::app::Screen;

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

    pub fn view(&self, title: String) -> Element<'_, NavigatorMessage> {
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
                container(
                    text_input("Search...", self.search_query.as_str())
                        .on_input(|s| NavigatorMessage::SetSearchQuery(s))
                )
                .width(Length::Fill)
                .align_y(Alignment::Center)
                .padding(5),

                // Shortcuts
                row![
                    button("⚙ Settings")
                        .on_press(NavigatorMessage::SetScreen(Screen::Settings)),
                    button("☀ Dark Mode")
                        .on_press(NavigatorMessage::ToggleTheme),
                ]
                .spacing(10),
            ]
            .align_y(Alignment::Center)
            .spacing(20)
            .padding(10)
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
        .height(70)
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
                    *theme = Theme::Dracula;
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
