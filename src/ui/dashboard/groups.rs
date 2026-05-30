use iced::widget::{Column, button, container, row, text};
use iced::Element;

use crate::ui::app::AppMessage;


#[derive(Debug, Clone, PartialEq)]
pub struct Groups {
    group_list: Vec<Group>,
    pub search_query: String,
    pub search_results: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
enum GroupName {
    Favorite,
    Recent,
    Custom(String)
}

#[derive(Debug, Clone, PartialEq)]
struct Group {
    group_id: String,
    group_name: GroupName,
    item_count: usize,
}


impl Groups {
    pub fn new() -> Self {
        let group_list: Vec<Group> = Vec::from([
            Group {
                group_id: "favorite".to_string(),
                group_name: GroupName::Favorite,
                item_count: 0,
            },
            Group {
                group_id: "recent".to_string(),
                group_name: GroupName::Recent,
                item_count: 2,
            },
            Group {
                group_id: "custom".to_string(),
                group_name: GroupName::Custom("Custom".to_string()),
                item_count: 0,
            },
            Group {
                group_id: "custom2".to_string(),
                group_name: GroupName::Custom("Custom".to_string()),
                item_count: 5,
            },
        ]);

        Self {
            group_list: group_list,
            search_query: String::new(),
            search_results: Vec::new(),
        }
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        let groups = self.group_list.iter().fold(
            Column::new(),
            |column, group| column.push(GroupCard::view(group.group_name.clone(), group.item_count)),
        );

        container(groups).into()
    }
}


struct GroupCard;

impl GroupCard {
    fn view(group_name: GroupName, item_count: usize) -> Element<'static, AppMessage> {
        // Convert group name to string
        let group_name = match group_name {
            GroupName::Favorite => "Favorite".to_string(),
            GroupName::Recent => "Recent".to_string(),
            GroupName::Custom(name) => name,
        };

        row![
            button(text(group_name)),
            text(format!("{} items", item_count)),
        ].into()
    }
}
