use iced::theme::Palette;
use iced::widget::{Row, center, center_y};
use iced::{Background, Border, Color, Element, Length, Padding, Theme, color};
use iced::widget::{button, column, container, row, space, text, text::Wrapping, text_editor, text_input, text_editor::{Content, Action}};

use super::{Dashboard, DashboardMessage};
use crate::components::modal::modal_view;
use crate::icon;
use crate::utils::font_size::{sv, sv_16, sv_20};
use crate::utils::formatting::clamp_name;

#[derive(Debug, Clone, Default)]
pub struct ItemEditor {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub content: Option<Content>,
    pub description: Option<Content>,
    pub tag: Option<String>,
    pub tags: Option<Vec<String>>,
    pub error: Option<String>,

    pub dialog: Dialog,
}

#[derive(Debug, Clone, Default)]
pub enum Dialog {
    #[default]
    NewGroup,
    EditGroup,
    DeleteGroup,
    NewCommand,
    EditCommand,
    DeleteCommand,
    NewScript,
    EditScript,
    DeleteScript,
}

impl Dashboard {
    pub fn item_editor_view(&self) -> Element<'_, DashboardMessage> {
        match &self.item_editor {
            Some(editor) => modal_view(
                container(
                    column![
                        // Box title
                        match editor.dialog {
                            Dialog::NewGroup => text("New Group"),
                            Dialog::EditGroup => text("Edit Group"),
                            Dialog::DeleteGroup => text("Delete Group"),
                            Dialog::NewCommand => text("New Command"),
                            Dialog::EditCommand => text("Edit Command"),
                            Dialog::DeleteCommand => text("Delete Command"),
                            Dialog::NewScript => text("New Script"),
                            Dialog::EditScript => text("Edit Script"),
                            Dialog::DeleteScript => text("Delete Script"),
                        }
                        .size(sv(18.0))
                        .style(|theme: &Theme| text::Style {
                            color: Some(Self::accent_color(&editor.dialog, &theme.palette())),
                            ..Default::default()
                        }),

                        // Error message
                        container(
                            match &editor.error {
                                Some(error) => text(error),
                                None => text(""),
                            }
                            .size(sv(15.0))
                            .style(|theme: &Theme| text::Style {
                                color: Some(theme.palette().danger),
                                ..Default::default()
                            })
                        )
                        .padding(5),


                        // Input fields, Tag input / Body text
                        match editor.dialog {
                            Dialog::DeleteGroup | Dialog::DeleteCommand | Dialog::DeleteScript => column![
                                match &editor.name {
                                    Some(name) => container(
                                        text(format!("Are you sure you want to delete '{}' ?", clamp_name(name, 30))).size(sv(13.0))
                                    )
                                    .padding(Padding {
                                        bottom: 15.0,
                                        ..Default::default()
                                    }),
                                    None => container(space()),
                                }
                            ],
                            _ => column![
                                match &editor.name {
                                    Some(name) => Self::text_input_view(
                                        match &editor.dialog {
                                            Dialog::NewCommand | Dialog::EditCommand => "Content",
                                            _ => "Name",
                                        },
                                        name, &false, |name| DashboardMessage::SetItemEditorName(name)
                                    ),
                                    None => space().into(),
                                },
                                match &editor.content {
                                    Some(content) => Self::text_editor_view("Content", content, &false, |action| DashboardMessage::SetItemEditorContent(action)),
                                    None => space().into(),
                                },
                                match &editor.description {
                                    Some(description) => Self::text_editor_view("Description (optional)", description, &false, |action| DashboardMessage::SetItemEditorDescription(action)),
                                    None => space().into(),
                                },
                                match &editor.tags {
                                    Some(tags) => Self::tag_input_view(tags, &editor.tag.as_deref().unwrap_or(""), &false, |tag| DashboardMessage::SetItemEditorTag(tag), DashboardMessage::AddToItemEditorTags, |index| DashboardMessage::RemoveFromItemEditorTags(index)),
                                    None => space().into(),
                                },

                            ]
                            .spacing(10),
                        },

                        // Confirm, Cancel
                        row![
                            space()
                                .width(Length::Fill),

                            // Cancel
                            button(center(
                                text("Cancel").size(sv_16())
                            ))
                            .on_press(DashboardMessage::CloseItemEditor)
                            .height(40)
                            .width(100)
                            .style(|theme: &Theme, _| button::Style {
                                border: Border {
                                    color: theme.palette().primary,
                                    width: 1.0,
                                    radius: 5.into(),
                                },
                                text_color: theme.palette().text,
                                ..Default::default()
                            }),

                            space()
                                .width(15),

                            // Confirm
                            button(center(
                                text("Confirm")
                                    .size(sv_16())
                                    .style(|_| text::Style {
                                        color: Some(color!(0x27374D)),
                                        ..Default::default()
                                    })
                            ))
                            .on_press(DashboardMessage::SubmitItemForm)
                            .height(40)
                            .width(100)
                            .style(|theme: &Theme, _| button::Style {
                                background: Some(Background::Color(Self::accent_color(&editor.dialog, &theme.palette()))),
                                border: Border {
                                    color: Self::accent_color(&editor.dialog, &theme.palette()),
                                    width: 1.0,
                                    radius: 5.into(),
                                },
                                ..Default::default()
                            }),
                        ]
                        .width(Length::Fill)
                        .spacing(10)
                    ]
                    .padding(30)
                    .spacing(10)
                )
                .width(match editor.dialog {
                    Dialog::NewGroup | Dialog::EditGroup |
                    Dialog::DeleteGroup | Dialog::DeleteCommand | Dialog::DeleteScript => 430.0,
                    _ => sv(500.0),
                })
                .height(Length::Shrink)
                .style(|theme| container::Style {
                    border: Border {
                        color: Self::accent_color(&editor.dialog, &theme.palette()),
                        width: 1.0,
                        radius: 5.into(),
                    },
                    background: Some(Background::from(theme.extended_palette().background.base.color)),
                    ..Default::default()
                }),

                DashboardMessage::CloseItemEditor,

                0.4
            )
            .into(),

            None => space().into()
        }
    }

    fn accent_color(item_box: &Dialog, palette: &Palette) -> Color {
        match item_box {
            Dialog::NewGroup | Dialog::NewCommand | Dialog::NewScript => {
                palette.success
            },
            Dialog::EditGroup | Dialog::EditCommand | Dialog::EditScript => {
                palette.primary
            },
            Dialog::DeleteGroup | Dialog::DeleteCommand | Dialog::DeleteScript => {
                palette.danger
            },
        }
    }

    fn text_input_view<'a, F>(placeholder: &str, value: &'a str, error: &'a bool, on_input: F) -> Element<'a, DashboardMessage>
    where F: Fn(String) -> DashboardMessage + 'a
    {
        text_input(placeholder, value)
            .on_input(on_input)
            .size(sv_16())
            .padding(10)
            .style(move |theme: &Theme, _| text_input::Style {
                background: Background::Color(theme.palette().primary.scale_alpha(0.1)),
                border: Border {
                    color: match error {
                        true => theme.palette().danger,
                        false => theme.palette().primary,
                    },
                    width: match error {
                        true => 1.5,
                        false => 1.0,
                    },
                    radius: 5.0.into(),
                },
                icon: theme.palette().primary,
                placeholder: theme.palette().primary,
                value: theme.palette().text,
                selection: theme.palette().primary.scale_alpha(0.3),
            })
            .into()
    }

    fn text_editor_view<'a, F>(placeholder: &'a str, content: &'a Content, error: &'a bool, on_action: F) -> Element<'a, DashboardMessage>
        where F: Fn(Action) -> DashboardMessage + 'a
    {
        text_editor(content)
            .on_action(on_action)
            .placeholder(placeholder)
            .size(sv_16())
            .height(125.0)
            .padding(10)
            .wrapping(Wrapping::WordOrGlyph)
            .style(move |theme: &Theme, _| text_editor::Style {
                background: Background::Color(theme.palette().primary.scale_alpha(0.1)),
                border: Border {
                    color: match error {
                        true => theme.palette().danger,
                        false => theme.palette().primary,
                    },
                    width: match error {
                        true => 1.5,
                        false => 1.0,
                    },
                    radius: 5.0.into(),
                },
                placeholder: theme.palette().primary,
                value: theme.palette().text,
                selection: theme.palette().primary.scale_alpha(0.3),
            })
            .into()
    }

    fn tag_input_view<'a>(
        tags: &'a [String],
        tag: &'a str,
        error: &'a bool,
        on_input: impl Fn(String) -> DashboardMessage + 'a,
        on_add: DashboardMessage,
        on_remove: impl Fn(usize) -> DashboardMessage + 'a,
    ) -> Element<'a, DashboardMessage> {
        fn tag_view(name: &str, on_remove: impl Fn() -> DashboardMessage) -> Element<'_, DashboardMessage> {
            container(
                row![
                    text(name)
                        .color(color!(0xF8F8F2))
                        .size(sv(13.0)),

                    button(icon::x().size(sv(15.0)))
                        .on_press(on_remove())
                        .padding(0)
                        .style(|theme, _| button::Style {
                            text_color: theme.palette().danger,
                            ..Default::default()
                        }),
                ]
                .spacing(8)
                .align_y(iced::Alignment::Center),
            )
            .padding([2, 5])
            .style(|theme| container::Style {
                background: Some(Background::Color(theme.palette().primary).scale_alpha(0.6)),
                border: Border {
                    radius: 5.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
        }

        column![
            // Tag list
            tags
                .iter()
                .enumerate()
                .fold(
                    Row::new()
                        .spacing(5),
                    |row, (index, name)| row.push(
                        tag_view(name, || on_remove(index))
                    )
                )
                .wrap(),

            row![
                // Input
                text_input("Tag name", tag)
                    .on_input(on_input)
                    .on_submit(on_add.clone())
                    .size(sv_16())
                    .padding(10)
                    .style(move |theme: &Theme, _| text_input::Style {
                        background: Background::Color(theme.palette().primary.scale_alpha(0.1)),
                        border: Border {
                            color: match error {
                                true => theme.palette().danger,
                                false => theme.palette().primary,
                            },
                            width: match error {
                                true => 1.5,
                                false => 1.0,
                            },
                            radius: 5.0.into(),
                        },
                        icon: theme.palette().primary,
                        placeholder: theme.palette().primary,
                        value: theme.palette().text,
                        selection: theme.palette().primary.scale_alpha(0.3),
                    }),

                // Add
                button(center_y(icon::plus().size(sv_20())))
                .on_press(on_add)
                .height(sv(40.0)) // ISSUE: Approximate height that fits the input field
                .style(|theme, _| button::Style {
                    text_color: theme.palette().text,
                    border: Border {
                        color: theme.palette().primary,
                        width: 1.0,
                        radius: 5.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            ]
            .spacing(5),
        ]
        .padding(Padding {
            top: 10.0,
            bottom: 10.0,
            ..Default::default()
        })
        .spacing(5)
        .into()
    }
}
