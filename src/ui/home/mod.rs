pub mod home;

pub use home::Home;

#[derive(Debug, Clone, PartialEq)]
pub enum HomeMessage {
    GotoDashboard,
}
