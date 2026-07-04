use iced::{Alignment, Background, Border, Color, Element, Length};
use iced::widget::{button, container, row};

use crate::icon;
use super::{Dashboard, DashboardMessage};


impl Dashboard {
    pub fn status_bar_view(&self) -> Element<'_, DashboardMessage> {
        container(
            row![
                // Collapse side bar
                button(
                    container(
                        icon::menu()
                            .size(15.0)
                            .color(Color::from_rgb(0.3, 0.4, 0.6))
                    )
                )
                .style(|_, _| button::Style {
                    background: Some(Background::Color(Color::from_rgba(0.0, 0.7, 0.0, 0.8))),
                    ..Default::default()
                })
                .on_press(DashboardMessage::ToggleGroupNavigatorOpen),
            ]
        )
        .height(Length::Fixed(30.0))
        .width(Length::Fill)
        .padding(5.0)
        .style(|_| container::Style {
            border: Border {
                color: Color::from_rgb(0.4, 0.4, 0.4),
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
    }
}
