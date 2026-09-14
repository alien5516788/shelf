use iced::{Border, Element, Length, Task, Theme};
use iced::widget::{container, row};

#[derive(Debug, Clone, PartialEq)]
pub struct StatusBar {

}

#[derive(Debug, Clone, PartialEq)]
pub enum StatusBarMessage {

}

impl StatusBar {
    pub fn new() -> (Self, Task<StatusBarMessage>) {
        (
            Self {},
            Task::none(),
        )
    }
    pub fn view(&self) -> Element<'_, StatusBarMessage> {
        container(
            row![]
        )
        .height(Length::Fixed(30.0))
        .width(Length::Fill)
        .padding(5.0)
        .style(|theme: &Theme| container::Style {
            border: Border {
                color: theme.extended_palette().secondary.weak.color,
                width: 0.5,
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
    }

    pub fn update(&mut self, _message: StatusBarMessage) -> Task<StatusBarMessage> {
        Task::none()
        // match message {
        // }
    }
}
