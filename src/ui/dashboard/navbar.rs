use iced::widget::{button, container, row, text, text_input};
use iced::{Alignment, Border, Color, Element, Length, Theme};

use super::DashboardMessage;


#[derive(Debug, PartialEq)]
pub struct Navbar {
    app_name: &'static str,
    pub search_query: String,
    pub search_results: Vec<String>,
}

impl Navbar {
    pub fn new() -> Self {
        Self {
            app_name: "Shelf",
            search_query: String::new(),
            search_results: Vec::<String>::new(),
        }
    }

    pub fn view(&self) -> Element<'_, DashboardMessage> {
        container(
            row![
                // App name
                button(text(self.app_name).size(24))
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
                    .on_press(DashboardMessage::GotoHome),

                // Search bar
                container(
                    text_input("Search...", &self.search_query)
                        .on_input(|s| DashboardMessage::SearchChanged(s))
                )
                .width(Length::Fill)
                .align_y(Alignment::Center)
                .padding(5),

                // Shortcuts
                row![
                    button("⚙ Settings")
                        .on_press(DashboardMessage::GotoSettings),
                    button("☀ Dark Mode")
                        .on_press(DashboardMessage::ToggleDarkMode),
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
                width: 1.0,
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
