use iced::theme::Palette;
use iced::{Border, Color, Element, Length, Theme};
use iced::widget::{button, center, container, row, space, text};

use crate::icon;
use crate::utils::font_size::{sv, sv_16};


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
pub fn status_bar_view<'a, Message: Clone + 'a>(status_message: &'a StatusMessage, on_clear: Message) -> Element<'a, Message> {
    let StatusMessage { message, status } = status_message;

    container(
        match message {
            Some(message) => {
                row![
                    text(message)
                        .size(sv(14.0))
                        .style(|theme: &Theme| text::Style {
                            color: Some(accent_color(status, &theme.palette())),
                            ..Default::default()
                        }),

                    space()
                        .width(Length::Fill),

                    button(center(icon::circle_x().size(sv_16())))
                        .on_press(on_clear)
                        .padding(0)
                        .width(20)
                        .style(|theme: &Theme, _| button::Style {
                            text_color: theme.palette().danger,
                            ..Default::default()
                        })
                ]
            },
            None => row![
                text("No problem")
                    .size(sv(14.0))
                    .style(|theme: &Theme| text::Style {
                        color: Some(accent_color(&StatusType::None, &theme.palette())),
                        ..Default::default()
                    })
            ],
        }
    )
    .height(Length::Fixed(30.0))
    .width(Length::Fill)
    .padding([5, 10])
    .style(|theme: &Theme| container::Style {
        border: Border {
            color: theme.palette().primary,
            width: 0.5,
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}

fn accent_color(status: &StatusType, palette: &Palette) -> Color {
    match status {
        StatusType::Success => palette.success,
        StatusType::Warn => palette.warning,
        StatusType::Error => palette.danger,
        StatusType::None => palette.primary,
    }
}
