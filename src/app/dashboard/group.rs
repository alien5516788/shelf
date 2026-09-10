use std::sync::Arc;

use iced::font::{Family, Style};
use iced::widget::text_editor::Content;
use iced::widget::{Grid, button, column, container, row, space, text};
use iced::{Alignment, Background, Border, Color, Element, Font, Length, Task};
use sqlx::SqlitePool;
use sqlx::types::chrono::NaiveDateTime;

use crate::app::dashboard::group::GroupMessage::ReloadGroup;
use crate::app::dashboard::{GroupInfo, ItemBox, ItemForm};
use crate::icon;

use crate::services::command::load_commands_for_group;
use crate::services::script::load_scripts_for_group;


#[derive(Debug, Clone)]
pub struct Group {
    command_list: Vec<CommandInfo>,
    script_list: Vec<ScriptInfo>,

    pool: Option<Arc<SqlitePool>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommandInfo {
    pub id: i32,
    pub content: String,
    pub description: String,
    pub is_favourite: bool,
    pub last_used_at: Option<NaiveDateTime>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScriptInfo {
    pub id: i32,
    pub name: String,
    pub content: String,
    pub description: String,
    pub is_favourite: bool,
    pub last_used_at: Option<NaiveDateTime>,
    pub tags: Vec<String>,
}


#[derive(Debug, Clone)]
pub enum GroupMessage {
    LoadGroup(Arc<SqlitePool>),
    ReloadGroup,
    SetCommandList(Vec<CommandInfo>),
    SetScriptList(Vec<ScriptInfo>),

    OpenEditGroupBox(GroupInfo),
    OpenDeleteGroupBox(GroupInfo),
    OpenNewCommandBox,
    OpenEditCommandBox(CommandInfo),
    OpenDeleteCommandBox(CommandInfo),
    OpenNewScriptBox,
    OpenEditScriptBox(ScriptInfo),
    OpenDeleteScriptBox(ScriptInfo),
}

impl Group {
    pub fn new(pool: Arc<SqlitePool>) -> (Self, Task<GroupMessage>) {
        (
            Self {
                command_list: Vec::new(),
                script_list: Vec::new(),

                pool: None,
            },

            Task::done(GroupMessage::LoadGroup(pool)),
        )
    }

    pub fn view(&self, current_group: &GroupInfo) -> Element<'_, GroupMessage> {
        container(
            column![
                // Fancy group name
                row![
                    text("user@shelf")
                        .color(Color::from_rgb(0.3, 0.9, 0.4)),
                    text(":")
                        .color(Color::from_rgb(1.0, 1.0, 1.0)),
                    text("~/")
                        .color(Color::from_rgb(0.3, 0.5, 1.0)),
                    text(current_group.name.clone())
                        .color(Color::from_rgb(0.3, 0.5, 1.0)),
                    text("$")
                        .color(Color::from_rgb(1.0, 1.0, 1.0)),
                ],

                // Group controls and filters
                row![
                    //Filters
                    button(icon::square_terminal()
                        .size(20.0)
                        .color(Color::from_rgb(0.5, 0.9, 0.9)))
                        .style(|_, _| button::Style {
                            background: None,
                            ..Default::default()
                        }),

                    button(icon::code_xml()
                        .size(20.0)
                        .color(Color::from_rgb(0.5, 0.9, 0.9)))
                        .style(|_, _| button::Style {
                            background: None,
                            ..Default::default()
                        }),
                    button(icon::a_large_small()
                        .size(20.0)
                        .color(Color::from_rgb(0.5, 0.9, 0.9)))
                        .style(|_, _| button::Style {
                            background: None,
                            ..Default::default()
                        }),

                    // Space
                    space()
                        .width(Length::Fill),

                    // Add new command
                    button(
                        row![
                            icon::plus()
                                .size(20.0)
                                .color(Color::from_rgb(0.3, 0.9, 0.4)),

                            text("Command")
                                .color(Color::from_rgb(0.3, 0.9, 0.4)),
                        ]
                        .align_y(Alignment::Center)
                        .spacing(10)
                    )
                    .on_press(GroupMessage::OpenNewCommandBox)
                    .style(|_, _| button::Style {
                        background: None,
                        border: Border {
                            color: Color::from_rgb(0.3, 0.9, 0.4),
                            width: 2.0,
                            radius: 4.into(),
                        },
                        ..Default::default()
                    }),

                    // Space
                    space()
                        .width(Length::Fixed(10.0)),

                    // Add new script
                    button(
                        row![
                            icon::plus()
                                .size(20.0)
                                .color(Color::from_rgb(0.3, 0.9, 0.4)),

                            text("Script")
                                .color(Color::from_rgb(0.3, 0.9, 0.4)),
                        ]
                        .align_y(Alignment::Center)
                        .spacing(10)
                    )
                    .on_press(GroupMessage::OpenNewScriptBox)
                    .style(|_, _| button::Style {
                        background: None,
                        border: Border {
                            color: Color::from_rgb(0.3, 0.9, 0.4),
                            width: 2.0,
                            radius: 4.into(),
                        },
                        ..Default::default()
                    }),

                ],

                // Group description and edit button
                row![
                    // Description
                    container(
                        match current_group.description.as_str() {
                            "" => text("No description available")
                                .color(Color::from_rgb(0.5, 0.5, 0.5))
                                .font(Font {
                                    family: Family::Monospace,
                                    style: Style::Italic,
                                    ..Default::default()
                                }),
                            description => text(description.to_string())
                                .color(Color::from_rgb(0.9, 0.9, 0.9)),
                        }
                    )
                    .width(Length::Fill)
                    .padding(10)
                    .style(|_| container::Style {
                        background: Some(Background::Color(Color::from_rgba(0.2, 0.2, 0.3, 0.5))),
                        ..Default::default()
                    }),

                    // Space
                    space()
                        .width(10.0),

                    // Edit group
                    button(
                        column![
                            space()
                                .height(5.0),

                            icon::pen()
                                .size(15.0)
                                .color(Color::from_rgb(0.5, 0.9, 0.9))
                        ]
                        .height(Length::Fill)
                    )
                    .on_press(GroupMessage::OpenEditGroupBox(current_group.clone()))
                    .height(40.0)
                    .padding(0)
                    .style(|_, _| button::Style {
                        background: None,
                        ..Default::default()
                    }),

                    // Space
                    space()
                        .width(10.0),

                    // Delete group
                    button(
                        column![
                            space()
                                .height(5.0),

                            icon::trash()
                                .size(15.0)
                                .color(Color::from_rgb(0.9, 0.1, 0.1))
                        ]
                        .height(Length::Fill)
                    )
                    .on_press(GroupMessage::OpenDeleteGroupBox(current_group.clone()))
                    .height(40.0)
                    .padding(0)
                    .style(|_, _| button::Style {
                        background: None,
                        ..Default::default()
                    })
                ],

                // Commands
                self.command_list
                    .iter()
                    .fold(Grid::new(), |column, command| column.push(Self::command_card_view(command))),

                // Scripts
                self.script_list
                    .iter()
                    .fold(Grid::new(), |column, script| column.push(Self::script_card_view(script))),
            ]
            .spacing(10)
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .padding(10)
        .style(|_| container::Style {
            border: Border {
                color: Color::from_rgb(0.4, 0.4, 0.4),
                width: 0.5,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
    }

    fn command_card_view(command: &CommandInfo) -> Element<'_, GroupMessage> {
        container(
            row![
                column![
                    text(format!("{}", command.content)),
                    text(
                        format!(
                            "{}",
                            match command.description.as_str() {
                                "" => "No description available".to_string(),
                                description => description.to_string(),
                            }
                        )),
                ]
            ]
        )
        .into()
    }

    fn script_card_view(script: &ScriptInfo) -> Element<'_, GroupMessage> {
        container(
            row![
                column![
                    text(format!("{}", script.content)),
                    text(
                        format!(
                            "{}",
                            match script.description.as_str() {
                                "" => "No description available".to_string(),
                                description => description.to_string(),
                            }
                        )),
                ]
            ]
        )
        .into()
    }

    pub fn update(&mut self, message: GroupMessage, current_group: &mut GroupInfo, item_box: &mut ItemBox, item_form: &mut ItemForm) -> Task<GroupMessage> {
        match message {
            GroupMessage::LoadGroup(pool) => {
                self.pool = Some(pool.clone());

                Task::batch([
                    Task::perform(
                        load_commands_for_group(pool.clone(), current_group.id),
                        |commands| match commands {
                            Ok(commands) => GroupMessage::SetCommandList(
                                commands.into_iter().map(|(command, tags)| CommandInfo {
                                    id: command.id,
                                    content: command.content,
                                    description: command.description,
                                    is_favourite: match command.is_favourite {
                                        0 => false,
                                        _ => true,
                                    },
                                    last_used_at: command.last_used_at.as_ref().and_then(|s| {
                                        NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok()
                                    }),
                                    tags: tags,
                                }).collect()
                            ),
                            Err(e) => {
                                eprintln!("Failed to load commands: {}", e);
                                GroupMessage::SetCommandList(Vec::from([]))
                            },
                        }
                    ),
                    Task::perform(
                        load_scripts_for_group(pool, current_group.id),
                        |scripts| match scripts {
                            Ok(scripts) => GroupMessage::SetScriptList(
                                scripts.into_iter().map(|(script, tags)| ScriptInfo {
                                    id: script.id,
                                    name: script.name,
                                    content: script.content,
                                    description: script.description,
                                    is_favourite: match script.is_favourite {
                                        0 => false,
                                        _ => true,
                                    },
                                    last_used_at: script.last_used_at.as_ref().and_then(|s| {
                                        NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok()
                                    }),
                                    tags: tags,
                                }).collect()
                            ),
                            Err(e) => {
                                eprintln!("Failed to load scripts: {}", e);
                                GroupMessage::SetScriptList(Vec::from([]))
                            },
                        }
                    ),
                ])
            },
            ReloadGroup => {
                // TODO: Remove dupliacte later
                //
                let Some(pool) = self.pool.clone() else {
                    return Task::none();
                };

                Task::batch([
                    Task::perform(
                        load_commands_for_group(pool.clone(), current_group.id),
                        |commands| match commands {
                            Ok(commands) => GroupMessage::SetCommandList(
                                commands.into_iter().map(|(command, tags)| CommandInfo {
                                    id: command.id,
                                    content: command.content,
                                    description: command.description,
                                    is_favourite: match command.is_favourite {
                                        0 => false,
                                        _ => true,
                                    },
                                    last_used_at: command.last_used_at.as_ref().and_then(|s| {
                                        NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok()
                                    }),
                                    tags: tags,
                                }).collect()
                            ),
                            Err(e) => {
                                eprintln!("Failed to load commands: {}", e);
                                GroupMessage::SetCommandList(Vec::from([]))
                            },
                        }
                    ),
                    Task::perform(
                        load_scripts_for_group(pool, current_group.id),
                        |scripts| match scripts {
                            Ok(scripts) => GroupMessage::SetScriptList(
                                scripts.into_iter().map(|(script, tags)| ScriptInfo {
                                    id: script.id,
                                    name: script.name,
                                    content: script.content,
                                    description: script.description,
                                    is_favourite: match script.is_favourite {
                                        0 => false,
                                        _ => true,
                                    },
                                    last_used_at: script.last_used_at.as_ref().and_then(|s| {
                                        NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok()
                                    }),
                                    tags: tags,
                                }).collect()
                            ),
                            Err(e) => {
                                eprintln!("Failed to load scripts: {}", e);
                                GroupMessage::SetScriptList(Vec::from([]))
                            },
                        }
                    ),
                ])
            },
            GroupMessage::SetCommandList(commands) => {
                self.command_list = commands;
                Task::none()
            },
            GroupMessage::SetScriptList(scripts) => {
                self.script_list = scripts;
                Task::none()
            },
            GroupMessage::OpenEditGroupBox(group) => {
                *item_box = ItemBox::EditGroup;
                *item_form = ItemForm {
                    id: Some(group.id),
                    name: Some(group.name),
                    description: Some(Content::new()),
                    ..Default::default()
                };
                Task::none()
            },
            GroupMessage::OpenDeleteGroupBox(group) => {
                *item_box = ItemBox::DeleteGroup;
                *item_form = ItemForm {
                    id: Some(group.id),
                    name: Some(group.name),
                    ..Default::default()
                };
                Task::none()
            },
            GroupMessage::OpenNewCommandBox => {
                *item_box = ItemBox::NewCommand;
                *item_form = ItemForm {
                    name: Some(String::new()),
                    description: Some(Content::new()),
                    tag: Some(String::new()),
                    tags: Some(Vec::new()),
                    ..Default::default()
                };
                Task::none()
            },
            GroupMessage::OpenEditCommandBox(command) => {
                *item_box = ItemBox::EditCommand;
                *item_form = ItemForm {
                    id: Some(command.id),
                    name: Some(command.content),
                    description: Some(Content::with_text(&command.description)),
                    tag: Some(String::new()),
                    tags: Some(command.tags),
                    ..Default::default()
                };
                Task::none()
            },
            GroupMessage::OpenDeleteCommandBox(command) => {
                *item_box = ItemBox::DeleteCommand;
                *item_form = ItemForm {
                    id: Some(command.id),
                    name: Some(command.content),
                    ..Default::default()
                };
                Task::none()
            },
            GroupMessage::OpenNewScriptBox => {
                *item_box = ItemBox::NewScript;
                *item_form = ItemForm {
                    name: Some(String::new()),
                    content: Some(Content::new()),
                    description: Some(Content::new()),
                    tag: Some(String::new()),
                    tags: Some(Vec::new()),
                    ..Default::default()
                };
                Task::none()
            },
            GroupMessage::OpenEditScriptBox(script) => {
                *item_box = ItemBox::EditScript;
                *item_form = ItemForm {
                    id: Some(script.id),
                    name: Some(script.name),
                    content: Some(Content::with_text(&script.content)),
                    description: Some(Content::with_text(&script.description)),
                    tag: Some(String::new()),
                    tags: Some(script.tags),
                    ..Default::default()
                };
                Task::none()
            },
            GroupMessage::OpenDeleteScriptBox(script) => {
                *item_box = ItemBox::DeleteScript;
                *item_form = ItemForm {
                    id: Some(script.id),
                    name: Some(script.name),
                    ..Default::default()
                };
                Task::none()
            },
        }
    }
}
