use iced::Element;
use iced::widget::{column, row};

mod navbar;
mod group_navigator;
mod current_group;
mod current_script;


#[derive(Debug, PartialEq)]
pub struct Dashboard {
    pub app_name: &'static str,
    // navbar
    pub search_query: String,
    pub search_results: Vec<String>,
    // group navigator
    pub group_navigator_open: bool,
    pub group_list: Vec<GroupInfo>,
    pub current_group: GroupInfo,
    //current group
    pub command_list: Vec<CommandInfo>,
}

#[derive(Debug, PartialEq)]
pub struct CommandInfo {
    pub command_id: String,
    pub command_name: String,
    pub command_description: String,
}

#[derive(Debug, PartialEq)]
pub struct GroupInfo {
    pub group_id: String,
    pub group_name: String,
    pub item_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DashboardMessage {
    GotoHome,
    GotoSettings,
    ToggleTheme,
    ToggleGroupNavigatorOpen,
    SearchItem(String),
}


impl Dashboard {
    pub fn new(app_name: &'static str) -> Self {
        Self {
            app_name: app_name,
            // navbar
            search_query: String::new(),
            search_results: Vec::<String>::new(),
            // group navigator
            group_navigator_open: true,
            group_list: Vec::from([
                GroupInfo {
                    group_id: "favorite".to_string(),
                    group_name: "Favorite".to_string(),
                    item_count: 0,
                },
                GroupInfo {
                    group_id: "recent".to_string(),
                    group_name: "Recent".to_string(),
                    item_count: 2,
                },
                GroupInfo {
                    group_id: "cus2346245723tom".to_string(),
                    group_name: "Custom 1".to_string(),
                    item_count: 0,
                },
                GroupInfo {
                    group_id: "custo2342662m2".to_string(),
                    group_name: "Custom 2".to_string(),
                    item_count: 5,
                },
            ]),
            current_group: GroupInfo {
                group_id: "tempGroupId".to_string(),
                group_name: "tempGroupName".to_string(),
                item_count: 0,
            },
            // current group
            command_list: Vec::from([
                CommandInfo {
                    command_id: "tempCommandId".to_string(),
                    command_name: "tempCommandName".to_string(),
                    command_description: "tempCommandDescription".to_string(),
                },
                CommandInfo {
                    command_id: "tempCommandId".to_string(),
                    command_name: "tempCommandName".to_string(),
                    command_description: "tempCommandDescription".to_string(),
                }
            ]),
        }
    }

    pub fn view(&self) -> Element<'_, DashboardMessage> {
        column![
            self.navbar_view(),
            row![
                self.group_navigator_view(),
                self.current_group_view(),
                self.current_script_view(),
            ]
        ]
        .into()
    }

    pub fn toggle_group_navigator_open(&mut self) {
        self.group_navigator_open = !self.group_navigator_open;
    }

    pub fn search_item(&mut self, item_name: String) {
        self.search_query = item_name;

        // TODO: Search in database
        self.search_results = Vec::from(["temp".to_string()]);
    }
}
