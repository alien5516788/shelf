use iced::{Alignment, Background, Border, Color, Element, Length, Theme};
use iced::widget::{Column, Space, button, column, container, grid, row, text, text_input};

use crate::icon;

#[derive(Debug, PartialEq)]
pub struct Dashboard {
    app_name: &'static str,

    pub search_query: String,
    pub search_results: Vec<String>,

    group_list: Vec<GroupInfo>,
    current_group: GroupInfo,

    pub command_list: Vec<CommandInfo>,

}

#[derive(Debug, PartialEq)]
pub struct CommandInfo {
    pub command_id: String,
    pub command_name: String,
    pub command_description: String,
}



#[derive(Debug, PartialEq)]
struct GroupInfo {
    group_id: String,
    group_name: String,
    item_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DashboardMessage {
    GotoHome,
    GotoSettings,
    ToggleDarkMode,
    SearchChanged(String),
}


impl Dashboard {
    pub fn new(app_name: &'static str) -> Self {
        Self {
            app_name: app_name,

            search_query: String::new(),
            search_results: Vec::<String>::new(),

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

    fn navbar_view(&self) -> Element<'_, DashboardMessage> {
        container(
            row![
                // App name
                button(text(self.app_name).size(24))
                    .style(|theme: &Theme, status: button::Status| {
                        button::Style {
                            background: None,
                            border: Border::default(),
                            text_color: match status {
                                button::Status::Hovered => Color::from_rgb(0.3, 0.6, 1.0),
                                _ => theme.palette().text,
                            },
                            ..Default::default()
                        }
                    })
                    .padding(0)
                    .on_press(DashboardMessage::GotoHome),

                // Search bar
                container(
                    text_input("Search...", self.search_query.as_str())
                        .on_input(|s| DashboardMessage::SearchChanged(s))
                )
                .width(Length::Fill)
                .align_y(Alignment::Center)
                .padding(5),

                // Shortcuts
                row![
                    button("⚙ Settings")
                        .on_press(DashboardMessage::GotoSettings),
                    button("☀ Dark Mode")
                        .on_press(DashboardMessage::ToggleDarkMode),
                ]
                .spacing(10),
            ]
            .align_y(Alignment::Center)
            .spacing(20)
            .padding(10)
        )
        .style(|theme: &Theme| container::Style {
            border: Border {
                color: Color::from_rgb(0.4, 0.4, 0.45),
                width: 1.0,
                radius: 0.0.into(),
            },
            background: Some(iced::Background::Color(theme.palette().background)),
            ..Default::default()
        })
        .height(70)
        .width(Length::Fill)
        .into()
    }

    fn group_navigator_view(&self) -> Element<'_, DashboardMessage> {
        fn group_card_view(group_info: &GroupInfo) -> Element<'_, DashboardMessage> {
            button(
                row![
                    // Card icon
                    group_icon_view(&group_info.group_name),
                    Space::new()
                        .width(Length::Fixed(10.0)),

                    // Card title
                    text(&group_info.group_name)
                        .style(|_| text::Style {
                            color: Some(Color::from_rgb(1.0, 1.0, 1.0)),
                            ..Default::default()
                        }),
                    Space::new()
                        .width(Length::Fill),

                    // Card item count
                    text(&group_info.item_count)
                        .style(|_| text::Style {
                            color: Some(Color::from_rgb(1.0, 1.0, 1.0)),
                            ..Default::default()
                        }),
                ]
            )
            .on_press(DashboardMessage::ToggleDarkMode)
            .padding(8)
            .width(220)
            .style(|_, status| button::Style {
                background: match status {
                    button::Status::Hovered => Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.3))),
                    _ => None,
                },
                ..Default::default()
            })
            .into()
        }

        fn group_icon_view(group_name: &str) -> Element<'static, DashboardMessage> {
            let icon = match group_name {
                "Favorite" => icon::star()
                    .size(20.0)
                    .color(Color::from_rgb(0.5, 0.9, 0.9)),
                "Recent" => icon::history()
                    .size(20.0)
                    .color(Color::from_rgb(0.5, 0.9, 0.9)),
                _ => icon::group_box()
                    .size(20.0)
                    .color(Color::from_rgb(1.0, 0.7, 0.4)),
            };

            icon.into()
        }


        container(
            column![
                // Collapse side bar
                button(
                    icon::menu()
                        .size(20.0)
                        .color(Color::from_rgb(0.3, 0.4, 0.6))
                )
                .style(|_, _| button::Style {
                    background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.0))),
                    ..Default::default()
                }),

                Space::new()
                    .height(Length::Fixed(20.0)),

                // Add a new group
                button(
                    container(
                        row![
                            icon::plus()
                                .color(Color::from_rgb(0.3, 0.9, 0.4)),
                            text("New Group")
                                .color(Color::from_rgb(0.3, 0.9, 0.4))
                        ]
                        .spacing(10)
                    )
                    .align_x(Alignment::Center)
                )
                .padding(8)
                .width(220)
                .style(|_, _| button::Style {
                    background: None,
                    border: Border {
                        color: Color::from_rgb(0.3, 0.9, 0.4),
                        width: 2.0,
                        radius: 4.into(),
                    },
                    ..Default::default()
                }),

                Space::new()
                    .height(Length::Fixed(20.0)),

                // Group list
                self.group_list.iter().fold(
                    Column::new().spacing(10),
                    |column, group| column.push(group_card_view(&group)),
                ),
            ]
        )
        .height(Length::Fill)
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

    fn current_group_view(&self) -> Element<'_, DashboardMessage> {
        fn command_card_view() -> Element<'static, DashboardMessage> {
            button("command").into()
        }

        fn script_card_view() -> Element<'static, DashboardMessage> {
            button("script").into()
        }

        container(
            column![
                // Group name
                text(&self.current_group.group_name),

                // Items
                grid!(
                    command_card_view(),
                    script_card_view(),
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

    pub fn current_script_view(&self) -> Element<'static, DashboardMessage> {
        container(
            grid!(button("scripts"))
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

impl Dashboard {
    pub fn search(&self) -> Vec<String> {
        // TODO: Search in database
        Vec::from(["temp".to_string()])
    }
}
