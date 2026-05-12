pub mod app;
pub mod screens;

use iced::Theme;

use app::App;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .theme(Theme::Light)
        .centered()
        .run()
}
