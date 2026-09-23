mod dashboard;
mod navigator;
mod group_navigator;
mod group;
mod item_editor;
mod search_result;

use std::sync::Arc;
use iced::widget::text_editor::Action;
use sqlx::SqlitePool;
use sqlx::types::chrono::NaiveDateTime;

use item_editor::{ItemEditor, Dialog};
use crate::app::dashboard::search_result::SearchResult;
use crate::components::status_bar::StatusMessage;
use crate::data::settings::AppScreen;
use crate::services::{group::GroupRow, item::ItemRow, search::SearchRow};


#[derive(Debug, Clone)]
pub struct Dashboard {
    pub group_list: Vec<GroupInfo>,
    pub group_selected: GroupInfo,
    pub item_filter: ItemFilter,
    pub item_list: Vec<ItemInfo>,
    pub item_editor: Option<ItemEditor>,

    pub search_query: String,
    pub search_result: Option<Vec<SearchResult>>,

    pub group_navigator_open: bool,
    pub group_description_open: bool,

    pub status_message: StatusMessage,

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

#[derive(Debug, Clone)]
pub struct ItemFilter {
    command: bool,
    script: bool,
    alphabetical: bool,
}

#[derive(Debug, Clone)]
pub struct ItemInfo {
    pub id: i32,
    pub item_type: ItemType,
    pub name: Option<String>,
    pub content: String,
    pub description: String,
    pub is_favourite: bool,
    pub last_used_at: Option<NaiveDateTime>,
    pub tags: Vec<String>,

    pub hovered: bool,
    pub highlighted: bool,
    pub copied: bool,
    pub description_open: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Command,
    Script,
}

#[derive(Debug, Clone)]
pub enum DashboardMessage {
    LoadDashboard,
    SetGroupList(Result<Vec<GroupRow>, String>),
    SetGroupSelected(GroupInfo),
    LoadGroupSelected,
    SetItemList(Result<Vec<(ItemRow, Vec<String>)>, String>),
    SetFilter(ItemFilter),

    SetScreen(AppScreen),
    SetTheme,
    SetSearchQuery(Option<String>),
    Search,
    SetSearchResult(Result<Vec<SearchRow>, String>),
    ClearSearchResult,
    ScrollToSearchResult(i32, i32),

    SetGroupNavigatoOpen,
    SetGroupHovered(i32),

    SetGroupDescriptionOpen,
    SetItemFavourite(i32),
    SetItemUsed(i32),
    SetItemHovered(i32),
    HighlightItem(i32),
    CopyItemContent(i32),
    RunCommand(String),

    OpenNewGroupBox,
    OpenEditGroupBox(GroupInfo),
    OpenDeleteGroupBox(GroupInfo),
    OpenNewItemBox(ItemType),
    OpenEditItemBox(ItemInfo),
    OpenDeleteItemBox(ItemInfo),
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

    SetStatusMessage(StatusMessage),

    None,
}
