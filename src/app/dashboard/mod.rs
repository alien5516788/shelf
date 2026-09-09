mod navigator;
mod group_navigator;
mod group;
mod status_bar;

use std::sync::Arc;
use iced::border::Radius;
use iced::widget::{Row, center_y};
use iced::widget::text::Wrapping;
use iced::{Background, Border, Color, Element, Length, Task, Theme};
use iced::widget::{button, column, container, row, space, stack, text, text_editor, text_input, text_editor::{Content, Action}};
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
use crate::utils::formatting::clamp_name;

use super::Screen;


#[derive(Debug, Clone)]
pub struct Dashboard {
    pub current_group: GroupInfo,
    pub item_box: ItemBox,
    pub item_error: Option<String>,

    pub navigator: Option<Navigator>,
    pub group_navigator: Option<GroupNavigator>,
    pub group: Option<Group>,
    pub status_bar: Option<StatusBar>,

    pub pool: Option<Arc<SqlitePool>>
}

#[derive(Debug, Clone)]
pub struct GroupInfo {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub item_count: usize,
}

#[derive(Debug, Clone)]
pub enum ItemBox {
    None,
    NewGroup(GroupForm),
    EditGroup(GroupForm),
    DeleteGroup(GroupForm),
    NewCommand(CommandForm),
    EditCommand(CommandForm),
    DeleteCommand(CommandForm),
    NewScript(ScriptForm),
    EditScript(ScriptForm),
    DeleteScript(ScriptForm),
}

#[derive(Debug, Clone)]
pub struct GroupForm {
    pub id: i32,
    pub name: String,
    pub description: Content,
}

#[derive(Debug, Clone)]
pub struct CommandForm {
    pub id: i32,
    pub content: String,
    pub description: Content,
    pub tag: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ScriptForm {
    pub id: i32,
    pub name: String,
    pub content: Content,
    pub description: Content,
    pub tag: String,
    pub tags: Vec<String>,
}


#[derive(Debug, Clone)]
pub enum DashboardMessage {
    LoadDashboard(Arc<SqlitePool>),

    SetGroupFormName(String),
    SetGroupFormDescription(Action),
    SetCommandFormContent(String),
    SetCommandFormDescription(Action),
    SetCommandFormTag(String),
    AddCommandFormTag,
    RemoveCommandFormTag(usize),
    SetScriptFormName(String),
    SetScriptFormContent(Action),
    SetScriptFormDescription(Action),
    SetScriptFormTag(String),
    AddScriptFormTag,
    RemoveScriptFormTag(usize),

    CloseItemBox,
    SubmitItemBox,
    ItemBoxDone(Result<(), String>),
    // GroupsReloaded(Result<Vec<GroupRow>, String>),

    NavigatorMessage(NavigatorMessage),
    GroupNavigatorMessage(GroupNavigatorMessage),
    GroupMessage(GroupMessage),
    StatusBarMessage(StatusBarMessage),
}


impl Dashboard {
    pub fn new(pool: Arc<SqlitePool>) -> (Self, Task<DashboardMessage>) {
        (
            Self {
                current_group: GroupInfo {
                    id: 0,
                    name: "".to_string(),
                    description: "".to_string(),
                    item_count: 0,
                },
                item_box: ItemBox::None,
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
                        ItemBox::NewGroup(_) => text("New Group").size(13),
                        ItemBox::EditGroup(_) => text("Edit Group").size(13),
                        ItemBox::DeleteGroup(_) => text("Delete Group").size(13),
                        ItemBox::NewCommand(_) => text("New Command").size(13),
                        ItemBox::EditCommand(_) => text("Edit Command").size(13),
                        ItemBox::DeleteCommand(_) => text("Delete Command").size(13),
                        ItemBox::NewScript(_) => text("New Script").size(13),
                        ItemBox::EditScript(_) => text("Edit Script").size(13),
                        ItemBox::DeleteScript(_) => text("Delete Script").size(13),
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

                    // Input fields and Tag inputs
                    match &self.item_box {
                        ItemBox::NewGroup(form) | ItemBox::EditGroup(form) => column![
                            text_input_view("Name", &form.name, false, |name| DashboardMessage::SetGroupFormName(name)),
                            text_editor_view("Description (optional)", &form.description, false, |action| DashboardMessage::SetGroupFormDescription(action)),
                        ],
                        ItemBox::DeleteGroup(form) => column![
                            text(format!("Are you sure you want to delete group '{}' ?", form.name)).size(13),
                        ],
                        ItemBox::NewCommand(form) | ItemBox::EditCommand(form) => column![
                            text_input_view("Content", &form.content, false, |content| DashboardMessage::SetCommandFormContent(content)),
                            text_editor_view("Description (optional)", &form.description, false, |action| DashboardMessage::SetCommandFormDescription(action)),
                            tag_input_view(
                                &form.tags,
                                &form.tag,
                                DashboardMessage::SetCommandFormTag,
                                DashboardMessage::AddCommandFormTag,
                                DashboardMessage::RemoveCommandFormTag,
                            )
                        ],
                        ItemBox::DeleteCommand(form) => column![
                            text(format!("Are you sure you want to delete command '{}' ?", clamp_name(&form.content, 20))).size(13),
                        ],
                        ItemBox::NewScript(form) | ItemBox::EditScript(form) => column![
                            text_input_view("Name", &form.name, false, |name| DashboardMessage::SetScriptFormName(name)),
                            text_editor_view("Content", &form.content, false, |action| DashboardMessage::SetScriptFormContent(action)),
                            text_editor_view("Description (optional)", &form.description, false, |action| DashboardMessage::SetScriptFormDescription(action)),
                            tag_input_view(
                                &form.tags,
                                &form.tag,
                                DashboardMessage::SetScriptFormTag,
                                DashboardMessage::AddScriptFormTag,
                                DashboardMessage::RemoveScriptFormTag,
                            )
                        ],
                        ItemBox::DeleteScript(form) => column![
                            text(format!("Are you sure you want to delete script '{}' ?", form.name)).size(13),
                        ],
                        _ => column!(),
                    }
                    .spacing(20),

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
            DashboardMessage::SetGroupFormName(name) => {
                match &mut self.item_box {
                    ItemBox::NewGroup(form) | ItemBox::EditGroup(form)=> {
                        form.name = name;
                        Task::none()
                    },
                    _ => Task::none(),
                }
            },
            DashboardMessage::SetGroupFormDescription(action) => {
                match &mut self.item_box {
                    ItemBox::NewGroup(form) | ItemBox::EditGroup(form)=> {
                        form.description.perform(action);
                        Task::none()
                    },
                    _ => Task::none(),
                }
            },
            DashboardMessage::SetCommandFormContent(content) => {
                match &mut self.item_box {
                    ItemBox::NewCommand(form) | ItemBox::EditCommand(form)=> {
                        form.content = content;
                        Task::none()
                    },
                    _ => Task::none(),
                }
            },
            DashboardMessage::SetCommandFormDescription(action) => {
                match &mut self.item_box {
                    ItemBox::NewCommand(form) | ItemBox::EditCommand(form)=> {
                        form.description.perform(action);
                        Task::none()
                    },
                    _ => Task::none(),
                }
            },
            DashboardMessage::SetCommandFormTag(tag) => {
                match &mut self.item_box {
                    ItemBox::NewCommand(form) | ItemBox::EditCommand(form) => {
                        form.tag = tag;
                        Task::none()
                    },
                    _ => Task::none(),
                }
            },
            DashboardMessage::AddCommandFormTag => {
                match &mut self.item_box {
                    ItemBox::NewCommand(form) | ItemBox::EditCommand(form) => {
                        form.tags.push(form.tag.clone());
                        form.tag.clear();
                        Task::none()
                    },
                    _ => Task::none(),
                }
            },
            DashboardMessage::RemoveCommandFormTag(idx) => {
                match &mut self.item_box {
                    ItemBox::NewCommand(form) | ItemBox::EditCommand(form) => {
                        form.tags.remove(idx);
                        Task::none()
                    },
                    _ => Task::none(),
                }
            },
            DashboardMessage::SetScriptFormName(name) => {
                match &mut self.item_box {
                    ItemBox::NewScript(form) | ItemBox::EditScript(form)=> {
                        form.name = name;
                        Task::none()
                    },
                    _ => Task::none(),
                }
            },
            DashboardMessage::SetScriptFormContent(action) => {
                match &mut self.item_box {
                    ItemBox::NewScript(form) | ItemBox::EditScript(form) => {
                        form.content.perform(action);
                        Task::none()
                    },
                    _ => Task::none(),
                }
            },
            DashboardMessage::SetScriptFormDescription(action) => {
                match &mut self.item_box {
                    ItemBox::NewScript(form) | ItemBox::EditScript(form) => {
                        form.description.perform(action);
                        Task::none()
                    },
                    _ => Task::none(),
                }
            },
            DashboardMessage::SetScriptFormTag(tag) => {
                match &mut self.item_box {
                    ItemBox::NewScript(form) | ItemBox::EditScript(form) => {
                        form.tag = tag;
                        Task::none()
                    },
                    _ => Task::none(),
                }
            },
            DashboardMessage::AddScriptFormTag => {
                match &mut self.item_box {
                    ItemBox::NewScript(form) | ItemBox::EditScript(form) => {
                        form.tags.push(form.tag.clone());
                        form.tag.clear();
                        Task::none()
                    },
                    _ => Task::none(),
                }
            },
            DashboardMessage::RemoveScriptFormTag(idx) => {
                match &mut self.item_box {
                    ItemBox::NewScript(form) | ItemBox::EditScript(form) => {
                        form.tags.remove(idx);
                        Task::none()
                    },
                    _ => Task::none(),
                }
            },
            DashboardMessage::SubmitItemBox => {
                let Some(pool) = self.pool.clone() else {
                    self.item_error = Some("Failed to save content".to_string());
                    eprintln!("Shelf: Failed to save content: no database connection");
                    return Task::none();
                };

                match &self.item_box {
                    ItemBox::NewGroup(form) => {
                        let name = form.name.clone();
                        let description = form.description.text();
                        Task::perform(
                            async move { create_group(pool, name, description).await.map(|_| ()) },
                            DashboardMessage::ItemBoxDone,
                        )
                    },
                    ItemBox::EditGroup(form) => {
                        let id = form.id;
                        let name = form.name.clone();
                        let description = form.description.text();
                        Task::perform(
                            async move { update_group(pool, id, name, description).await },
                            DashboardMessage::ItemBoxDone,
                        )
                    },
                    ItemBox::DeleteGroup(form) => {
                        let id = form.id;
                        Task::perform(
                            async move { delete_group(pool, id).await },
                            DashboardMessage::ItemBoxDone,
                        )
                    },
                    ItemBox::NewCommand(form) => {
                        let group_id = self.current_group.id;
                        let content = form.content.clone();
                        let description = form.description.text();
                        let tags = form.tags.clone();
                        Task::perform(
                            async move {
                                create_command(pool, group_id, content, description, tags)
                                    .await
                                    .map(|_| ())
                            },
                            DashboardMessage::ItemBoxDone,
                        )
                    },
                    ItemBox::EditCommand(form) => {
                        let id = form.id;
                        let content = form.content.clone();
                        let description = form.description.text();
                        let tags = form.tags.clone();
                        Task::perform(
                            async move {
                                update_command(pool, id, content, description, tags)
                                    .await
                                    .map(|_| ())
                            },
                            DashboardMessage::ItemBoxDone,
                        )
                    },
                    ItemBox::DeleteCommand(form) => {
                        let id = form.id;
                        Task::perform(
                            async move { delete_command(pool, id).await },
                            DashboardMessage::ItemBoxDone,
                        )
                    },
                    ItemBox::NewScript(form) => {
                        let group_id = self.current_group.id;
                        let name = form.name.clone();
                        let content = form.content.text();
                        let description = form.description.text();
                        let tags = form.tags.clone();
                        Task::perform(
                            async move {
                                create_script(pool, group_id, name, content, description, tags)
                                    .await
                                    .map(|_| ())
                            },
                            DashboardMessage::ItemBoxDone,
                        )
                    },
                    ItemBox::EditScript(form) => {
                        let id = form.id;
                        let name = form.name.clone();
                        let content = form.content.text();
                        let description = form.description.text();
                        let tags = form.tags.clone();
                        Task::perform(
                            async move {
                                update_script(pool, id, name, content, description, tags)
                                    .await
                                    .map(|_| ())
                            },
                            DashboardMessage::ItemBoxDone,
                        )
                    },
                    ItemBox::DeleteScript(form) => {
                        let id = form.id;
                        Task::perform(
                            async move { delete_script(pool, id).await },
                            DashboardMessage::ItemBoxDone,
                        )
                    }
                    _ => Task::none(),
                }
            },
            DashboardMessage::CloseItemBox => {
                self.item_box = ItemBox::None;
                self.item_error = None;
                Task::none()
            },
            DashboardMessage::ItemBoxDone(result) => {
                match result {
                    Ok(()) => {
                        match self.item_box {
                            ItemBox::NewGroup(_) | ItemBox::EditGroup(_) | ItemBox::DeleteGroup(_) => {
                                self.item_box = ItemBox::None;
                                self.item_error = None;
                                Task::done(DashboardMessage::GroupNavigatorMessage(GroupNavigatorMessage::ReloadGroupNavigator))
                            },
                            ItemBox::NewCommand(_) | ItemBox::EditCommand(_) | ItemBox::DeleteCommand(_)
                            | ItemBox::NewScript(_) | ItemBox::EditScript(_) | ItemBox::DeleteScript(_) => {
                                self.item_box = ItemBox::None;
                                self.item_error = None;
                                Task::done(DashboardMessage::GroupMessage(GroupMessage::ReloadGroup))
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
                Some(group_navigator) => group_navigator.update(group_navigator_m, &mut self.current_group, &mut self.item_box).map(|m| DashboardMessage::GroupNavigatorMessage(m)),
                None => Task::none(),
            },
            DashboardMessage::GroupMessage(group_m) => match &mut self.group {
                Some(group) => group.update(group_m, &mut self.current_group, &mut self.item_box).map(|m| DashboardMessage::GroupMessage(m)),
                None => Task::none(),
            },
            DashboardMessage::StatusBarMessage(status_bar_m) => match &mut self.status_bar {
                Some(status_bar) => status_bar.update(status_bar_m).map(|m| DashboardMessage::StatusBarMessage(m)),
                None => Task::none(),
            },
        }
    }
}
