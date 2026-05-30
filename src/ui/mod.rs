pub mod app;
pub mod home;
pub mod dashboard;

#[derive(Debug, Clone, PartialEq)]
pub enum AppMessage {
    HomeMessage(home::HomeMessage),
    DashboardMessage(dashboard::DashboardMessage),
}
