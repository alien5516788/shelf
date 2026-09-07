use iced::widget::{center, text};
use iced::Element;

pub fn loading_screen_view<'a, Message: 'a>() -> Element<'a, Message> {
    center(
        text("Loading...")
    )
    .into()
}
