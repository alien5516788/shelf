use iced::Element;
use iced::widget::column;

use super::DashboardMessage;
use super::Navbar;
use super::Groups;


#[derive(Debug, PartialEq)]
pub struct Dashboard {
    pub navbar: Navbar,
    pub groups: Groups,
}

impl Dashboard {
    pub fn new() -> Self {
        Self {
            navbar: Navbar::new(),
            groups: Groups::new(),
        }
    }

    pub fn view(&self) -> Element<'_, DashboardMessage> {
        column![
            self.navbar.view(),
            self.groups.view(),
        ]
        .spacing(10)
        .into()
    }
}
