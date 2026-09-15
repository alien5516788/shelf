use std::sync::Arc;

use iced::font::{Family, Style};
use iced::widget::text::{Span, Wrapping};
use iced::widget::text_editor::Content;
use iced::widget::{Column, Row, button, center, center_x, column, container, mouse_area, rich_text, row, scrollable, space, text};
use iced::{Alignment, Background, Border, Color, Element, Font, Length, Padding, Task, Theme, color};
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
    item_hovered: Option<i32>, // TODO: Move this inside the ItemInfo struct
    item_content_copied: Option<i32>, // TODO: Move this inside the ItemInfo struct

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
    SetItemContentCopied(Option<i32>),
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
                item_content_copied: None,
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
                // ISSUE: Mismathcing colors on light theme
                rich_text![
                    Span::<&str>::new("user@shelf").color(color!(0x50FA7B)),
                    Span::new(":"),
                    Span::new("~/").color(color!(0x276CF5)),
                    Span::new(current_group.name.clone()).color(color!(0x276CF5)),
                    Span::new("$ "),
                ]
                .wrapping(Wrapping::WordOrGlyph),

                // Group controls and filters
                row![
                    // Command filter
                    button(icon::square_terminal().size(20.0))
                    .on_press(GroupMessage::SetFilter(Filter {
                        command: !self.filter.command,
                        ..self.filter
                    }))
                    .style(|theme, _| button::Style {
                        background: None,
                        text_color: theme.palette().primary,
                        ..Default::default()
                    }),

                    // Script filter
                    button(icon::code_xml().size(20.0))
                    .on_press(GroupMessage::SetFilter(Filter {
                        script: !self.filter.script,
                        ..self.filter
                    }))
                    .style(|theme, _| button::Style {
                        background: None,
                        text_color: theme.palette().primary,
                        ..Default::default()
                    }),

                    // Alphabetical filter
                    button(
                        icon::a_large_small()
                            .size(20.0)
                    )
                    .on_press(GroupMessage::SetFilter(Filter {
                        alphabetical: !self.filter.alphabetical,
                        ..self.filter
                    }))
                    .style(|theme, _| button::Style {
                        background: None,
                        text_color: theme.palette().primary,
                        ..Default::default()
                    }),

                    space()
                        .width(Length::Fill),

                    // Edit group, Add command/script
                    if current_group.name.as_str() != "Favourites"
                    && current_group.name.as_str() != "Recent" {
                        row![
                            // Edit group
                            match current_group.name.as_str() != "Default" {
                                true => container(
                                    button(center(
                                        icon::pen().size(15.0)
                                    ))
                                    .on_press(GroupMessage::OpenEditGroupBox(current_group.clone()))
                                    .height(40)
                                    .width(40)
                                    .style(|theme, _| button::Style {
                                        text_color: theme.palette().primary,
                                        ..Default::default()
                                    })
                                ),
                                false => container(space()),
                            },

                            space()
                                .width(Length::Fixed(20.0)),

                            // Add new command
                            button(
                                row![
                                    icon::plus()
                                        .size(20.0)
                                        .style(|theme| text::Style {
                                            color: Some(theme.palette().success),
                                            ..Default::default()
                                        }),

                                    text("Command")
                                        .style(|theme: &Theme| text::Style {
                                            color: Some(theme.palette().success),
                                            ..Default::default()
                                        }),
                                ]
                                .align_y(Alignment::Center)
                                .spacing(10)
                            )
                            .on_press(GroupMessage::OpenNewItemBox(ItemType::Command))
                            .style(|theme, _| button::Style {
                                border: Border {
                                    color: theme.palette().success,
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
                                        .style(|theme| text::Style {
                                            color: Some(theme.palette().success),
                                            ..Default::default()
                                        }),

                                    text("Script")
                                        .style(|theme: &Theme| text::Style {
                                            color: Some(theme.palette().success),
                                            ..Default::default()
                                        }),
                                ]
                                .align_y(Alignment::Center)
                                .spacing(10)
                            )
                            .on_press(GroupMessage::OpenNewItemBox(ItemType::Script))
                            .style(|theme, _| button::Style {
                                border: Border {
                                    color: theme.palette().success,
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

                // Group description
                row![
                    // Description
                    container(
                        row![
                            // Text
                            container(match current_group.description.as_str() {
                                "" => text("No description")
                                    .size(16)
                                    .font(Font {
                                        family: Family::Monospace,
                                        style: Style::Italic, // ISSUE: Italic not working
                                        ..Default::default()
                                    })
                                    .style(|theme: &Theme| text::Style {
                                        color: Some(theme.palette().primary.scale_alpha(0.5)),
                                    }),
                                description => text(description.to_string())
                                    .size(16)
                                    .style(|theme: &Theme| text::Style {
                                        color: Some(theme.palette().primary),
                                    }),
                            })
                            .width(Length::Fill),

                            // Collapse button
                            match current_group.description.as_str() {
                                "" => container(space()),
                                _ => container(button(match self.description_collapsed {
                                        true => icon::chevron_down().size(20.0),
                                        false => icon::chevron_up().size(20.0),
                                    })
                                    .on_press(GroupMessage::ToggleDescriptionCollapsed)
                                    .style(|theme, _| button::Style {
                                        text_color: theme.palette().primary,
                                        ..Default::default()
                                    })),
                            }
                        ]
                    )
                    .height(match self.description_collapsed { // ISSUE: Collapsed height covers half the height of second line
                        true => Length::Fixed(20.0 * 2.5),
                        false => Length::Shrink,
                    })
                    .padding(10)
                    .clip(true)
                    .style(|theme| container::Style {
                        background: Some(Background::Color(theme.palette().primary.scale_alpha(0.1))),
                        ..Default::default()
                    }),
                ],

                // Item list
                scrollable(
                    column![
                        center_x(self.item_list
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
                color: theme.palette().primary,
                width: 0.5,
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
    }

    fn item_card_view<'a>(&self, item: &'a ItemInfo) -> Element<'a, GroupMessage> {
        let hovered = self.item_hovered == Some(item.id);

        let content_copied = self.item_content_copied == Some(item.id);

        let (id, icon, name, content, description, tags, is_favourite) = match item.item_type {
            ItemType::Command => (
                item.id,
                match content_copied {
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
                match content_copied {
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

        fn tag_view(tag: &str) -> Element<'_, GroupMessage> {
            container(
                text(tag)
                    .size(13)
                    .color(color!(0xF8F8F2))
            )
            .padding([2, 5])
            .style(|theme: &Theme| container::Style {
                background: Some(Background::Color(theme.palette().primary.scale_alpha(0.6))),
                border: Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
        }

        mouse_area(
            container(
                row![
                    // Icon, Content, Description and Tags
                    // Clicking anywhere copies the item content
                    button(
                        column![
                            // Icon, Name
                            row![
                                // Icon
                                icon
                                    .size(20.0)
                                    .color(Color::from_rgb(0.3, 0.4, 0.6)),

                                // Name of script / Content of command
                                text(name)
                                    .wrapping(Wrapping::WordOrGlyph)
                                    .size(15)
                                    .style(|theme: &Theme| text::Style {
                                        color: Some(theme.palette().text),
                                        ..Default::default()
                                    }),

                                space().width(Length::Fill),
                            ]
                            .width(Length::Fill)
                            .spacing(8)
                            .align_y(Alignment::Center),

                            // Content
                            // Only available to scripts
                            container(
                                text(match content {
                                    Some(content) => content,
                                    None => "No content",
                                })
                                .size(15)
                                .color(Color::from_rgb(0.5, 0.5, 0.6))
                            )
                            .padding(5),

                            // Description
                            container(match description.as_str() {
                                "" => text("No description")
                                    .size(15)
                                    .style(|theme: &Theme| text::Style {
                                        color: Some(theme.palette().primary.scale_alpha(0.5)),
                                        ..Default::default()
                                    }),
                                _ => text(description)
                                    .size(15)
                                    .style(|theme: &Theme| text::Style {
                                        color: Some(theme.palette().primary),
                                        ..Default::default()
                                    }),
                            })
                            .padding(5),

                            // Tags
                            tags
                                .iter()
                                .fold(
                                    Row::new().padding(5).spacing(6),
                                    |row, tag| row.push(tag_view(tag)),
                                ),
                        ]
                        .spacing(8)
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
                    .style(|_, _| button::Style {
                        ..Default::default()
                    }),

                    // Favourite, Edit, Delete buttons
                    column![
                        // Run
                        button(icon::play().size(15))
                            .style(|theme, _| button::Style {
                                text_color: theme.palette().success,
                                ..Default::default()
                            }),

                        // Favourite
                        match is_favourite {
                            // If the item is a favourite, show the star icon
                            true => container(
                                button(icon::star().size(16))
                                    .on_press(GroupMessage::UpdateItemFavourite(id, false))
                                    .style(|theme, _| button::Style {
                                        text_color: theme.palette().warning,
                                        ..Default::default()
                                    })
                            ),
                            // If the item is not a favourite, hide the star icon but show the empty star icon on hover
                            false => match hovered {
                                true => container(
                                    button(icon::star().size(16))
                                        .on_press(GroupMessage::UpdateItemFavourite(id, true))
                                        .style(|theme, _| button::Style {
                                            text_color: theme.palette().primary,
                                            ..Default::default()
                                        })
                                ),
                                false => container(space()),
                            },
                        },

                        // Edit, Delete
                        // Show the edit, delete buttons on hover
                        match hovered {
                            true => column![
                                button(icon::pen().size(15) )
                                    .on_press(GroupMessage::OpenEditItemBox(item.clone()))
                                    .style(|theme, _| button::Style {
                                        text_color: theme.palette().primary,
                                        ..Default::default()
                                    }),

                                button(icon::trash().size(15))
                                    .on_press(GroupMessage::OpenDeleteItemBox(item.clone()))
                                    .style(|theme, _| button::Style {
                                        text_color: theme.palette().danger,
                                        ..Default::default()
                                    })
                            ],
                            false => column![],
                        }
                        .height(Length::Shrink)
                        .width(35)
                        .spacing(10),
                    ]
                    .height(150) // ISSUE: Approximate height that fits visible icon set
                    .spacing(10)
                ]
            )
            .width(800)
            .height(Length::Shrink)
            .padding(12)
            .style(|theme| container::Style {
                border: Border {
                    color: theme.palette().primary,
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
                Task::done(GroupMessage::SetItemContentCopied(None))
            },
            GroupMessage::SetItemContentCopied(copied) => {
                self.item_content_copied = copied;
                Task::none()
            },
            GroupMessage::CopyItemContent(id, content) => {
                Task::batch([
                    clipboard::write(content),
                    Task::done(GroupMessage::UpdateItemUsed(id))
                        .chain(Task::done(GroupMessage::SetItemContentCopied(Some(id)))),
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
