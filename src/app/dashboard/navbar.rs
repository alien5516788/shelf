use iced::{Alignment, Border, Color, Element, Length, Theme};
use iced::widget::{button, container, row, text, text_input};

use super::{Dashboard, DashboardMessage};


impl Dashboard {
    pub fn navbar_view(&self) -> Element<'_, DashboardMessage> {
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
                    text_input("Search...", self.search_query.as_str())
                        .on_input(|s| DashboardMessage::SearchItem(s))
                )
                .width(Length::Fill)
                .align_y(Alignment::Center)
                .padding(5),

                // Shortcuts
                row![
                    button("⚙ Settings")
                        .on_press(DashboardMessage::GotoSettings),
                    button("☀ Dark Mode")
                        .on_press(DashboardMessage::ToggleTheme),
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
