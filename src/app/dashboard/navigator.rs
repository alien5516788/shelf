use std::sync::Arc;

use iced::{Alignment, Background, Border, Element, Length, Padding, Task, Theme, color};
use iced::widget::{button, center_x, container, row, space, text, text_input};
use sqlx::SqlitePool;

use crate::data::settings::{AppScreen, AppTheme};
use crate::icon;
use crate::services::search::{SearchRow, search_items};
use crate::utils::font_size::{sv, sv_16, sv_20};
use crate::utils::formatting::clamp_name;

#[derive(Debug, Clone)]
pub struct Navigator {
    pub search_query: String,

    pub pool: Option<Arc<SqlitePool>>,
}

#[derive(Debug, Clone)]
pub struct SearchInfo {
    pub id: i32,
    pub name: String,
    pub item_type: String,
    pub group_id: i32,
    pub group_name: String,
}

#[derive(Debug, Clone)]
pub enum NavigatorMessage {
    SetPool(Arc<SqlitePool>),
    SetScreen(AppScreen),
    ToggleTheme,
    SetSearchQuery(Option<String>),
    SearchQuery,
    SetSearchResult(Result<Vec<SearchRow>, String>),
}

impl Navigator {
    pub fn new(pool: Arc<SqlitePool>) -> (Self, Task<NavigatorMessage>) {
        (
            Self {
                search_query: String::new(),
                pool: None,
            },

            Task::done(NavigatorMessage::SetPool(pool))
        )
    }

    pub fn view(&self, title: String, theme: &AppTheme) -> Element<'_, NavigatorMessage> {
        container(
            row![
                // App name
                button(
                    text(title)
                        .size(sv(18.0)))
                        .style(|theme: &Theme, _| {
                            button::Style {
                                border: Border::default(),
                                text_color: theme.palette().text,
                                ..Default::default()
                            }
                        }
                )
                .padding([0, 15])
                .on_press(NavigatorMessage::SetScreen(AppScreen::Home)),

                // Search bar
                center_x(
                    container(
                        row![
                            // Search button
                            button(icon::search().size(sv(15.0)))
                                .on_press(NavigatorMessage::SearchQuery)
                                .style(|theme, _| button::Style {
                                    text_color: theme.palette().primary,
                                    ..Default::default()
                                }),

                            // Input
                            text_input("Search", &self.search_query)
                                .on_input(|query| NavigatorMessage::SetSearchQuery(Some(query)))
                                .size(sv_16())
                                .style(|theme: &Theme, _| text_input::Style {
                                    background: Background::Color(theme.palette().background).scale_alpha(0.0),
                                    border: Border {
                                        ..Default::default()
                                    },
                                    icon: theme.palette().primary,
                                    placeholder: theme.palette().primary,
                                    value: theme.palette().text,
                                    selection: theme.palette().primary.scale_alpha(0.3),
                                }),

                            // Clear button
                            match self.search_query.as_str() {
                                "" => container(space()),
                                _ => container(button(icon::x().size(sv(15.0)))
                                    .on_press(NavigatorMessage::SetSearchQuery(None))
                                    .style(|theme, _| button::Style {
                                        text_color: theme.palette().primary,
                                        ..Default::default()
                                    })),
                            }
                        ]
                        .align_y(Alignment::Center)
                    )
                    .width(sv(600.0))
                    .padding(2)
                    .align_y(Alignment::Center)
                    .style(|theme: &Theme| container::Style {
                        border: Border {
                            color: theme.palette().primary,
                            width: 1.0,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                )
                .width(Length::Fill)
                .padding(3),

                // Shortcuts
                row![
                    // Settings
                    button(icon::settings().size(sv_20()))
                        .style(|theme, _| button::Style {
                            text_color: theme.palette().primary,
                            ..Default::default()
                        })
                        .on_press(NavigatorMessage::SetScreen(AppScreen::Settings)),

                    // Switch theme
                    button(match theme {
                        AppTheme::Dark => icon::moon()
                            .size(sv_20())
                            .color(color!(0xF1FA8C)),
                        AppTheme::Light => icon::sun()
                            .size(sv_20())
                            .color(color!(0xE77B00)),
                    })
                    .style(|_, _| button::Style {
                        ..Default::default()
                    })
                    .on_press(NavigatorMessage::ToggleTheme),
                ]
                .spacing(10),
            ]
            .align_y(Alignment::Center)
            .spacing(20)
        )
        .padding(Padding {
            top: 5.0,
            right: 20.0,
            bottom: 5.0,
            left: 20.0
        })
        .style(|theme| container::Style {
            border: Border {
                color: theme.palette().primary,
                width: 0.5,
                ..Default::default()
            },
            ..Default::default()
        })
        .width(Length::Fill)
        .into()
    }

    pub fn update(&mut self, message: NavigatorMessage, screen: &mut AppScreen, theme: &mut AppTheme, search_result: &mut Option<Vec<SearchInfo>>) -> Task<NavigatorMessage> {
        match message {
            NavigatorMessage::SetPool(pool) => {
                self.pool = Some(pool);
                Task::none()
            },
            NavigatorMessage::SetScreen(scrn) => {
                *screen = scrn;
                Task::none()
            },
            NavigatorMessage::ToggleTheme => match theme {
                AppTheme::Light => {
                    *theme = AppTheme::Dark;
                    Task::none()
                },
                AppTheme::Dark => {
                    *theme = AppTheme::Light;
                    Task::none()
                },
            },
            NavigatorMessage::SetSearchQuery(query) => {
                match query {
                    Some(query) => self.search_query = query,
                    None => {
                        self.search_query.clear();
                        *search_result = None;
                    }
                }
                Task::none()
            },
            NavigatorMessage::SearchQuery => {
                match self.search_query.as_str() {
                    "" => Task::none(),
                    _ => match self.pool.clone() {
                        Some(pool) => Task::perform(
                            search_items(pool, self.search_query.clone(), 50),
                            |result| NavigatorMessage::SetSearchResult(result)
                        ),
                        None => Task::none(),
                    }
                }
            },
            NavigatorMessage::SetSearchResult(result) => {
                match result {
                    Ok(result) => *search_result = Some(Self::search_row_to_search_info(result)),
                    Err(_) => (),
                }
                Task::none()
            }
        }
    }

    fn search_row_to_search_info(result: Vec<SearchRow>) -> Vec<SearchInfo> {
        result.into_iter().map(|item| SearchInfo {
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
