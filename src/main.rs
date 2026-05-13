pub mod types;
pub mod app;
pub mod screens;
pub mod components;

use iced::Font;

use types::Component;
use app::App;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Shelf")
        .theme(App::theme)
        .default_font(Font::MONOSPACE)
        .centered()
        .run()
}
