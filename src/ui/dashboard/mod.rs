pub mod dashboard;
pub mod navbar;
pub mod groups;

pub use dashboard::Dashboard;
pub use navbar::Navbar;
pub use groups::Groups;

#[derive(Debug, Clone, PartialEq)]
pub enum DashboardMessage {
    GotoHome,
    GotoSettings,
    ToggleDarkMode,
    SearchChanged(String),
}
