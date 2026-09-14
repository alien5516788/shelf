use std::sync::Arc;

use iced::font::{Family, Style};
use iced::widget::text::{Span, Wrapping};
use iced::widget::text_editor::Content;
use iced::widget::{Column, Row, button, center_x, column, container, mouse_area, rich_text, row, scrollable, space, text};
use iced::{Alignment, Background, Border, Color, Element, Font, Length, Padding, Task, Theme};
use iced::clipboard;
use sqlx::SqlitePool;
use sqlx::types::chrono::NaiveDateTime;

use crate::app::dashboard::{GroupInfo, ItemBox, ItemForm};
use crate::icon;
use crate::services::item::{ItemRow, load_items_for_group, update_item_favourite, update_item_used};


#[derive(Debug, Clone)]
pub struct Group {
    description_collapsed: bool,
    item_list: Vec<ItemInfo>,
    filter: Filter,
    item_hovered: Option<i32>,

    pool: Option<Arc<SqlitePool>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemInfo {
    pub id: i32,
    pub item_type: ItemType,
    pub name: String,
    pub content: String,
    pub description: String,
    pub is_favourite: bool,
    pub last_used_at: Option<NaiveDateTime>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Command,
    Script,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Filter {
    command: bool,
    script: bool,
    alphabetical: bool,
}


#[derive(Debug, Clone)]
pub enum GroupMessage {
    SetPool(Arc<SqlitePool>),
    LoadGroup,
    ToggleDescriptionCollapsed,
    SetItemList(Result<Vec<(ItemRow, Vec<String>)>, String>),
    SetFilter(Filter),
    SetItemHovered(Option<i32>),
    CopyItemContent(i32, String),

    OpenEditGroupBox(GroupInfo),
    OpenNewItemBox(ItemType),
    OpenEditItemBox(ItemInfo),
    OpenDeleteItemBox(ItemInfo),
    UpdateItemFavourite(i32, bool),
    UpdateItemUsed(i32),
    None,
}

impl Group {
    pub fn new(pool: Arc<SqlitePool>) -> (Self, Task<GroupMessage>) {
        (
            Self {
                description_collapsed: true,
                item_list: Vec::new(),
                item_hovered: None,
                filter: Filter {
                    command: true,
                    script: true,
                    alphabetical: true,
                },

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
                        .color(match self.filter.command {
                            true => Color::from_rgb(0.5, 0.9, 0.9),
                            false => Color::from_rgb(0.3, 0.5, 1.0),
                        })
                    )
                    .on_press(GroupMessage::SetFilter(Filter {
                        command: !self.filter.command,
                        ..self.filter
                    }))
                    .style(|_, _| button::Style {
                        background: None,
                        ..Default::default()
                    }),

                    button(icon::code_xml()
                        .size(20.0)
                        .color(match self.filter.script {
                            true => Color::from_rgb(0.5, 0.9, 0.9),
                            false => Color::from_rgb(0.3, 0.5, 1.0),
                        })
                    )
                    .on_press(GroupMessage::SetFilter(Filter {
                        script: !self.filter.script,
                        ..self.filter
                    }))
                    .style(|_, _| button::Style {
                        background: None,
                        ..Default::default()
                    }),


                    button(
                        icon::a_large_small()
                            .size(20.0)
                            .color(match self.filter.alphabetical {
                                true => Color::from_rgb(0.5, 0.9, 0.9),
                                false => Color::from_rgb(0.3, 0.5, 1.0),
                            })
                    )
                    .on_press(GroupMessage::SetFilter(Filter {
                        alphabetical: !self.filter.alphabetical,
                        ..self.filter
                    }))
                    .style(|_, _| button::Style {
                        background: None,
                        ..Default::default()
                    }),

                    // Space
                    space()
                        .width(Length::Fill),

                    if current_group.name.as_str() != "Favourites"
                    && current_group.name.as_str() != "Recent"
                    && current_group.name.as_str() != "Default" {
                        row![
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
                            .on_press(GroupMessage::OpenNewItemBox(ItemType::Command))
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
                            .on_press(GroupMessage::OpenNewItemBox(ItemType::Script))
                            .style(|_, _| button::Style {
                                background: None,
                                border: Border {
                                    color: Color::from_rgb(0.3, 0.9, 0.4),
                                    width: 2.0,
                                    radius: 4.into(),
                                },
                                ..Default::default()
                            }),
                        ]
                    } else {
                        row![]
                    }
                ],

                // Group description and edit button
                row![
                    // Description
                    // TODO: Make a gradient overlay for collapsed description
                    container(
                        row![
                            // Text
                            container(match current_group.description.as_str() {
                                "" => text("No description available")
                                    .size(16)
                                    .font(Font {
                                        family: Family::Monospace,
                                        style: Style::Italic,
                                        ..Default::default()
                                    })
                                    .style(|theme: &Theme| text::Style {
                                        color: Some(theme.extended_palette().secondary.base.color),
                                    }),
                                description => text(description.to_string())
                                    .size(16)
                                    .style(|theme: &Theme| text::Style {
                                        color: Some(theme.extended_palette().secondary.base.color),
                                    }),
                            })
                            .width(Length::Fill),

                            // Collapse button
                            button(match self.description_collapsed {
                                true => icon::chevron_down()
                                    .size(20.0),
                                false => icon::chevron_up()
                                    .size(20.0),
                            })
                            .on_press(GroupMessage::ToggleDescriptionCollapsed)
                            .style(|theme, _| button::Style {
                                background: None,
                                text_color: theme.extended_palette().secondary.base.color,
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
                    .style(|theme| container::Style {
                        background: Some(Background::Color(theme.extended_palette().secondary.base.color).scale_alpha(0.1)),
                        ..Default::default()
                    }),
                ],

                // Item list
                scrollable(
                    column![
                        center_x(
                            self.item_list
                                .iter()
                                .fold(
                                    Column::new()
                                        .spacing(10),
                                    |column, item| column.push(self.item_card_view(item)),
                                )
                        ),

                        space().height(10),
                    ]
                )
            ]
            .spacing(15)
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .padding(Padding {
            top: 10.0,
            left: 10.0,
            right: 10.0,
            ..Default::default()
        })
        .style(|theme| container::Style {
            border: Border {
                color: theme.extended_palette().secondary.weak.color,
                width: 0.5,
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
    }

    fn item_card_view<'a>(&self, item: &'a ItemInfo) -> Element<'a, GroupMessage> {
        // Hovered state
        let hovered = self.item_hovered == Some(item.id);

        let (id, icon, name, content, description, tags, is_favourite) = match item.item_type {
            ItemType::Command => (
                item.id,
                match hovered {
                    true => icon::copy(),
                    false => icon::square_terminal(),
                },
                &item.name,
                None,
                &item.description,
                item.tags.iter().collect::<Vec<_>>(),
                item.is_favourite,
            ),
            ItemType::Script => (
                item.id,
                match hovered {
                    true => icon::copy(),
                    false => icon::code_xml(),
                },
                &item.name,
                Some(&item.content),
                &item.description,
                item.tags.iter().collect::<Vec<_>>(),
                item.is_favourite,
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
                            .on_press(GroupMessage::CopyItemContent(
                                id,
                                match item.item_type {
                                    ItemType::Command => name.to_string(),
                                    ItemType::Script => match content {
                                        Some(content) => content.to_string(),
                                        None => "".to_string(),
                                    },
                                }
                            ))
                            .style(|_, _| button::Style { background: None, ..Default::default() }),
                        ]
                        .spacing(8)
                        .align_y(Alignment::Center),

                        // Content
                        text(match content {
                            Some(content) => content,
                            None => "",
                        })
                        .size(13)
                        .color(Color::from_rgb(0.5, 0.5, 0.6)),

                        // Description
                        text(description)
                        .size(13)
                        .color(Color::from_rgb(0.5, 0.5, 0.6)),

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
                        button(icon::play().size(14).color(Color::from_rgb(0.3, 0.9, 0.4)))
                            .style(|_, _| button::Style { background: None, ..Default::default() }),

                        // Favourite
                        match is_favourite {
                            // If the item is a favourite, show the star icon
                            true => container(
                                button(
                                    icon::star()
                                        .size(16)
                                )
                                .on_press(GroupMessage::UpdateItemFavourite(id, false))
                                .style(|_, _| button::Style {
                                    background: None,
                                    text_color: Color::from_rgb(0.9, 0.9, 0.5),
                                    ..Default::default()
                                })
                            ),
                            // If the item is not a favourite, hide the star icon but show the empty star icon on hover
                            false => match hovered {
                                true => container(
                                    button(
                                        icon::star()
                                            .size(16)
                                            .color(Color::from_rgb(0.45, 0.45, 0.5))
                                    )
                                    .on_press(GroupMessage::UpdateItemFavourite(id, true))
                                    .style(|theme, _| button::Style {
                                        background: None,
                                        text_color: theme.extended_palette().secondary.base.color,
                                        ..Default::default()
                                    })
                                ),
                                false => container(space()),
                            },
                        },

                        // Edit, Delete
                        // Show the edit and delete buttons on hover
                        match hovered {
                            true => column![
                                button(
                                    icon::pen().size(14)
                                )
                                .on_press(GroupMessage::OpenEditItemBox(item.clone()))
                                .style(|_, _| button::Style {
                                    background: None,
                                    text_color: Color::from_rgb(0.5, 0.9, 0.9),
                                    ..Default::default()
                                }),

                                button(
                                    icon::trash().size(14)
                                )
                                .on_press(GroupMessage::OpenDeleteItemBox(item.clone()))
                                .style(|_, _| button::Style {
                                    background: None,
                                    text_color: Color::from_rgb(0.9, 0.3, 0.3),
                                    ..Default::default()
                                })
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
            .style(|theme| container::Style {
                border: Border {
                    color: theme.extended_palette().secondary.weak.color,
                    width: 1.0,
                    ..Default::default()
                },
                ..Default::default()
            })
        )
        .on_enter(GroupMessage::SetItemHovered(Some(id)))
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
                Task::perform(
                    load_items_for_group(pool.clone(), current_group.id, self.filter.command, self.filter.script),
                    GroupMessage::SetItemList
                )
            },
            GroupMessage::ToggleDescriptionCollapsed => {
                self.description_collapsed = !self.description_collapsed;
                Task::none()
            },
            GroupMessage::SetItemList(items) => {
                match items {
                    Ok(items) => self.item_list = Self::item_row_to_item_info(items),
                    Err(e) => {
                        eprintln!("Failed to load items: {}", e);
                        self.item_list = Vec::new();
                    },
                }
                if self.filter.alphabetical {
                    self.item_list.sort_by(|a, b| a.name.cmp(&b.name));
                }
                Task::none()
            },
            GroupMessage::SetFilter(filter) => {
                // ISSUE: No items present when both command and script filters are enabled
                self.filter = filter;
                Task::done(GroupMessage::LoadGroup)
            }
            GroupMessage::SetItemHovered(hovered) => {
                self.item_hovered = hovered;
                Task::none()
            },
            GroupMessage::CopyItemContent(id, content) => {
                Task::batch([
                    clipboard::write(content),
                    Task::done(GroupMessage::UpdateItemUsed(id))
                ])
            },
            GroupMessage::OpenEditGroupBox(group) => {
                *item_box = ItemBox::EditGroup;
                *item_form = ItemForm {
                    id: Some(group.id),
                    name: Some(group.name),
                    description: Some(Content::with_text(&group.description)),
                    ..Default::default()
                };
                Task::none()
            },
            GroupMessage::OpenNewItemBox(item_type) => {
                *item_box = match item_type {
                    ItemType::Command => ItemBox::NewCommand,
                    ItemType::Script => ItemBox::NewScript,
                };
                match item_type {
                    ItemType::Command => *item_form = ItemForm {
                        name: Some(String::new()),
                        description: Some(Content::new()),
                        tag: Some(String::new()),
                        tags: Some(Vec::new()),
                        ..Default::default()
                    },
                    ItemType::Script => *item_form = ItemForm {
                        name: Some(String::new()),
                        content: Some(Content::new()),
                        description: Some(Content::new()),
                        tag: Some(String::new()),
                        tags: Some(Vec::new()),
                        ..Default::default()
                    },
                }

                Task::none()
            },
            GroupMessage::OpenEditItemBox(item) => {
                *item_box = match item.item_type {
                    ItemType::Command => ItemBox::EditCommand,
                    ItemType::Script => ItemBox::EditScript,
                };
                match item.item_type {
                    ItemType::Command => *item_form = ItemForm {
                        id: Some(item.id),
                        name: Some(item.name),
                        description: Some(Content::with_text(&item.description)),
                        tag: Some(String::new()),
                        tags: Some(item.tags),
                        ..Default::default()
                    },
                    ItemType::Script => *item_form = ItemForm {
                        id: Some(item.id),
                        name: Some(item.name),
                        content: Some(Content::with_text(&item.content)),
                        description: Some(Content::with_text(&item.description)),
                        tag: Some(String::new()),
                        tags: Some(item.tags),
                        ..Default::default()
                    },
                };
                Task::none()
            },
            GroupMessage::OpenDeleteItemBox(item) => {
                *item_box = match item.item_type {
                    ItemType::Command => ItemBox::DeleteCommand,
                    ItemType::Script => ItemBox::DeleteScript,
                };
                *item_form = ItemForm {
                    id: Some(item.id),
                    name: Some(item.name),
                    ..Default::default()
                };
                Task::none()
            },
            GroupMessage::UpdateItemFavourite(id, is_favourite) => {
                let pool = match self.pool.clone() {
                    Some(pool) => pool,
                    None => return Task::none(),
                };
                Task::perform(
                    update_item_favourite(pool, id, is_favourite),
                    |_| GroupMessage::LoadGroup,
                )
            },
            GroupMessage::UpdateItemUsed(id) => {
                let pool = match self.pool.clone() {
                    Some(pool) => pool,
                    None => return Task::none(),
                };
                Task::perform(
                    update_item_used(pool, id),
                    |_| GroupMessage::None,
                )
            },
            GroupMessage::None => Task::none(),
        }
    }

    fn item_row_to_item_info(items: Vec<(ItemRow, Vec<String>)>) -> Vec<ItemInfo> {
        items.into_iter().map(|(item, tags)| ItemInfo {
            id: item.id,
            item_type: match item.item_type.as_str() {
                "command" => ItemType::Command,
                _ => ItemType::Script,
            },
            name: item.name,
            content: item.content,
            description: item.description,
            is_favourite: match item.is_favourite {
                0 => false,
                _ => true,
            },
            last_used_at: item.last_used_at.as_ref().and_then(|s| {
                NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok()
            }),
            tags: tags,
        })
        .collect()
    }
}
