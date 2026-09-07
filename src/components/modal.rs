use iced::widget::{center, container, mouse_area, opaque};
use iced::{Color, Element};

/*
 * Creates and overlay for dialog boxes notification
 * Close on when clicked on the mouse area
 */
pub fn modal_view<'a, Message: Clone + 'a>(content: impl Into<Element<'a, Message>>, on_blur: Message) -> Element<'a, Message> {
    opaque(
        mouse_area(
            center(
                // Wrapping content in an another opaque to avoid mouse click propagation
                opaque(content)
            )
            .style(|_theme| {
                container::Style {
                    background: Some(Color { a: 0.75, ..Color::BLACK }.into()),
                    ..container::Style::default()
                }
            })
        )
        .on_press(on_blur)
    )
    .into()
}
