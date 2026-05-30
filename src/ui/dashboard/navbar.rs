use iced::widget::{button, container, row, text, text_input};
use iced::{Alignment, Border, Color, Element, Length, Theme};

use crate::ui::app::AppMessage;
use super::DashboardMessage;


const APP_NAME: &str = "Shelf";

#[derive(Debug, Clone, PartialEq)]
pub struct Navbar {
    pub search_query: String,
    pub search_results: Vec<String>,
}

impl Navbar {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
            search_results: Vec::new(),
        }
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        container(
            row![
                // App name
                button(text(APP_NAME).size(24))
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
                    .on_press(AppMessage::DashboardMessage(DashboardMessage::GotoHome)),

                // Search bar
                container(
                    text_input("Search...", &self.search_query)
                        .on_input(|s| AppMessage::DashboardMessage(DashboardMessage::SearchChanged(s)))
                )
                .width(Length::Fill)
                .align_y(Alignment::Center)
                .padding(5),

                // Shortcuts
                row![
                    button("⚙ Settings")
                        .on_press(AppMessage::DashboardMessage(DashboardMessage::GotoSettings)),
                    button("☀ Dark Mode")
                        .on_press(AppMessage::DashboardMessage(DashboardMessage::ToggleDarkMode)),
                ]
                .spacing(10),
            ]
            .align_y(Alignment::Center)
            .spacing(20)
            .padding(10)
        )
        .padding(0)
        .style(|theme: &Theme| container::Style {
            border: Border {
                color: Color::from_rgb(0.4, 0.4, 0.45),
                width: 2.0,
                radius: 0.0.into(),
            },
            background: Some(iced::Background::Color(theme.palette().background)),
            ..Default::default()
        })
        .height(70)
        .width(Length::Fill)
        .into()
    }
}

impl Navbar {
    pub fn search(&self) -> Vec<String> {
        // TODO: Search in database
        Vec::from(["temp".to_string()])
    }
}
