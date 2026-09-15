mod navigator;
mod group_navigator;
mod group;
mod status_bar;

use std::sync::Arc;
use iced::theme::Palette;
use iced::widget::{Row, center, center_y};
use iced::{Background, Border, Color, Element, Length, Padding, Task, Theme, color};
use iced::widget::{button, column, container, row, space, stack, text, text::Wrapping, text_editor, text_input, text_editor::{Content, Action}};
use sqlx::SqlitePool;

use navigator::{Navigator, NavigatorMessage};
use group_navigator::{GroupNavigator, GroupNavigatorMessage};
use group::{Group, GroupMessage};
use status_bar::{StatusBar, StatusBarMessage};

use crate::app::AppTheme;
use crate::components::loading_screen::loading_screen_view;
use crate::components::modal::modal_view;
use crate::icon;
use crate::services::item::{create_item, delete_item, update_item};
use crate::services::group::{create_group, delete_group, update_group};
use crate::utils::formatting::clamp_name;

use super::Screen;


#[derive(Debug, Clone)]
pub struct Dashboard {
    pub current_group: GroupInfo,
    pub item_box: ItemBox,
    pub item_form: ItemForm,
    pub item_error: Option<String>,

    pub navigator: Option<Navigator>,
    pub group_navigator: Option<GroupNavigator>,
    pub group: Option<Group>,
    pub status_bar: Option<StatusBar>,

    pub pool: Option<Arc<SqlitePool>>
}

#[derive(Debug, Clone, Default)]
pub struct GroupInfo {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub item_count: usize,

    pub hovered: bool,
}

#[derive(Debug, Clone)]
pub enum ItemBox {
    None,
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

#[derive(Debug, Clone, Default)]
pub struct ItemForm {
    pub id: Option<i32>,
    pub name: Option<String>, // NOTE: For commands, the content field is the name field
    pub content: Option<Content>,
    pub description: Option<Content>,
    pub tag: Option<String>,
    pub tags: Option<Vec<String>>,
}


#[derive(Debug, Clone)]
pub enum DashboardMessage {
    LoadDashboard(Arc<SqlitePool>),

    SetItemFormName(String),
    SetItemFormContent(Action),
    SetItemFormDescription(Action),
    SetItemFormTag(String),
    AddItemFormTag,
    RemoveItemFormTag(usize),

    CloseItemBox, // Immediately closes the item box
    SubmitItemBox, // Emitting Task -> ItemBoxDone without closing
    ItemBoxDone(Result<(), String>), // Closes or Display error

    NavigatorMessage(NavigatorMessage),
    GroupNavigatorMessage(GroupNavigatorMessage),
    GroupMessage(GroupMessage),
    StatusBarMessage(StatusBarMessage),
}


impl Dashboard {
    pub fn new(pool: Arc<SqlitePool>) -> (Self, Task<DashboardMessage>) {
        (
            Self {
                current_group: GroupInfo::default(),
                item_box: ItemBox::None,
                item_form: ItemForm::default(),
                item_error: None,
                navigator: None,
                group_navigator: None,
                group: None,
                status_bar: None,

                pool: None,
            },

            Task::done(DashboardMessage::LoadDashboard(pool)),
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
        let Some(status_bar) = &self.status_bar else {
            return loading_screen_view();
        };

        stack![
            // Dashboard content
            column![
                // Navigator
                navigator
                    .view(title.clone(), theme)
                    .map(DashboardMessage::NavigatorMessage),

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

                // Status bar
                status_bar
                    .view()
                    .map(DashboardMessage::StatusBarMessage),
            ],

            // Popup to create/edit/delete items
            match self.item_box {
                ItemBox::None => space().into(),
                _ => self.item_box_view(),
            },
        ]
        .into()
    }

    fn item_box_view(&self) -> Element<'_, DashboardMessage> {

        // ISSUE: Accent color depends on theme palette
        fn accent_color(item_box: &ItemBox, palette: &Palette) -> Color {
            match item_box {
                ItemBox::None | ItemBox::NewGroup | ItemBox::NewCommand | ItemBox::NewScript => {
                    palette.success
                },
                ItemBox::EditGroup | ItemBox::EditCommand | ItemBox::EditScript => {
                    palette.primary
                },
                ItemBox::DeleteGroup | ItemBox::DeleteCommand | ItemBox::DeleteScript => {
                    palette.danger
                },
            }
        }

        fn text_input_view<'a, F>(placeholder: &str, value: &'a str, error: &'a bool, on_input: F) -> Element<'a, DashboardMessage>
        where F: Fn(String) -> DashboardMessage + 'a
        {
            text_input(placeholder, value)
                .on_input(on_input)
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
                .height(Length::Fixed(125.0))
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
                            .size(13),

                        button(icon::x().size(15))
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
                    button(center_y(icon::plus()))
                    .on_press(on_add)
                    .height(40) // ISSUE: Approximate height that fits the input field
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
                    match &self.item_box {
                        ItemBox::NewGroup => text("New Group"),
                        ItemBox::EditGroup => text("Edit Group"),
                        ItemBox::DeleteGroup => text("Delete Group"),
                        ItemBox::NewCommand => text("New Command"),
                        ItemBox::EditCommand => text("Edit Command"),
                        ItemBox::DeleteCommand => text("Delete Command"),
                        ItemBox::NewScript => text("New Script"),
                        ItemBox::EditScript => text("Edit Script"),
                        ItemBox::DeleteScript => text("Delete Script"),
                        _ => text(""),
                    }
                    .size(18)
                    .style(|theme: &Theme| text::Style {
                        color: Some(accent_color(&self.item_box, &theme.palette())),
                        ..Default::default()
                    }),

                    // Error message
                    container(
                        match &self.item_error {
                            Some(error) => text(error),
                            None => text(""),
                        }
                        .size(15)
                        .style(|theme: &Theme| text::Style {
                            color: Some(theme.palette().danger),
                            ..Default::default()
                        })
                    )
                    .padding(5),


                    // Input fields, Tag input / Body text
                    match &self.item_box {
                        ItemBox::DeleteGroup | ItemBox::DeleteCommand | ItemBox::DeleteScript => column![
                            match &self.item_form.name {
                                Some(name) => container(
                                    text(format!("Are you sure you want to delete '{}' ?", clamp_name(name, 30))).size(13)
                                )
                                .padding(Padding {
                                    bottom: 15.0,
                                    ..Default::default()
                                }),
                                None => container(space()),
                            }
                        ],
                        _ => column![
                            match &self.item_form.name {
                                Some(name) => text_input_view(
                                    match self.item_box {
                                        ItemBox::NewCommand | ItemBox::EditCommand => "Content",
                                        _ => "Name",
                                    },
                                    name, &false, |name| DashboardMessage::SetItemFormName(name)
                                ),
                                None => space().into(),
                            },
                            match &self.item_form.content {
                                Some(content) => text_editor_view("Content", content, &false, |action| DashboardMessage::SetItemFormContent(action)),
                                None => space().into(),
                            },
                            match &self.item_form.description {
                                Some(description) => text_editor_view("Description (optional)", description, &false, |action| DashboardMessage::SetItemFormDescription(action)),
                                None => space().into(),
                            },
                            match &self.item_form.tags {
                                Some(tags) => tag_input_view(tags, self.item_form.tag.as_deref().unwrap_or(""), &false, |tag| DashboardMessage::SetItemFormTag(tag), DashboardMessage::AddItemFormTag, |index| DashboardMessage::RemoveItemFormTag(index)),
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
                        button(center(text("Cancel")))
                            .on_press(DashboardMessage::CloseItemBox)
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
                                .style(|_| text::Style {
                                    color: Some(color!(0x27374D)),
                                    ..Default::default()
                                })
                        ))
                        .on_press(DashboardMessage::SubmitItemBox)
                        .height(40)
                        .width(100)
                        .style(|theme: &Theme, _| button::Style {
                            background: Some(Background::Color(accent_color(&self.item_box, &theme.palette()))),
                            border: Border {
                                color: accent_color(&self.item_box, &theme.palette()),
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
            .width(match &self.item_box {
                ItemBox::NewGroup | ItemBox::EditGroup |
                ItemBox::DeleteGroup | ItemBox::DeleteCommand | ItemBox::DeleteScript => 425,
                _ => 500,
            })
            .height(Length::Shrink)
            .style(|theme| container::Style {
                border: Border {
                    color: accent_color(&self.item_box, &theme.palette()),
                    width: 1.0,
                    radius: 5.into(),
                },
                background: Some(Background::from(theme.extended_palette().background.base.color)),
                ..Default::default()
            }),

            DashboardMessage::CloseItemBox
        )
    }

    pub fn update(&mut self, message: DashboardMessage, screen: &mut Screen, theme: &mut AppTheme) -> Task<DashboardMessage> {
        match message {
            DashboardMessage::LoadDashboard(pool) => {
                self.pool = Some(pool.clone());

                let (navigator, navigator_task) = Navigator::new();
                self.navigator = Some(navigator);

                let (group_navigator, group_navigator_task) = GroupNavigator::new(pool.clone());
                self.group_navigator = Some(group_navigator);

                let (group, group_task) = Group::new(pool);
                self.group = Some(group);

                let (status_bar, status_bar_task) = StatusBar::new();
                self.status_bar = Some(status_bar);

                Task::batch([
                    navigator_task.map(|m| DashboardMessage::NavigatorMessage(m)),
                    group_navigator_task.map(|m| DashboardMessage::GroupNavigatorMessage(m)),
                    group_task.map(|m| DashboardMessage::GroupMessage(m)),
                    status_bar_task.map(|m| DashboardMessage::StatusBarMessage(m)),
                ])
            },
            DashboardMessage::SetItemFormName(nm) => {
                match &mut self.item_form.name {
                    Some(name) => *name = nm,
                    None => ()
                }
                Task::none()
            },
            DashboardMessage::SetItemFormContent(action) => {
                match &mut self.item_form.content {
                    Some(content) => content.perform(action),
                    None => ()
                }
                Task::none()
            },
            DashboardMessage::SetItemFormDescription(action) => {
                match &mut self.item_form.description {
                    Some(description) => description.perform(action),
                    None => ()
                }
                Task::none()
            },
            DashboardMessage::SetItemFormTag(t) => {
                match &mut self.item_form.tag {
                    Some(tag) => *tag = t,
                    None => ()
                }
                Task::none()
            },
            DashboardMessage::AddItemFormTag => {
                match &mut self.item_form.tags {
                    Some(tags) => {
                        match &mut self.item_form.tag {
                            Some(tag) => {
                                tags.push(tag.clone());
                                tag.clear();
                                Task::none()
                            },
                            None => Task::none(),
                        }
                    },
                    None => Task::none(),
                }
            },
            DashboardMessage::RemoveItemFormTag(idx) => {
                match &mut self.item_form.tags {
                    Some(tags) => {
                        tags.remove(idx);
                        Task::none()
                    },
                    _ => Task::none(),
                }
            },
            DashboardMessage::CloseItemBox => {
                self.item_box = ItemBox::None;
                self.item_form = ItemForm::default();
                self.item_error = None;
                Task::none()
            },
            DashboardMessage::SubmitItemBox => {
                // Check db pool
                let Some(pool) = self.pool.clone() else {
                    self.item_error = Some("Failed to save content".to_string());
                    eprintln!("Shelf: Failed to save content: no database connection");
                    return Task::none();
                };

                // Extract fields
                let id = self.item_form.id.unwrap_or(0);
                let name = self.item_form.name.clone().unwrap_or(String::new());
                let content = self.item_form.content.clone().unwrap_or(Content::new());
                let description = self.item_form.description.clone().unwrap_or(Content::new());
                let tags = self.item_form.tags.clone().unwrap_or(Vec::new());

                match &self.item_box {
                    ItemBox::NewGroup => {
                        Task::perform(
                            async move { create_group(pool, name, description.text()).await.map(|_| ()) },
                            DashboardMessage::ItemBoxDone,
                        )
                    },
                    ItemBox::EditGroup => {
                        Task::perform(
                            async move { update_group(pool, id, name, description.text()).await },
                            DashboardMessage::ItemBoxDone,
                        )
                    },
                    ItemBox::DeleteGroup => {
                        Task::perform(
                            async move { delete_group(pool, id).await },
                            DashboardMessage::ItemBoxDone,
                        )
                    },
                    ItemBox::NewCommand => {
                        let group_id = self.current_group.id;
                        Task::perform(
                            async move {
                                create_item(pool, group_id, "command", name, content.text(), description.text(), tags)
                                    .await
                                    .map(|_| ())
                            },
                            DashboardMessage::ItemBoxDone,
                        )
                    },
                    ItemBox::EditCommand => {
                        Task::perform(
                            async move {
                                update_item(pool, id, "command", name, content.text(), description.text(), tags)
                                    .await
                                    .map(|_| ())
                            },
                            DashboardMessage::ItemBoxDone,
                        )
                    },
                    ItemBox::NewScript => {
                        let group_id = self.current_group.id;
                        Task::perform(
                            async move {
                                create_item(pool, group_id, "script", name, content.text(), description.text(), tags)
                                    .await
                                    .map(|_| ())
                            },
                            DashboardMessage::ItemBoxDone,
                        )
                    },
                    ItemBox::EditScript => {
                        Task::perform(
                            async move {
                                update_item(pool, id, "script", name, content.text(), description.text(), tags)
                                    .await
                                    .map(|_| ())
                            },
                            DashboardMessage::ItemBoxDone,
                        )
                    },
                    ItemBox::DeleteCommand | ItemBox::DeleteScript => {
                        Task::perform(
                            async move { delete_item(pool, id).await },
                            DashboardMessage::ItemBoxDone,
                        )
                    },
                    _ => Task::none(),
                }
            },
            DashboardMessage::ItemBoxDone(result) => {
                match result {
                    Ok(()) => {
                        match self.item_box {
                            ItemBox::NewGroup | ItemBox::EditGroup | ItemBox::DeleteGroup => {
                                Task::done(DashboardMessage::CloseItemBox)
                                    .chain(Task::done(DashboardMessage::GroupNavigatorMessage(GroupNavigatorMessage::LoadGroupNavigator)))
                            },
                            ItemBox::NewCommand | ItemBox::EditCommand | ItemBox::DeleteCommand
                            | ItemBox::NewScript | ItemBox::EditScript | ItemBox::DeleteScript => {
                                Task::done(DashboardMessage::CloseItemBox)
                                    .chain(Task::done(DashboardMessage::GroupMessage(GroupMessage::LoadGroup)))
                            },
                            _ => Task::none(),
                        }
                    },
                    Err(e) => {
                        self.item_error = Some(e);
                        Task::none()
                    }
                }
            },
            DashboardMessage::NavigatorMessage(navigator_m) => match &mut self.navigator {
                Some(navigator) => navigator.update(navigator_m, screen, theme).map(|m| DashboardMessage::NavigatorMessage(m)),
                None => Task::none(),
            },
            DashboardMessage::GroupNavigatorMessage(group_navigator_m) => {
                if let GroupNavigatorMessage::ReloadGroup = group_navigator_m {
                    return Task::done(DashboardMessage::GroupMessage(GroupMessage::LoadGroup));
                }

                match &mut self.group_navigator {
                    Some(group_navigator) => group_navigator.update(group_navigator_m, &mut self.current_group, &mut self.item_box, &mut self.item_form).map(|m| DashboardMessage::GroupNavigatorMessage(m)),
                    None => Task::none(),
                }
            },
            DashboardMessage::GroupMessage(group_m) => match &mut self.group {
                Some(group) => group.update(group_m, &mut self.current_group, &mut self.item_box, &mut self.item_form).map(|m| DashboardMessage::GroupMessage(m)),
                None => Task::none(),
            },
            DashboardMessage::StatusBarMessage(status_bar_m) => match &mut self.status_bar {
                Some(status_bar) => status_bar.update(status_bar_m).map(|m| DashboardMessage::StatusBarMessage(m)),
                None => Task::none(),
            },
        }
    }
}
