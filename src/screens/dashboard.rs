use iced::Element;
use iced::widget::column;

use crate::types::Component;
use crate::components::navbar::{Navbar, NavbarMessage};

#[derive(Debug, Clone, PartialEq)]
pub enum DashboardMessage {
    NavbarMessage(NavbarMessage),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dashboard {
    navbar: Navbar,
}

impl Component for Dashboard {
    type Message = DashboardMessage;

    fn new() -> Self {
        Self {
            navbar: Navbar::new(),
        }
    }

    fn update(&mut self, message: DashboardMessage) -> () {
        match message {
            DashboardMessage::NavbarMessage(m) => self.navbar.update(m)
        }
    }

    fn view(&self) -> Element<'_, DashboardMessage> {
        column![
            self.navbar.view().map(|m| DashboardMessage::NavbarMessage(m))
        ]
        .spacing(10)
        .into()
    }
}
