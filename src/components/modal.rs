use iced::widget::{center, container, mouse_area, opaque};
use iced::{Background, Color, Element};

/*
 * Creates and overlay for dialog boxes notification
 * Closes when clicked on the mouse area
 */
pub fn modal_view<'a, Message: Clone + 'a>(content: impl Into<Element<'a, Message>>, on_blur: Message) -> Element<'a, Message> {
    opaque(
        mouse_area(
            // TODO: Add back drop filter
            center(
                // Wrapping content in an another opaque to avoid mouse click propagation
                opaque(content)
            )
            .style(|_| {
                container::Style {
                    background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.4))),
                    ..Default::default()
                }
            })
        )
        .on_press(on_blur)
    )
    .into()
}
