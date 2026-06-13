use iced::{Border, Color, Element, Length};
use iced::widget::{button, column, container, grid, text};

use super::{Dashboard, DashboardMessage};


impl Dashboard {
    pub fn current_group_view(&self) -> Element<'_, DashboardMessage> {
        fn command_card_view() -> Element<'static, DashboardMessage> {
            button("command").into()
        }

        fn script_card_view() -> Element<'static, DashboardMessage> {
            button("script").into()
        }

        container(
            column![
                // Group name
                text(&self.current_group.group_name),

                // Items
                grid!(
                    command_card_view(),
                    script_card_view(),
                )
            ]
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
