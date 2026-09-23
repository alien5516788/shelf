mod home;

use crate::data::settings::AppScreen;


#[derive(Debug, PartialEq)]
pub struct Home {}

#[derive(Debug, Clone)]
pub enum HomeMessage {
    SetScreen(AppScreen),
}
