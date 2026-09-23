use iced::widget::{button, container, row, text};
use iced::{Alignment, Border, Element, Length, Theme};

use super::{Settings, SettingsMessage};
use crate::data::settings::AppScreen;
use crate::icon;
use crate::utils::font_size::sv;


impl Settings {
    pub fn navigator_view(&self) -> Element<'static, SettingsMessage> {
        container(
            row![
                // Back button
                button(icon::chevron_left().size(sv(20.0)))
                    .padding(2.5)
                    .style(|theme: &Theme, _| {
                        button::Style {
                            text_color: theme.palette().text,
                            ..Default::default()
                        }
                    })
                    .on_press(SettingsMessage::SetScreen(AppScreen::Dashboard)),

                // Title
                text("Settings")
                    .size(sv(18.0))
                    .style(|theme: &Theme| {
                        text::Style {
                            color: Some(theme.palette().text),
                            ..Default::default()
                        }
                    }),
            ]
            .padding(10)
            .spacing(20)
            .align_y(Alignment::Center),
        )
        .padding([0, 20])
        .style(|theme: &Theme| container::Style {
            border: Border {
                color: theme.palette().primary,
                width: 0.5,
                ..Default::default()
            },
            ..Default::default()
        })
        .width(Length::Fill)
        .into()
    }
}
