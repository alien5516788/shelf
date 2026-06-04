use iced::Element;
use iced::widget::{column, text, button};


#[derive(Debug, PartialEq)]
pub struct Home {}

#[derive(Debug, Clone, PartialEq)]
pub enum HomeMessage {
    GotoDashboard,
}

impl Home {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<'_, HomeMessage> {
        column![
            text("Home Page").size(30),

            button("Go to Dashboard")
                .on_press(HomeMessage::GotoDashboard),
        ]
        .spacing(10)
        .into()
    }
}
