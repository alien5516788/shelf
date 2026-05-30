pub mod ui;

use iced::Font;

use ui::app::App;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Shelf")
        .theme(App::theme)
        .default_font(Font::MONOSPACE)
        .centered()
        .run()
}
