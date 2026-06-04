pub mod icon;
pub mod ui;

use iced::Font;
use ui::app::App;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title(App::title)
        .theme(App::theme)
        .font(icon::FONT)
        .default_font(Font::MONOSPACE)
        .centered()
        .run()
}
