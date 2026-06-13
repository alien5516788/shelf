use iced::{Border, Color, Element, Length};
use iced::widget::{button, container, grid};

use super::{Dashboard, DashboardMessage};


impl Dashboard {
    pub fn current_script_view(&self) -> Element<'static, DashboardMessage> {
        container(
            grid!(button("scripts"))
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .padding(10)
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
