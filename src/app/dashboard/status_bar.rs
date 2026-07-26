use iced::{Border, Color, Element, Length};
use iced::widget::{container, row};

#[derive(Debug, Clone, PartialEq)]
pub struct StatusBar {
    
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatusBarMessage {
    
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            
        }
    }
    pub fn view(&self) -> Element<'_, StatusBarMessage> {
        container(
            row![
                
            ]
        )
        .height(Length::Fixed(30.0))
        .width(Length::Fill)
        .padding(5.0)
        .style(|_| container::Style {
            border: Border {
                color: Color::from_rgb(0.4, 0.4, 0.4),
                width: 0.5,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
    }

    pub fn update(&mut self, message: StatusBarMessage) {
        match message {
        }
    }
}
