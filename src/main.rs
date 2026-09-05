pub mod app;
pub mod data;
pub mod icon;

use iced::{application, Font};
use dotenv::dotenv;

use app::App;

fn main() -> iced::Result {
    dotenv().ok();

    application(App::new, App::update, App::view)
        .title(App::title)
        .theme(App::theme)
        .font(icon::FONT)
        .default_font(Font::MONOSPACE)
        .centered()
        .run()
}
