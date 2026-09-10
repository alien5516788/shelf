mod navigator;
mod group_navigator;
mod group;
mod status_bar;

use std::sync::Arc;
use iced::border::Radius;
use iced::widget::{Row, center_y};
use iced::{Background, Border, Color, Element, Length, Task, Theme};
use iced::widget::{button, column, container, row, space, stack, text, text::Wrapping, text_editor, text_input, text_editor::{Content, Action}};
use sqlx::SqlitePool;

use navigator::{Navigator, NavigatorMessage};
use group_navigator::{GroupNavigator, GroupNavigatorMessage};
use group::{Group, GroupMessage};
use status_bar::{StatusBar, StatusBarMessage};

use crate::components::loading_screen::loading_screen_view;
use crate::components::modal::modal_view;
use crate::icon;
use crate::services::command::{create_command, delete_command, update_command};
use crate::services::group::{create_group, delete_group, update_group};
use crate::services::script::{create_script, delete_script, update_script};

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

    pub fn view(&self, title: &String) -> Element<'_, DashboardMessage> {
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
                    .view(title.clone())
                    .map(DashboardMessage::NavigatorMessage),

                // Group view
                row![
                    // Group list
                    group_navigator
                        .view()
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
        fn text_input_view<'a, F>(placeholder: &str, value: &'a str, error: bool, on_input: F) -> Element<'a, DashboardMessage>
        where F: Fn(String) -> DashboardMessage + 'a
        {
            text_input(placeholder, value)
                .on_input(on_input)
                .padding(10)
                .style(move |_, _| text_input::Style {
                    background: Background::Color(Color::from_rgba(0.15, 0.15, 0.18, 1.0)),
                    border: Border {
                        color: match error {
                            true => Color::from_rgb(0.35, 0.35, 0.4),
                            false => Color::from_rgb(0.3, 0.3, 0.35),
                        },
                        width: match error {
                            true => 1.5,
                            false => 1.0,
                        },
                        radius: 5.0.into(),
                    },
                    icon: Color::from_rgb(0.6, 0.6, 0.6),
                    placeholder: Color::from_rgb(0.5, 0.5, 0.55),
                    value: Color::from_rgb(0.95, 0.95, 0.95),
                    selection: Color::from_rgb(0.3, 0.5, 0.8),
                })
                .into()
        }

        fn text_editor_view<'a, F>(placeholder: &'a str, content: &'a Content, error: bool, on_action: F) -> Element<'a, DashboardMessage>
            where F: Fn(Action) -> DashboardMessage + 'a
        {
            text_editor(content)
                .on_action(on_action)
                .placeholder(placeholder)
                .height(Length::Fixed(125.0))
                .padding(10)
                .wrapping(Wrapping::WordOrGlyph)
                .style(move |_, _| text_editor::Style {
                    background: Background::Color(Color::from_rgba(0.15, 0.15, 0.18, 1.0)),
                    border: Border {
                        color: match error {
                            true => Color::from_rgb(0.35, 0.35, 0.4),
                            false => Color::from_rgb(0.3, 0.3, 0.35),
                        },
                        width: match error {
                            true => 1.5,
                            false => 1.0,
                        },
                        radius: 5.0.into(),
                    },
                    placeholder: Color::from_rgb(0.5, 0.5, 0.55),
                    value: Color::from_rgb(0.95, 0.95, 0.95),
                    selection: Color::from_rgb(0.3, 0.5, 0.8),
                })
                .into()
        }

        fn tag_input_view<'a>(
            tags: &'a [String], tag: &'a str,
            on_input: impl Fn(String) -> DashboardMessage + 'a, on_add: DashboardMessage, on_remove: impl Fn(usize) -> DashboardMessage + 'a,
        ) -> Element<'a, DashboardMessage> {
            column![
                // Tag list
                tags
                    .iter()
                    .enumerate()
                    .fold(
                        Row::new()
                            .spacing(5),
                        |row, (index, name)| {
                            row.push(
                                container(
                                    row![
                                        text(name.as_str()),
                                        button(text("x").size(15).color(Color::from_rgb(0.8, 0.0, 0.0)))
                                            .on_press(on_remove(index))
                                            .padding(0)
                                            .style(|_, _| button::Style {
                                                background: None,
                                                ..Default::default()
                                            }),
                                    ]
                                    .spacing(8)
                                    .align_y(iced::Alignment::Center),
                                )
                                .padding(5)
                                .style(|_| container::Style {
                                    border: Border {
                                        color: Color::from_rgb(0.4, 0.4, 0.4),
                                        width: 0.5,
                                        radius: 5.0.into(),
                                    },
                                    ..Default::default()
                                }),
                            )
                        }
                    )
                    .wrap(),

                row![
                    text_input("Tag name", tag)
                        .on_input(on_input)
                        .on_submit(on_add.clone())
                        .padding(10)
                        .style(|_, _| text_input::Style {
                            background: Background::Color(Color::from_rgba(0.15, 0.15, 0.18, 1.0)),
                            border: Border {
                                color: Color::from_rgb(0.35, 0.35, 0.4),
                                width: 1.0,
                                radius: 5.0.into(),
                            },
                            icon: Color::from_rgb(0.6, 0.6, 0.6),
                            placeholder: Color::from_rgb(0.5, 0.5, 0.55),
                            value: Color::from_rgb(0.95, 0.95, 0.95),
                            selection: Color::from_rgb(0.3, 0.5, 0.8),
                        }),

                    button(
                        center_y(icon::plus())
                    )
                    .on_press(on_add)
                    .height(40) // TODO: Guessed height
                    .style(|_, _| button::Style {
                        border: Border {
                            radius: 5.0.into(),
                            ..Default::default()
                        },
                        background: Some(Background::Color(Color::from_rgb(0.4, 0.4, 0.9))),
                        ..Default::default()
                    }),
                ]
                .spacing(5),
            ]
            .spacing(5)
            .into()
        }

        modal_view(
            container(
                column![
                    // Box title
                    match &self.item_box {
                        ItemBox::NewGroup => text("New Group").size(13),
                        ItemBox::EditGroup => text("Edit Group").size(13),
                        ItemBox::DeleteGroup => text("Delete Group").size(13),
                        ItemBox::NewCommand => text("New Command").size(13),
                        ItemBox::EditCommand => text("Edit Command").size(13),
                        ItemBox::DeleteCommand => text("Delete Command").size(13),
                        ItemBox::NewScript => text("New Script").size(13),
                        ItemBox::EditScript => text("Edit Script").size(13),
                        ItemBox::DeleteScript => text("Delete Script").size(13),
                        _ => text(""),
                    }
                    .size(18)
                    .style(|_| text::Style {
                        color: Some(Color::from_rgb(0.0, 1.0, 0.0)),
                        ..Default::default()
                    }),

                    // Error message
                    match &self.item_error {
                        Some(error) => text(error).size(15),
                        None => text("").size(0.1),
                    }
                    .style(|_| text::Style {
                        color: Some(Color::from_rgb(1.0, 0.0, 0.0)),
                        ..Default::default()
                    }),

                    // Input fields and Tag input or body text
                    match &self.item_box {
                        ItemBox::DeleteGroup | ItemBox::DeleteCommand | ItemBox::DeleteScript => column![
                            match &self.item_form.name {
                                Some(name) => container(text(format!("Are you sure you want to delete '{}' ?", name)).size(13)),
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
                                    name, false, |name| DashboardMessage::SetItemFormName(name)
                                ),
                                None => space().into(),
                            },
                            match &self.item_form.content {
                                Some(content) => text_editor_view("Content", content, false, |action| DashboardMessage::SetItemFormContent(action)),
                                None => space().into(),
                            },
                            match &self.item_form.description {
                                Some(description) => text_editor_view("Description (optional)", description, false, |action| DashboardMessage::SetItemFormDescription(action)),
                                None => space().into(),
                            },
                            match &self.item_form.tags {
                                Some(tags) => tag_input_view(tags, "", |tag| DashboardMessage::SetItemFormTag(tag), DashboardMessage::AddItemFormTag, |index| DashboardMessage::RemoveItemFormTag(index)),
                                None => space().into(),
                            },

                        ]
                        .spacing(20),
                    },

                    // Confirm and Cancel buttons
                    row![
                        space()
                            .width(Length::Fill),

                        button("Cancel")
                            .on_press(DashboardMessage::CloseItemBox),

                        space()
                            .width(20),

                        button("Confirm")
                            .on_press(DashboardMessage::SubmitItemBox),
                    ]
                    .width(Length::Fill)
                    .spacing(10)
                ]
                .padding(30)
                .spacing(20)
            )
            .width(425)
            .height(Length::Shrink)
            .style(|_| container::Style {
                border: Border {
                    color: Color::from_rgb(0.0, 1.0, 0.0),
                    width: 1.0,
                    radius: Radius::from(10),
                },
                background: Some(Background::from(Color::from_rgb(0.2, 0.2, 0.2))),
                ..Default::default()
            }),

            DashboardMessage::CloseItemBox
        )
    }

    pub fn update(&mut self, message: DashboardMessage, screen: &mut Screen, theme: &mut Theme) -> Task<DashboardMessage> {
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
                let id = self.item_form.id.unwrap_or(-1);
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
                                create_command(pool, group_id, name, description.text(), tags)
                                    .await
                                    .map(|_| ())
                            },
                            DashboardMessage::ItemBoxDone,
                        )
                    },
                    ItemBox::EditCommand => {
                        Task::perform(
                            async move {
                                update_command(pool, id, name, description.text(), tags)
                                    .await
                                    .map(|_| ())
                            },
                            DashboardMessage::ItemBoxDone,
                        )
                    },
                    ItemBox::DeleteCommand => {
                        Task::perform(
                            async move { delete_command(pool, id).await },
                            DashboardMessage::ItemBoxDone,
                        )
                    },
                    ItemBox::NewScript => {
                        let group_id = self.current_group.id;
                        Task::perform(
                            async move {
                                create_script(pool, group_id, name, content.text(), description.text(), tags)
                                    .await
                                    .map(|_| ())
                            },
                            DashboardMessage::ItemBoxDone,
                        )
                    },
                    ItemBox::EditScript => {
                        Task::perform(
                            async move {
                                update_script(pool, id, name, content.text(), description.text(), tags)
                                    .await
                                    .map(|_| ())
                            },
                            DashboardMessage::ItemBoxDone,
                        )
                    },
                    ItemBox::DeleteScript => {
                        Task::perform(
                            async move { delete_script(pool, id).await },
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
                            ItemBox::NewGroup | ItemBox::DeleteGroup => {
                                Task::done(DashboardMessage::CloseItemBox)
                                    .chain(Task::done(DashboardMessage::GroupNavigatorMessage(GroupNavigatorMessage::LoadGroupNavigator)))
                            },
                            ItemBox::NewCommand | ItemBox::EditCommand | ItemBox::DeleteCommand
                            | ItemBox::NewScript | ItemBox::EditScript | ItemBox::DeleteScript
                            | ItemBox::EditGroup => {
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
            DashboardMessage::GroupNavigatorMessage(group_navigator_m) => match &mut self.group_navigator {
                Some(group_navigator) => group_navigator.update(group_navigator_m, &mut self.current_group, &mut self.item_box, &mut self.item_form).map(|m| DashboardMessage::GroupNavigatorMessage(m)),
                None => Task::none(),
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
