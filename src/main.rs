pub mod icon;
pub mod app;

use iced::Font;

use app::App;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title(App::title)
        .theme(App::theme)
        .font(icon::FONT)
        .default_font(Font::MONOSPACE)
        .centered()
        .run()
}
