use iced::theme::Palette;
use iced::{Border, Color, Element, Length, Theme};
use iced::widget::{button, container, row, space};


#[derive(Debug, Clone, Default)]
pub struct StatusMessage {
    pub message: Option<String>,
    pub status: StatusType,
}

#[derive(Debug, Clone, Default)]
pub enum StatusType {
    Success,
    Warn,
    Error,
    #[default]
    None,
}

impl StatusMessage {
    pub fn new(message: Option<String>, status: StatusType) -> Self {
        Self { message, status }
    }
}

/*
 * Shows status messges such as error, success etc
 */
// TODO: Complete the UI
pub fn status_bar_view<'a, Message: Clone + 'a>(
    message: &'a Option<String>,
    _status: &'a StatusType,
    on_clear: Message,
) -> Element<'a, Message> {
    container(
        if let Some(message) = message {
            row![
                iced::widget::text(message),
                space().width(Length::Fill),
                button("Close")
                    .on_press(on_clear)
            ]
        } else {
            row![
                iced::widget::text("Ok")
            ]
        },
    )
    .height(Length::Fixed(30.0))
    .width(Length::Fill)
    .padding(5.0)
    .style(|theme: &Theme| container::Style {
        border: Border {
            color: theme.extended_palette().secondary.weak.color,
            width: 0.5,
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}

fn _accent_color(status: StatusType, palette: &Palette) -> Color {
    match status {
        StatusType::Success => palette.success,
        StatusType::Warn => palette.warning,
        StatusType::Error => palette.danger,
        StatusType::None => palette.primary,
    }
}
