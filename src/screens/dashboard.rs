use iced::Element;
use iced::widget::column;

use crate::components::navbar::{Navbar, NavbarMessage};

#[derive(Debug, Clone, PartialEq)]
pub enum DashboardMessage {
    // recieved
    NavbarMessage(NavbarMessage),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dashboard {
    navbar: Navbar,
}

impl Dashboard {
    pub fn new() -> Self {
        Self {
            navbar: Navbar::new(),
        }
    }

    pub fn update(&mut self, message: &DashboardMessage) -> () {
        match message {
            DashboardMessage::NavbarMessage(m) => {
                self.navbar.update(m);
            }
        }
    }

    pub fn view(&self) -> Element<'_, DashboardMessage> {
        column![
            self.navbar.view().map(|m| DashboardMessage::NavbarMessage(m))
        ]
        .spacing(10)
        .into()
    }
}
