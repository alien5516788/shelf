pub mod app;
pub mod screens;
pub mod components;

use app::App;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Shelf")
        .theme(App::theme)
        .centered()
        .run()
}
