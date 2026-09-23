use iced::widget::{Column, scrollable};
use iced::{Alignment, Background, Border, Element, Length, Theme};
use iced::widget::{button, column, container, row, space, text, text::Wrapping};

use super::{Dashboard, DashboardMessage};
use crate::components::modal::modal_view;
use crate::icon;
use crate::utils::font_size::{sv, sv_16};


#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: i32,
    pub name: String,
    pub item_type: String,
    pub group_id: i32,
    pub group_name: String,
}

impl Dashboard {
    pub fn search_result_view(&self) -> Element<'_, DashboardMessage> {
        match &self.search_result {
            Some(result) => modal_view(
                container(
                    container(
                        scrollable(
                            match result.len() {
                                0 => column!(text("No result")),
                                _ => result
                                    .iter()
                                    .fold(
                                        Column::new()
                                            .spacing(10),
                                        |column, item| column.push(Self::result_item_view(item)),
                                    )
                            }
                            .width(600)
                        )
                    )
                    .style(|theme| container::Style {
                        background: Some(Background::Color(theme.palette().background)),
                        border: Border {
                            width: 1.0,
                            color: theme.palette().primary,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                )
                .height(Length::Fill), // ISSUE: Invisible container length prevents the modal click detection

                DashboardMessage::ClearSearchResult,

                0.2
            )
            .into(),

            None => space().into(),
        }
    }

    fn result_item_view(item: &SearchResult) -> Element<'_, DashboardMessage>  {
        let (id, name, item_icon, group_id, group_name) = (
            item.id,
            item.name.as_str(),
            match item.item_type.as_str() {
                "command" => icon::terminal(),
                _ => icon::code_xml(),
            },
            item.group_id,
            item.group_name.as_str(),
        );

        button(
            row![
                item_icon
                    .size(sv_16())
                    .style(|theme: &Theme| text::Style {
                        color: Some(theme.palette().primary),
                        ..Default::default()
                    }),

                text(name)
                    .size(sv(15.0))
                    .wrapping(Wrapping::None),

                space()
                    .width(Length::Fill),

                icon::group_box()
                    .size(sv(14.0))
                    .style(|theme: &Theme| text::Style {
                        color: Some(theme.palette().primary),
                        ..Default::default()
                    }),

                text(group_name)
                    .size(sv(13.0))
                    .wrapping(Wrapping::None)
                    .style(|theme: &Theme| text::Style {
                        color: Some(theme.palette().text.scale_alpha(0.5)),
                        ..Default::default()
                    }),
            ]
            .padding([6, 10])
            .spacing(10)
            .align_y(Alignment::Center)
            .width(Length::Fill)
        )
        .width(Length::Fill)
        .on_press(DashboardMessage::ScrollToSearchResult(id, group_id))
        .style(|theme: &Theme, _| {
            button::Style {
                background: Some(Background::Color(theme.palette().background)),
                text_color: theme.palette().text,
                ..Default::default()
            }
        })
        .into()
    }
}
