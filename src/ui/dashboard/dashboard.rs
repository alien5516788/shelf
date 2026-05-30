use iced::Element;
use iced::widget::column;

use crate::ui::app::AppMessage;
use super::Navbar;
use super::Groups;

// Dashboard message
#[derive(Debug, Clone, PartialEq)]
pub enum DashboardMessage {
    GotoHome,
    GotoSettings,
    ToggleDarkMode,
    SearchChanged(String),
}

// Dashboard state
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

    pub fn view(&self) -> Element<'_, AppMessage> {
        column![
            self.navbar.view(),
            self.groups.view(),
        ]
        .spacing(10)
        .into()
    }
}
