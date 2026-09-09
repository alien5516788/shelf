use iced::font::{Family, Style};
use iced::widget::text_editor::Content;
use iced::widget::tooltip::Position;
use iced::widget::{Grid, Space, button, column, container, row, text, tooltip};
use iced::{Alignment, Background, Border, Color, Element, Font, Length, Task};
use tokio::time::Instant;

use crate::app::dashboard::group::GroupMessage::OpenEditGroupBox;
use crate::app::dashboard::{GroupInfo, ItemBox, GroupForm, CommandForm, ScriptForm};
use crate::icon;


#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    command_list: Vec<CommandInfo>,
    script_list: Vec<ScriptInfo>
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommandInfo {
    pub id: i32,
    pub content: String,
    pub description: String,
    pub is_favourite: bool,
    pub last_used_at: Instant,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScriptInfo {
    pub id: i32,
    pub name: String,
    pub content: String,
    pub description: String,
    pub is_favourite: bool,
    pub last_used_at: Instant,
    pub tags: Vec<String>,
}


#[derive(Debug, Clone)]
pub enum GroupMessage {
    OpenEditGroupBox(GroupInfo),
    OpenDeleteGroupBox(i32),
    OpenNewCommandBox,
    OpenEditCommandBox(CommandInfo),
    OpenDeleteCommandBox(i32),
    OpenNewScriptBox,
    OpenEditScriptBox(ScriptInfo),
    OpenDeleteScriptBox(i32),
}

impl Group {
    pub fn new() -> (Self, Task<GroupMessage>) {
        (
            Self {
                command_list: Vec::new(),
                script_list: Vec::new(),
            },
            Task::none(),
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
                    Space::new()
                        .width(10.0),

                    // Edit group
                    tooltip(
                        button(
                            column![
                                Space::new()
                                    .height(5.0),

                                icon::pen()
                                    .size(15.0)
                                    .color(Color::from_rgb(0.5, 0.9, 0.9))
                            ]
                            .height(Length::Fill)
                        )
                        .on_press(OpenEditGroupBox(current_group.clone()))
                        .height(40.0)
                        .padding(0)
                        .style(|_, _| button::Style {
                            background: None,
                            ..Default::default()
                        }),

                        text("Edit Group")
                            .size(15.0)
                            .color(Color::from_rgb(0.8, 0.8, 0.8)),

                        Position::Top
                    ),
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
                    Space::new().width(Length::Fill),

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
                    Space::new()
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
                ],
                column![

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
                ],
                column![

                ]
            ]
        )
        .into()
    }

    pub fn update(&mut self, message: GroupMessage, item_box: &mut ItemBox) -> Task<GroupMessage> {
        match message {
            GroupMessage::OpenEditGroupBox(group_info) => {
                *item_box = ItemBox::EditGroup(GroupForm {
                    name: group_info.name,
                    description: Content::with_text(&group_info.description)
                });
                Task::none()
            },
            GroupMessage::OpenDeleteGroupBox(group_id) => {
                *item_box = ItemBox::DeleteGroup(group_id);
                Task::none()
            },
            GroupMessage::OpenNewCommandBox => {
                *item_box = ItemBox::NewCommand(CommandForm {
                    content: String::new(),
                    description: Content::new(),
                    tag: String::new(),
                    tags: Vec::new(),
                });
                Task::none()
            },
            GroupMessage::OpenEditCommandBox(item_info) => {
                *item_box = ItemBox::EditCommand(CommandForm {
                    content: item_info.content,
                    description: Content::with_text(&item_info.description),
                    tag: String::new(),
                    tags: item_info.tags,
                });
                Task::none()
            },
            GroupMessage::OpenDeleteCommandBox(item_id) => {
                *item_box = ItemBox::DeleteCommand(item_id);
                Task::none()
            },
            GroupMessage::OpenNewScriptBox => {
                *item_box = ItemBox::NewScript(ScriptForm {
                    name: String::new(),
                    content: Content::new(),
                    description: Content::new(),
                    tag: String::new(),
                    tags: Vec::new(),
                });
                Task::none()
            },
            GroupMessage::OpenEditScriptBox(item_info) => {
                *item_box = ItemBox::EditScript(ScriptForm {
                    name: item_info.name.clone(),
                    content: Content::with_text(&item_info.content),
                    description: Content::with_text(&item_info.description),
                    tag: String::new(),
                    tags: item_info.tags,
                });
                Task::none()
            },
            GroupMessage::OpenDeleteScriptBox(item_id) => {
                *item_box = ItemBox::DeleteScript(item_id);
                Task::none()
            },
        }
    }
}
