use std::sync::Arc;

use iced::font::{Family, Style};
use iced::widget::text::{Span, Wrapping};
use iced::widget::text_editor::Content;
use iced::widget::{Column, Row, button, center_x, column, container, mouse_area, rich_text, row, scrollable, space, text};
use iced::{Alignment, Background, Border, Color, Element, Font, Length, Task};
use sqlx::SqlitePool;
use sqlx::types::chrono::NaiveDateTime;

use crate::app::dashboard::{GroupInfo, ItemBox, ItemForm};
use crate::icon;
use crate::services::command::{CommandRow, load_commands_for_group};
use crate::services::script::{ScriptRow, load_scripts_for_group};


#[derive(Debug, Clone)]
pub struct Group {
    description_collapsed: bool,
    command_list: Vec<CommandInfo>,
    script_list: Vec<ScriptInfo>,
    item_hovered: Option<(char, i32)>, // ('c'/'s', id)

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

#[derive(Debug, Clone, PartialEq)]
pub enum ItemInfo<'a> {
    Script(&'a ScriptInfo),
    Command(&'a CommandInfo),
}

#[derive(Debug, Clone)]
pub enum GroupMessage {
    SetPool(Arc<SqlitePool>),
    LoadGroup,
    ToggleDescriptionCollapsed,
    SetCommandList(Result<Vec<(CommandRow, Vec<String>)>, String>),
    SetScriptList(Result<Vec<(ScriptRow, Vec<String>)>, String>),
    SetItemHovered(Option<(char, i32)>), // Make the type formal
    CopyItemContent(String),

    OpenEditGroupBox(GroupInfo),
    OpenNewCommandBox,
    OpenEditCommandBox(CommandInfo),
    OpenDeleteCommandBox(CommandInfo),
    OpenNewScriptBox,
    OpenEditScriptBox(ScriptInfo),
    OpenDeleteScriptBox(ScriptInfo),
    AddItemToFavourites((char, i32)), // Make the type formal
}

impl Group {
    pub fn new(pool: Arc<SqlitePool>) -> (Self, Task<GroupMessage>) {
        (
            Self {
                description_collapsed: true,
                command_list: Vec::new(),
                script_list: Vec::new(),
                item_hovered: None,

                pool: None,
            },

            Task::done(GroupMessage::SetPool(pool)),
        )
    }

    pub fn view(&self, current_group: &GroupInfo) -> Element<'_, GroupMessage> {
        container(
            column![
                // Fancy group name
                rich_text![
                    Span::<&str>::new("user@shelf").color(Color::from_rgb(0.3, 0.9, 0.4)),
                    Span::new(":").color(Color::from_rgb(1.0, 1.0, 1.0)),
                    Span::new("~/").color(Color::from_rgb(0.3, 0.5, 1.0)),
                    Span::new(current_group.name.clone()).color(Color::from_rgb(0.3, 0.5, 1.0)),
                    Span::new("$ ").color(Color::from_rgb(1.0, 1.0, 1.0)),
                ]
                .wrapping(Wrapping::WordOrGlyph),

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

                    space()
                        .width(Length::Fixed(20.0)),

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
                    // TODO: Make a gradient overlay for collapsed description
                    container(
                        row![
                            container(match current_group.description.as_str() {
                                "" => text("No description available")
                                    .size(16)
                                    .color(Color::from_rgb(0.5, 0.5, 0.5))
                                    .font(Font {
                                        family: Family::Monospace,
                                        style: Style::Italic,
                                        ..Default::default()
                                    }),
                                description => text(description.to_string())
                                    .size(16)
                                    .color(Color::from_rgb(0.9, 0.9, 0.9)),
                            })
                            .width(Length::Fill),

                            button(match self.description_collapsed {
                                true => icon::chevron_down()
                                    .size(20.0)
                                    .color(Color::from_rgb(0.5, 0.85, 0.9)),
                                false => icon::chevron_up()
                                    .size(20.0)
                                    .color(Color::from_rgb(0.5, 0.85, 0.9)),
                            })
                            .on_press(GroupMessage::ToggleDescriptionCollapsed)
                            .style(|_, _| button::Style {
                                background: None,
                                ..Default::default()
                            }),
                        ]
                    )
                    .height(match self.description_collapsed {
                        true => Length::Fixed(20.0 * 2.5),
                        false => Length::Shrink,
                    })
                    .width(Length::Fill)
                    .padding(10)
                    .clip(true)
                    .style(|_| container::Style {
                        background: Some(Background::Color(Color::from_rgba(0.2, 0.2, 0.3, 0.5))),
                        ..Default::default()
                    }),
                ],

                // Item list
                container(
                    scrollable(
                        center_x(
                            column![
                                // Commands
                                self.command_list
                                    .iter()
                                    .fold(
                                        Column::new()
                                            .spacing(10),
                                        |column, command| column.push(Self::item_card_view(ItemInfo::Command(command), &self.item_hovered))),

                                // Scripts
                                self.script_list
                                    .iter()
                                    .fold(
                                        Column::new()
                                            .spacing(10),
                                        |column, script| column.push(Self::item_card_view(ItemInfo::Script(script), &self.item_hovered))),
                            ]
                        )
                    )
                )
            ]
            .spacing(15)
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

    fn item_card_view<'a>(item: ItemInfo<'a>, item_hovered: &'a Option<(char, i32)>) -> Element<'a, GroupMessage> {
        // Hovered state
        let hovered = match item_hovered {
            Some(id) => *id == match item {
                ItemInfo::Command(command) => ('c', command.id),
                ItemInfo::Script(script) => ('s', script.id),
            },
            None => false,
        };

        let (icon, name, content, description, tags, is_favourite) = match item {
            ItemInfo::Command(command) => (
                icon::square_terminal(),
                command.content.as_str(),
                None,
                command.description.as_str(),
                command.tags.iter().collect::<Vec<_>>(),
                command.is_favourite,
            ),
            ItemInfo::Script(script) => (
                icon::code_xml(),
                script.name.as_str(),
                Some(script.content.as_str()),
                script.description.as_str(),
                script.tags.iter().collect::<Vec<_>>(),
                script.is_favourite,
            ),
        };

        mouse_area(
            container(
                row![
                    // Icon, Content, Description and Tags
                    column![
                        // Icon, Name
                        row![
                            button(
                                row![
                                    // Icon
                                    icon
                                        .size(20.0)
                                        .color(Color::from_rgb(0.3, 0.4, 0.6)),

                                    // Name of script or Content of command
                                    text(name)
                                        .wrapping(Wrapping::WordOrGlyph)
                                        .size(15)
                                        .color(Color::from_rgb(0.85, 0.95, 0.75)),

                                    space().width(Length::Fill),
                                ]
                                .width(Length::Fill)
                                .spacing(8),
                            )
                            .on_press(GroupMessage::CopyItemContent(name.to_string()))
                            .style(|_, _| button::Style { background: None, ..Default::default() }),
                        ]
                        .spacing(8)
                        .align_y(Alignment::Center),

                        // Content
                        // Only for scripts
                        button(
                            text(match content {
                                Some(content) => content,
                                None => "",
                            })
                            .size(13)
                            .color(Color::from_rgb(0.55, 0.55, 0.6)),
                        )
                        .on_press(GroupMessage::CopyItemContent(match content {
                            Some(content) => content.to_string(),
                            None => "".to_string(),
                        }))
                        .style(|_, _| button::Style { background: None, ..Default::default() }),

                        // Description
                        text(description)
                        .size(13)
                        .color(Color::from_rgb(0.55, 0.55, 0.6)),

                        // Tags
                        tags
                            .iter()
                            .fold(
                                Row::new().spacing(6),
                                |row, tag| {
                                    row.push(
                                        container(
                                            text(tag.as_str()).size(12)
                                        )
                                        .padding([3, 8])
                                        .style(|_| container::Style {
                                            background: Some(Background::Color(Color::from_rgb(0.2, 0.25, 0.32))),
                                            border: Border {
                                                radius: 4.0.into(),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        }),
                                    )
                                }
                            ),
                    ]
                    .spacing(8),

                    // Favourite, Edit, Delete buttons
                    column![
                        // Run
                        button(icon::play().size(14).color(Color::from_rgb(0.5, 0.85, 0.9)))
                            .style(|_, _| button::Style { background: None, ..Default::default() }),

                        // Favourite
                        match is_favourite {
                            // If the item is a favourite, show the star icon
                            true => container(
                                button(
                                    icon::star()
                                        .size(16)
                                        .color(Color::from_rgb(1.0, 0.85, 0.3))
                                )
                                .on_press(GroupMessage::AddItemToFavourites(
                                    match item {
                                        ItemInfo::Command(_) => ('c', 0),
                                        ItemInfo::Script(_) => ('s', 0),
                                    }
                                ))
                                .style(|_, _| button::Style { background: None, ..Default::default() })
                            ),
                            // If the item is not a favourite, hide the star icon but show the empty star icon on hover
                            false => match hovered {
                                true => container(
                                    button(
                                        icon::star()
                                            .size(16)
                                            .color(Color::from_rgb(0.45, 0.45, 0.5))
                                    )
                                    .on_press(GroupMessage::AddItemToFavourites(
                                        match item {
                                            ItemInfo::Command(_) => ('c', 0),
                                            ItemInfo::Script(_) => ('s', 0),
                                        }
                                    ))
                                    .style(|_, _| button::Style { background: None, ..Default::default() })
                                ),
                                false => container(space()),
                            },
                        },

                        // Edit, Delete
                        // Show the edit and delete buttons on hover
                        match hovered {
                            true => column![
                                button(icon::pen().size(14).color(Color::from_rgb(0.5, 0.85, 0.9)))
                                    .on_press(match item {
                                        ItemInfo::Command(command) => GroupMessage::OpenEditCommandBox(command.clone()),
                                        ItemInfo::Script(script) => GroupMessage::OpenEditScriptBox(script.clone()),
                                    })
                                    .style(|_, _| button::Style { background: None, ..Default::default() }),

                                button(icon::trash().size(14).color(Color::from_rgb(0.9, 0.35, 0.35)))
                                    .on_press(match item {
                                        ItemInfo::Command(command) => GroupMessage::OpenDeleteCommandBox(command.clone()),
                                        ItemInfo::Script(script) => GroupMessage::OpenDeleteScriptBox(script.clone()),
                                    })
                                    .style(|_, _| button::Style { background: None, ..Default::default() })
                            ],
                            false => column![],
                        }
                        .height(Length::Shrink)
                        .width(35)
                        .spacing(10),
                    ]
                    .spacing(10)
                ]
            )
            .width(Length::Fixed(800.0))
            .height(Length::Shrink)
            .padding(12)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgb(0.14, 0.14, 0.17))),
                border: Border {
                    color: Color::from_rgb(0.28, 0.28, 0.32),
                    width: 1.0,
                    ..Default::default()
                },
                ..Default::default()
            })
        )
        .on_enter(GroupMessage::SetItemHovered(
            Some(
                match item {
                    ItemInfo::Command(command) => ('c', command.id),
                    ItemInfo::Script(script) => ('s', script.id),
                },
            ),
        ))
        .on_exit(GroupMessage::SetItemHovered(None))
        .into()
    }

    pub fn update(&mut self, message: GroupMessage, current_group: &mut GroupInfo, item_box: &mut ItemBox, item_form: &mut ItemForm) -> Task<GroupMessage> {
        match message {
            GroupMessage::SetPool(pool) => {
                self.pool = Some(pool);
                Task::done(GroupMessage::LoadGroup)
            },
            GroupMessage::LoadGroup => {
                let pool = match self.pool.clone() {
                    Some(pool) => pool,
                    None => return Task::none(),
                };

                Task::batch([
                    Task::perform(
                        load_commands_for_group(pool.clone(), current_group.id),
                        GroupMessage::SetCommandList
                    ),
                    Task::perform(
                        load_scripts_for_group(pool, current_group.id),
                        GroupMessage::SetScriptList
                    ),
                ])
            },
            GroupMessage::ToggleDescriptionCollapsed => {
                self.description_collapsed = !self.description_collapsed;
                Task::none()
            },
            GroupMessage::SetCommandList(commands) => {
                match commands {
                    Ok(commands) => self.command_list = Self::command_row_to_command_info(commands),
                    Err(e) => {
                        eprintln!("Failed to load commands: {}", e);
                        self.command_list = Vec::new();
                    },
                }
                Task::none()
            },
            GroupMessage::SetScriptList(scripts) => {
                match scripts {
                    Ok(scripts) => self.script_list = Self::script_row_to_script_info(scripts),
                    Err(e) => {
                        eprintln!("Failed to load scripts: {}", e);
                        self.script_list = Vec::new()
                    },
                }
                Task::none()
            },
            GroupMessage::SetItemHovered(hovered) => {
                self.item_hovered = hovered;
                Task::none()
            },
            GroupMessage::CopyItemContent(content) => {
                // TODO
                println!("Copied: {}", content);
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
            GroupMessage::AddItemToFavourites(item) => {
                // TODO
                println!("Favourited: {:?}", item);
                Task::none()
            },
        }
    }

    fn command_row_to_command_info(commands: Vec<(CommandRow, Vec<String>)>) -> Vec<CommandInfo> {
        commands
            .into_iter()
            .map(|(command, tags)| CommandInfo {
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
            })
            .collect()
    }

    fn script_row_to_script_info(scripts: Vec<(ScriptRow, Vec<String>)>) -> Vec<ScriptInfo> {
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
        })
        .collect()
    }
}
