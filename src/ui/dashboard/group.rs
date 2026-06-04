use iced::widget::{Column, Space, button, column, container, grid, row, stack, text};
use iced::{Alignment, Background, Border, Color, Element, Length};

use crate::icon;
use super::DashboardMessage;


#[derive(Debug, PartialEq)]
pub struct Group {
    pub group_id: String,
    pub group_name: String,
    pub command_list: Vec<CommandInfo>,
}

#[derive(Debug, PartialEq)]
pub struct CommandInfo {
    pub command_id: String,
    pub command_name: String,
    pub command_description: String,
}


impl Group {
    pub fn new() -> Self {
        Self {
            group_id: "tempIdr34t4".to_string(),
            group_name: "tempgroup".to_string(),
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
        container(
            column![
                text(&self.group_name),
                grid!(
                    button("group")
                )
            ]
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .padding(10)
        .style(|_| container::Style {
            border: Border {
                color: Color::from_rgb(0.4, 0.4, 0.4),
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
    }
}






struct CommandCard;

impl CommandCard {
    fn view() -> Element<'static, DashboardMessage> {
        button("").into()
    }
}




struct ScriptCard;

impl ScriptCard {
    fn view() -> Element<'static, DashboardMessage> {
        button("").into()
    }
}
