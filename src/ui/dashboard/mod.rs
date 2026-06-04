pub mod navbar;
pub mod groups;
pub mod group;
pub mod scripts;


use iced::Element;
use iced::widget::{column, row};

use navbar::Navbar;
use groups::Groups;
use group::Group;
use scripts::Scripts;


#[derive(Debug, PartialEq)]
pub struct Dashboard {
    pub navbar: Navbar,
    pub groups: Groups,
    pub group: Group,
    pub scripts: Scripts,
}

// TODO: divide this into sub messages
#[derive(Debug, Clone, PartialEq)]
pub enum DashboardMessage {
    GotoHome,
    GotoSettings,
    ToggleDarkMode,
    SearchChanged(String),
}


impl Dashboard {
    pub fn new() -> Self {
        Self {
            navbar: Navbar::new(),
            groups: Groups::new(),
            group: Group::new(),
            scripts: Scripts::new(),
        }
    }

    pub fn view(&self) -> Element<'_, DashboardMessage> {
        column![
            self.navbar.view(),
            row![
                self.groups.view(),
                self.group.view(),
                self.scripts.view(),
            ]
        ]
        .into()
    }
}
