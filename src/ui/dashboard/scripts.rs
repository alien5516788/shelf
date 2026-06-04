use iced::widget::{Column, Space, button, column, container, grid, row, stack, text};
use iced::{Alignment, Background, Border, Color, Element, Length};

use crate::icon;
use super::DashboardMessage;


#[derive(Debug, PartialEq)]
pub struct Scripts {

}

impl Scripts {
    pub fn new() -> Self {
     

        Self {
            
        }
    }

    pub fn view(&self) -> Element<'_, DashboardMessage> {
        container(
            grid!(
                button("scripts")
            )
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
