mod navigator;
mod group_navigator;
mod group;

use std::sync::Arc;
use iced::theme::Palette;
use iced::widget::{Column, Row, center, center_y, scrollable};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Task, Theme, color};
use iced::widget::{button, column, container, row, space, stack, text, text::Wrapping, text_editor, text_input, text_editor::{Content, Action}};
use sqlx::SqlitePool;

use navigator::{Navigator, NavigatorMessage};
use group_navigator::{GroupNavigator, GroupNavigatorMessage};
use group::{Group, GroupMessage};

use crate::app::dashboard::navigator::SearchInfo;
use crate::components::loading_screen::loading_screen_view;
use crate::components::modal::modal_view;
use crate::components::status_bar::{StatusMessage, status_bar_view};
use crate::data::settings::{AppScreen, AppTheme};
use crate::icon;
use crate::services::item::{create_item, delete_item, update_item};
use crate::services::group::{create_group, delete_group, update_group};
use crate::utils::font_size::{sv, sv_16, sv_20};
use crate::utils::formatting::clamp_name;


#[derive(Debug, Clone)]
pub struct Dashboard {
    pub current_group: GroupInfo,
    pub item_editor: Option<ItemEditor>,
    pub search_result: Option<Vec<SearchInfo>>,
    pub status_message: StatusMessage,

    pub navigator: Option<Navigator>,
    pub group_navigator: Option<GroupNavigator>,
    pub group: Option<Group>,

    pub pool: Arc<SqlitePool>
}

#[derive(Debug, Clone, Default)]
pub struct GroupInfo {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub item_count: usize,

    pub hovered: bool,
}

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


#[derive(Debug, Clone)]
pub enum DashboardMessage {
    LoadDashboard,
    SetStatusMessage(StatusMessage),

    SetItemEditorName(String),
    SetItemEditorContent(Action),
    SetItemEditorDescription(Action),
    SetItemEditorTag(String),
    AddToItemEditorTags,
    RemoveFromItemEditorTags(usize),
    SetItemEditorError(String),

    CloseItemEditor, // Immediately closes the item box
    SubmitItemForm, // Emitting Task -> ItemBoxDone without closing
    ItemEditorDone(Result<(), String>), // Closes or Display error

    ClearSearchResult,
    ScrollToSearchResult(i32, i32),

    NavigatorMessage(NavigatorMessage),
    GroupNavigatorMessage(GroupNavigatorMessage),
    GroupMessage(GroupMessage),
}


impl Dashboard {
    pub fn new(pool: Arc<SqlitePool>) -> (Self, Task<DashboardMessage>) {
        (
            Self {
                current_group: GroupInfo::default(),
                item_editor: None,
                search_result: None,
                status_message: StatusMessage::default(),
                navigator: None,
                group_navigator: None,
                group: None,
                pool: pool,
            },

            Task::done(DashboardMessage::LoadDashboard),
        )
    }

    pub fn view(&self, title: &String, theme: &AppTheme) -> Element<'_, DashboardMessage> {
        let Some(navigator) = &self.navigator else {
            return loading_screen_view();
        };
        let Some(group_navigator) = &self.group_navigator else {
            return loading_screen_view();
        };
        let Some(group) = &self.group else {
            return loading_screen_view();
        };

        stack![
            // Dashboard content
            column![
                // Navigator
                navigator
                    .view(title.clone(), theme)
                    .map(DashboardMessage::NavigatorMessage),

                stack![
                    // Group view
                    row![
                        // Group list
                        group_navigator
                            .view(&self.current_group)
                            .map(DashboardMessage::GroupNavigatorMessage),

                        // Group
                        group
                            .view(&self.current_group)
                            .map(DashboardMessage::GroupMessage),
                    ],

                    // Search result
                    match &self.search_result {
                        None => space().into(),
                        Some(result) => Self::serach_result_view(result),
                    }
                ],

                // Status bar
                status_bar_view(
                    &self.status_message,
                    DashboardMessage::SetStatusMessage(StatusMessage::default()),
                )
            ],

            // Popup to create/edit/delete items
            match &self.item_editor {
                None => space().into(),
                Some(editor) => Self::item_editor_view(editor),
            },
        ]
        .into()
    }

    fn item_editor_view(editor: &ItemEditor) -> Element<'_, DashboardMessage> {
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

        modal_view(
            container(
                column![
                    // Box title
                    match &editor.dialog {
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
                        color: Some(accent_color(&editor.dialog, &theme.palette())),
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
                    match &editor.dialog {
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
                                Some(name) => text_input_view(
                                    match &editor.dialog {
                                        Dialog::NewCommand | Dialog::EditCommand => "Content",
                                        _ => "Name",
                                    },
                                    name, &false, |name| DashboardMessage::SetItemEditorName(name)
                                ),
                                None => space().into(),
                            },
                            match &editor.content {
                                Some(content) => text_editor_view("Content", content, &false, |action| DashboardMessage::SetItemEditorContent(action)),
                                None => space().into(),
                            },
                            match &editor.description {
                                Some(description) => text_editor_view("Description (optional)", description, &false, |action| DashboardMessage::SetItemEditorDescription(action)),
                                None => space().into(),
                            },
                            match &editor.tags {
                                Some(tags) => tag_input_view(tags, editor.tag.as_deref().unwrap_or(""), &false, |tag| DashboardMessage::SetItemEditorTag(tag), DashboardMessage::AddToItemEditorTags, |index| DashboardMessage::RemoveFromItemEditorTags(index)),
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
                            background: Some(Background::Color(accent_color(&editor.dialog, &theme.palette()))),
                            border: Border {
                                color: accent_color(&editor.dialog, &theme.palette()),
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
            .width(match &editor.dialog {
                Dialog::NewGroup | Dialog::EditGroup |
                Dialog::DeleteGroup | Dialog::DeleteCommand | Dialog::DeleteScript => 430.0,
                _ => sv(500.0),
            })
            .height(Length::Shrink)
            .style(|theme| container::Style {
                border: Border {
                    color: accent_color(&editor.dialog, &theme.palette()),
                    width: 1.0,
                    radius: 5.into(),
                },
                background: Some(Background::from(theme.extended_palette().background.base.color)),
                ..Default::default()
            }),

            DashboardMessage::CloseItemEditor,

            0.4
        )
    }

    pub fn serach_result_view(result: &Vec<SearchInfo>) -> Element<'_, DashboardMessage> {
        fn result_item_view(item: &SearchInfo) -> Element<'_, DashboardMessage>  {
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

        modal_view(
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
                                    |column, item| column.push(result_item_view(item)),
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
        .into()
    }

    pub fn update(&mut self, message: DashboardMessage, screen: &mut AppScreen, theme: &mut AppTheme) -> Task<DashboardMessage> {
        match message {
            DashboardMessage::LoadDashboard => {
                let (navigator, navigator_task) = Navigator::new(self.pool.clone());
                self.navigator = Some(navigator);

                let (group_navigator, group_navigator_task) = GroupNavigator::new(self.pool.clone());
                self.group_navigator = Some(group_navigator);

                let (group, group_task) = Group::new(self.pool.clone());
                self.group = Some(group);

                Task::batch([
                    navigator_task.map(|m| DashboardMessage::NavigatorMessage(m)),
                    group_navigator_task.map(|m| DashboardMessage::GroupNavigatorMessage(m)),
                    group_task.map(|m| DashboardMessage::GroupMessage(m)),
                ])
            },
            DashboardMessage::SetStatusMessage(message) => {
                self.status_message = message;
                Task::none()
            },
            DashboardMessage::SetItemEditorName(name) => {
                match &mut self.item_editor {
                    Some(editor) => match &mut editor.name {
                        Some(n) => *n = name,
                        None => ()
                    },
                    None => ()
                }
                Task::none()
            },
            DashboardMessage::SetItemEditorContent(action) => {
                match &mut self.item_editor {
                    Some(editor) => match &mut editor.content {
                        Some(content) => content.perform(action),
                        None => ()
                    },
                    None => (),
                }
                Task::none()
            },
            DashboardMessage::SetItemEditorDescription(action) => {
                match &mut self.item_editor {
                    Some(editor) => match &mut editor.description {
                        Some(description) => description.perform(action),
                        None => ()
                    },
                    None => (),
                }
                Task::none()
            },
            DashboardMessage::SetItemEditorTag(t) => {
                match &mut self.item_editor {
                    Some(editor) => match &mut editor.tag {
                        Some(tag) => *tag = t,
                        None => ()
                    },
                    None => ()
                }
                Task::none()
            },
            DashboardMessage::AddToItemEditorTags => {
                match &mut self.item_editor {
                    Some(editor) => {
                        let tag = match &mut editor.tag {
                            Some(tag) => tag,
                            None => return Task::none()
                        };
                        match &mut editor.tags {
                            Some(tags) => {
                                tags.push(tag.clone());
                                tag.clear();
                            },
                            None => ()
                        }
                    },
                    None => ()
                }
                Task::none()
            },
            DashboardMessage::RemoveFromItemEditorTags(index) => {
                match &mut self.item_editor {
                    Some(editor) => match &mut editor.tags {
                        Some(tags) => {
                            tags.remove(index);
                        },
                        None => (),
                    },
                    None => (),
                }
                Task::none()
            },
            DashboardMessage::SetItemEditorError(error) => {
                match &mut self.item_editor {
                    Some(editor) => editor.error = Some(error),
                    None => (),
                }
                Task::none()
            },
            DashboardMessage::CloseItemEditor => {
                self.item_editor = None;
                Task::none()
            },
            DashboardMessage::SubmitItemForm => {
                let editor = match &self.item_editor {
                    Some(editor) => editor,
                    None => return Task::none(),
                };

                let id = editor.id.unwrap_or(0);
                let name = editor.name.clone().unwrap_or(String::new());
                let content = editor.content.clone().unwrap_or(Content::new());
                let description = editor.description.clone().unwrap_or(Content::new());
                let tags = editor.tags.clone().unwrap_or(Vec::new());

                let dialog = &editor.dialog;

                match dialog {
                    Dialog::NewGroup => {
                        let pool = self.pool.clone();
                        Task::perform(
                            async move { create_group(pool, name, description.text()).await.map(|_| ()) },
                            DashboardMessage::ItemEditorDone,
                        )
                    },
                    Dialog::EditGroup => {
                        let pool = self.pool.clone();
                        Task::perform(
                            async move { update_group(pool, id, name, description.text()).await },
                            DashboardMessage::ItemEditorDone,
                        )
                    },
                    Dialog::DeleteGroup => {
                        let pool = self.pool.clone();
                        Task::perform(
                            async move { delete_group(pool, id).await },
                            DashboardMessage::ItemEditorDone,
                        )
                    },
                    Dialog::NewCommand => {
                        let pool = self.pool.clone();
                        let group_id = self.current_group.id;
                        Task::perform(
                            async move {
                                create_item(pool, group_id, "command", None, content.text(), description.text(), tags)
                                    .await
                                    .map(|_| ())
                            },
                            DashboardMessage::ItemEditorDone,
                        )
                    },
                    Dialog::EditCommand => {
                        let pool = self.pool.clone();
                        Task::perform(
                            async move {
                                update_item(pool, id, "command", None, content.text(), description.text(), tags)
                                    .await
                                    .map(|_| ())
                            },
                            DashboardMessage::ItemEditorDone,
                        )
                    },
                    Dialog::NewScript => {
                        let pool = self.pool.clone();
                        let group_id = self.current_group.id;
                        Task::perform(
                            async move {
                                create_item(pool, group_id, "script", Some(name), content.text(), description.text(), tags)
                                    .await
                                    .map(|_| ())
                            },
                            DashboardMessage::ItemEditorDone,
                        )
                    },
                    Dialog::EditScript => {
                        let pool = self.pool.clone();
                        Task::perform(
                            async move {
                                update_item(pool, id, "script", Some(name), content.text(), description.text(), tags)
                                    .await
                                    .map(|_| ())
                            },
                            DashboardMessage::ItemEditorDone,
                        )
                    },
                    Dialog::DeleteCommand | Dialog::DeleteScript => {
                        let pool = self.pool.clone();
                        Task::perform(
                            async move { delete_item(pool, id).await },
                            DashboardMessage::ItemEditorDone,
                        )
                    },
                }
            },
            DashboardMessage::ItemEditorDone(result) => {
                let editor = match &self.item_editor {
                    Some(editor) => editor,
                    None => return Task::none(),
                };

                match result {
                    Ok(()) => {
                        match editor.dialog {
                            Dialog::NewGroup | Dialog::EditGroup | Dialog::DeleteGroup => {
                                Task::done(DashboardMessage::CloseItemEditor)
                                    .chain(Task::done(DashboardMessage::GroupNavigatorMessage(GroupNavigatorMessage::LoadGroupNavigator)))
                            },
                            Dialog::NewCommand | Dialog::EditCommand | Dialog::DeleteCommand
                            | Dialog::NewScript | Dialog::EditScript | Dialog::DeleteScript => {
                                Task::done(DashboardMessage::CloseItemEditor)
                                    .chain(Task::done(DashboardMessage::GroupMessage(GroupMessage::LoadGroup)))
                            },
                        }
                    },
                    Err(e) => Task::done(DashboardMessage::SetItemEditorError(e)),
                }
            },
            DashboardMessage::ClearSearchResult => {
                self.search_result = None;
                Task::none()
            },
            DashboardMessage::ScrollToSearchResult(id, group_id) => {
                Task::done(DashboardMessage::GroupNavigatorMessage(GroupNavigatorMessage::SetCurrentGroup(group_id)))
                    // ISSUE: Highlight must run after loading the group
                    .chain(Task::done(DashboardMessage::GroupMessage(GroupMessage::HighlightItem(id))))
                    .chain(Task::done(DashboardMessage::ClearSearchResult))
            },
            DashboardMessage::NavigatorMessage(navigator_m) => match &mut self.navigator {
                Some(navigator) => navigator.update(navigator_m, screen, theme, &mut self.search_result).map(|m| DashboardMessage::NavigatorMessage(m)),
                None => Task::none(),
            },
            DashboardMessage::GroupNavigatorMessage(group_navigator_m) => {
                if let GroupNavigatorMessage::ReloadGroup = group_navigator_m {
                    return Task::done(DashboardMessage::GroupMessage(GroupMessage::LoadGroup));
                }

                match &mut self.group_navigator {
                    Some(group_navigator) => group_navigator.update(group_navigator_m, &mut self.current_group, &mut self.item_editor).map(|m| DashboardMessage::GroupNavigatorMessage(m)),
                    None => Task::none(),
                }
            },
            DashboardMessage::GroupMessage(group_m) => match &mut self.group {
                Some(group) => group.update(group_m, &mut self.current_group, &mut self.item_editor, &mut self.status_message).map(|m| DashboardMessage::GroupMessage(m)),
                None => Task::none(),
            },
        }
    }
}
