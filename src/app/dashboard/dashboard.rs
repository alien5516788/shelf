use std::sync::Arc;
use iced::{Element, Task, clipboard};
use iced::widget::{column, row, stack, text_editor::Content};
use sqlx::SqlitePool;
use sqlx::types::chrono::NaiveDateTime;

use super::{Dashboard, GroupInfo, Dialog, ItemInfo, ItemType, ItemFilter, DashboardMessage};
use crate::app::dashboard::item_editor::ItemEditor;
use crate::app::dashboard::search_result::SearchResult;
use crate::components::status_bar::{StatusMessage, StatusType, status_bar_view};
use crate::data::settings::{AppScreen, AppTheme};
use crate::services::item::{ItemRow, create_item, delete_item, load_items_for_group, update_item, update_item_favourite, update_item_used};
use crate::services::group::{GroupRow, create_group, delete_group, load_groups, update_group};
use crate::services::search::{SearchRow, search_items};
use crate::utils::formatting::clamp_name;
use crate::utils::logger::log_error;


impl Dashboard {
    pub fn new(pool: Arc<SqlitePool>) -> (Self, Task<DashboardMessage>) {
        (
            Self {
                group_list: Vec::new(),
                group_selected: GroupInfo::default(),
                item_filter: ItemFilter {
                    command: true,
                    script: true,
                    alphabetical: true,
                },
                item_list: Vec::new(),
                item_editor: None,
                search_query: String::new(),
                search_result: None,
                group_navigator_open: true,
                group_description_open: false,
                status_message: StatusMessage::default(),
                pool: pool,
            },

            Task::done(DashboardMessage::LoadDashboard),
        )
    }

    pub fn view(&self, title: &'static str, theme: &AppTheme) -> Element<'_, DashboardMessage> {
        stack![
            // Dashboard content
            column![
                // Navigator
                self.navigator_view(title, theme),

                stack![
                    // Group view
                    row![
                        // Group list
                        self.group_navigator_view(),

                        // Group
                        self.group_view(),
                    ],

                    // Search result
                    self.search_result_view(),
                ],

                // Status bar
                status_bar_view(
                    &self.status_message,
                    DashboardMessage::SetStatusMessage(StatusMessage::default()),
                )
            ],

            // Popup to create/edit/delete items
            self.item_editor_view(),
        ]
        .into()
    }

    pub fn update(&mut self, message: DashboardMessage, screen: &mut AppScreen, theme: &mut AppTheme) -> Task<DashboardMessage> {
        match message {
            DashboardMessage::LoadDashboard => {
                Task::perform(
                    load_groups(self.pool.clone()),
                    DashboardMessage::SetGroupList
                )
            },
            DashboardMessage::SetGroupList(groups) => {
                match groups {
                    Ok(groups) => self.group_list = Self::group_row_to_group_info(groups),
                    Err(e) => {
                        log_error(format!("Failed to load groups: {}", e).as_str());
                        self.group_list = Vec::new();
                    },
                }
                Task::done(
                    DashboardMessage::SetGroupSelected(
                        Self::find_selected_group_or_default(&self.group_selected, &self.group_list)
                    )
                )
            },
            DashboardMessage::SetGroupSelected(group) => {
                self.group_selected = group;
                Task::done(DashboardMessage::LoadGroupSelected)
            },
            DashboardMessage::LoadGroupSelected => {
                let pool = self.pool.clone();
                Task::perform(
                    load_items_for_group(pool, self.group_selected.id, self.item_filter.command, self.item_filter.script),
                    DashboardMessage::SetItemList
                )
            },
            DashboardMessage::SetItemList(items) => {
                match items {
                    Ok(items) => self.item_list = Self::item_row_to_item_info(items),
                    Err(e) => {
                        eprintln!("Failed to load items: {}", e);
                        self.item_list = Vec::new();
                    },
                }
                if self.item_filter.alphabetical {
                    self.item_list.sort_by(|a, b| a.name.cmp(&b.name));
                }
                Task::none()
            },
            DashboardMessage::SetFilter(filter) => {
                // ISSUE: No items present when both command and script filters are enabled
                self.item_filter = filter;
                Task::done(DashboardMessage::LoadGroupSelected)
            },
            DashboardMessage::SetScreen(scrn) => {
                *screen = scrn;
                Task::none()
            },
            DashboardMessage::SetTheme => {
                match theme {
                    AppTheme::Light => *theme = AppTheme::Dark,
                    AppTheme::Dark => *theme = AppTheme::Light,
                }
                Task::none()
            },
            DashboardMessage::SetSearchQuery(query) => {
                match query {
                    Some(query) => self.search_query = query,
                    None => {
                        self.search_query.clear();
                        self.search_result = None;
                    }
                }
                Task::none()
            },
            DashboardMessage::Search => {
                let pool = self.pool.clone();
                match self.search_query.as_str() {
                    "" => Task::none(),
                    _ => Task::perform(
                        search_items(pool, self.search_query.clone(), 50),
                        |result| DashboardMessage::SetSearchResult(result)
                    )
                }
            },
            DashboardMessage::SetSearchResult(result) => {
                match result {
                    Ok(result) => self.search_result = Some(Self::search_row_to_search_result(result)),
                    Err(_) => (),
                }
                Task::none()
            },
            DashboardMessage::ClearSearchResult => {
                self.search_result = None;
                Task::none()
            },
            DashboardMessage::ScrollToSearchResult(id, group_id) => {
                let group = match self.find_group(group_id) {
                    Some(group) => group,
                    None => return Task::none()
                };
                Task::done(DashboardMessage::SetGroupSelected(group.clone())) // ISSUE: no clone
                    // ISSUE: Highlight must run after loading the group
                    .chain(Task::done(DashboardMessage::HighlightItem(id)))
                    .chain(Task::done(DashboardMessage::ClearSearchResult))
                    .chain(Task::done(
                        DashboardMessage::SetStatusMessage(
                            StatusMessage::new(Some("Scroll to search result is not implemented".to_string()), StatusType::Warn)
                        )
                    ))
            },
            DashboardMessage::SetGroupNavigatoOpen =>{
                self.group_navigator_open = !self.group_navigator_open;
                Task::none()
            },
            DashboardMessage::SetGroupHovered(id) => {
                let group = match self.find_group(id) {
                    Some(group) => group,
                    None => return Task::none(),
                };
                group.hovered = !group.hovered;
                Task::none()
            },


            DashboardMessage::SetGroupDescriptionOpen => {
                self.group_description_open = !self.group_description_open;
                Task::none()
            },
            DashboardMessage::SetItemFavourite(id) => {
                let pool = self.pool.clone();
                let item = match self.find_item(id) {
                    Some(item) => item,
                    None => return Task::none(),
                };

                item.is_favourite = !item.is_favourite; // ISSUE: This must happen only after successfull result

                Task::perform(
                    update_item_favourite(pool, id, !item.is_favourite),
                    |result| match result {
                        Ok(_) => DashboardMessage::None,
                        Err(_) => DashboardMessage::None,
                    },
                )
            },
            DashboardMessage::SetItemUsed(id) => {
                let pool = self.pool.clone();
                Task::perform(
                    update_item_used(pool, id),
                    |_| DashboardMessage::None, // TODO: Recent group count must be incremented
                )
            },
            DashboardMessage::SetItemHovered(id) => {
                let item = match self.find_item(id) {
                    Some(item) => item,
                    None => return Task::none(),
                };
                item.hovered = !item.hovered;
                item.highlighted = false;
                item.copied = false;
                Task::none()
            },
            DashboardMessage::HighlightItem(id) => {
                let item = match self.find_item(id) {
                    Some(item) => item,
                    None => {
                        println!("No item");
                        return Task::none()
                    },
                };
                item.highlighted = true;
                Task::none()
            },
            DashboardMessage::CopyItemContent(id) => {
                let item = match self.find_item(id) {
                    Some(item) => item,
                    None => return Task::none(),
                };

                item.copied = true; // ISSUE: This must happen after copying

                Task::batch([
                    clipboard::write(item.content.clone()),
                    Task::done(DashboardMessage::SetItemUsed(item.id))
                ])
            },
            DashboardMessage::RunCommand(_command) => {
                Task::done(DashboardMessage::SetStatusMessage(
                    StatusMessage::new(Some(format!("Cannot run command (Feature not implemented)")), StatusType::Warn)
                ))
            },
            DashboardMessage::OpenNewGroupBox => {
                self.item_editor = Some(
                    ItemEditor {
                        name: Some(String::new()),
                        description: Some(Content::new()),
                        dialog: Dialog::NewGroup,
                        ..Default::default()
                    }
                );
                Task::none()
            },
            DashboardMessage::OpenEditGroupBox(group) => {
                self.item_editor = Some(
                    ItemEditor {
                        id: Some(group.id),
                        name: Some(group.name),
                        description: Some(Content::with_text(&group.description)),
                        dialog: Dialog::EditGroup,
                        ..Default::default()
                    }
                );
                Task::none()
            },
            DashboardMessage::OpenDeleteGroupBox(group) => {
                self.item_editor = Some(
                    ItemEditor {
                        id: Some(group.id),
                        name: Some(group.name),
                        dialog: Dialog::DeleteGroup,
                        ..Default::default()
                    }
                );
                Task::none()
            },
            DashboardMessage::OpenNewItemBox(item_type) => {
                match item_type {
                    ItemType::Command => self.item_editor = Some(
                        ItemEditor {
                            content: Some(Content::new()),
                            description: Some(Content::new()),
                            tag: Some(String::new()),
                            tags: Some(Vec::new()),
                            dialog: Dialog::NewCommand,
                            ..Default::default()
                        },
                    ),
                    ItemType::Script => self.item_editor = Some(
                        ItemEditor {
                            name: Some(String::new()),
                            content: Some(Content::new()),
                            description: Some(Content::new()),
                            tag: Some(String::new()),
                            tags: Some(Vec::new()),
                            dialog: Dialog::NewScript,
                            ..Default::default()
                        },
                    ),
                }

                Task::none()
            },
            DashboardMessage::OpenEditItemBox(item) => {
                match item.item_type {
                    ItemType::Command => self.item_editor = Some(
                        ItemEditor {
                            id: Some(item.id),
                            content: Some(Content::with_text(&item.content)),
                            description: Some(Content::with_text(&item.description)),
                            tag: Some(String::new()),
                            tags: Some(item.tags),
                            dialog: Dialog::EditCommand,
                            ..Default::default()
                        }
                    ),
                    ItemType::Script => self.item_editor = Some(
                        ItemEditor {
                            id: Some(item.id),
                            name: Some(item.name.unwrap_or_default()),
                            content: Some(Content::with_text(&item.content)),
                            description: Some(Content::with_text(&item.description)),
                            tag: Some(String::new()),
                            tags: Some(item.tags),
                            dialog: Dialog::EditScript,
                            ..Default::default()
                        }
                    ),
                };
                Task::none()
            },
            DashboardMessage::OpenDeleteItemBox(item) => {
                match item.item_type {
                    ItemType::Command => self.item_editor = Some(
                        ItemEditor {
                            id: Some(item.id),
                            name: Some(item.content), // content is used as name for the delete dialog
                            dialog: Dialog::DeleteCommand,
                            ..Default::default()
                        }
                    ),
                    ItemType::Script => self.item_editor = Some(
                        ItemEditor {
                            id: Some(item.id),
                            name: Some(item.name.unwrap_or_default()),
                            dialog: Dialog::DeleteScript,
                            ..Default::default()
                        }
                    ),
                };
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
                        let group_id = self.group_selected.id;
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
                        let group_id = self.group_selected.id;
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
                                    .chain(Task::done(DashboardMessage::LoadDashboard))
                            },
                            Dialog::NewCommand | Dialog::EditCommand | Dialog::DeleteCommand
                            | Dialog::NewScript | Dialog::EditScript | Dialog::DeleteScript => {
                                Task::done(DashboardMessage::CloseItemEditor)
                                    .chain(Task::done(DashboardMessage::LoadGroupSelected))
                            },
                        }
                    },
                    Err(e) => Task::done(DashboardMessage::SetItemEditorError(e)),
                }
            },
            DashboardMessage::SetStatusMessage(message) => {
                self.status_message = message;
                Task::none()
            },
            DashboardMessage::None => Task::none(),
        }
    }

    fn group_row_to_group_info(groups: Vec<GroupRow>) -> Vec<GroupInfo> {
        groups
            .into_iter()
            .map(|group| GroupInfo {
                id: group.id,
                name: group.name,
                description: group.description,
                item_count: group.item_count as usize,

                hovered: false,
            })
            .collect()
    }

    fn find_selected_group_or_default(group_selected: &GroupInfo, group_list: &Vec<GroupInfo>) -> GroupInfo {
        let mut group_found: GroupInfo = GroupInfo::default();

        for group in group_list {
            if group.id == group_selected.id {
                // Current group found, return it
                group_found =  group.clone();
                break;
            } else if group.name == "Default" {
                // Default group found, store its id as fallback
                group_found = group.clone()
            } else {
                continue;
            }
        }

        group_found
    }

    fn find_group(&mut self, id: i32) -> Option<&mut GroupInfo> {
        self.group_list.iter_mut().find(|group| group.id == id)
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

            hovered: false,
            highlighted: false,
            copied: false,
            description_open: false,
        })
        .collect()
    }

    fn find_item(&mut self, id: i32) -> Option<&mut ItemInfo> {
        self.item_list.iter_mut().find(|i| i.id == id)
    }

    fn search_row_to_search_result(result: Vec<SearchRow>) -> Vec<SearchResult> {
        result.into_iter().map(|item| SearchResult {
            id: item.id,
            name: match item.item_type.as_str() {
                "command" => clamp_name(&item.content, 30),
                _ => match item.name {
                    Some(name) => clamp_name(&name, 30),
                    None => "Unknown".to_string(),
                }
            },
            item_type: item.item_type,
            group_id: item.group_id,
            group_name: clamp_name(&item.group_name, 20)
        })
        .collect()
    }
}
