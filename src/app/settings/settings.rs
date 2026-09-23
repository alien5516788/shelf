use iced::widget::{center_x, column, container, scrollable};
use iced::{Border, Element, Length, Task, Theme};

use super::{Settings, SettingsMessage};
use crate::components::status_bar::{StatusMessage, StatusType, status_bar_view};
use crate::data::ensure_data_dir;
use crate::data::settings::{AppScreen, AppTheme, SettingsInfo, get_settings, load_settings, set_settings};
use crate::utils::logger::log_error;


impl Settings {
    pub fn new() -> (Self, Task<SettingsMessage>) {
        (
            Self {
                setting_list: SettingsInfo::default(),
                data_dir: String::new(),
                version: String::new(),
                status_message: StatusMessage::default(),
            },

            Task::done(SettingsMessage::LoadSettings),
        )
    }

    pub fn view(&self) -> Element<'_, SettingsMessage> {
        container(
            column![
                // Navigator
                self.navigator_view(), 

                // Setting list
                container(
                    center_x(
                        scrollable(
                            self.setting_list_view()
                        )
                    )
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|theme: &Theme| container::Style {
                    border: Border {
                        color: theme.palette().primary,
                        width: 0.5,
                        ..Default::default()
                    },
                    ..Default::default()
                }),

                // Status bar
                status_bar_view(
                    &self.status_message,
                    SettingsMessage::SetStatusMessage(StatusMessage::default()),
                ),
            ]
        )
        .style(|theme: &Theme| container::Style {
            border: Border {
                color: theme.palette().primary,
                width: 0.5,
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
    }

    pub fn update(&mut self, message: SettingsMessage, screen: &mut AppScreen, theme: &mut AppTheme) -> Task<SettingsMessage> {
        match message {
            SettingsMessage::LoadSettings => {
                match load_settings() {
                    Ok(_) => match get_settings() {
                        Ok(settings) => {
                            *theme = settings.theme.clone();
                            *screen = settings.screen.clone();
                            self.setting_list = settings;
                            self.data_dir = match ensure_data_dir() {
                                Ok(path) => path.to_string_lossy().into_owned(),
                                Err(e) => {
                                    log_error(&e);
                                    String::new()
                                }
                            };
                            self.version = "Version 0.1.0".to_string();
                            Task::none()
                        },
                        Err(e) => {
                            log_error(&e);
                            Task::none()
                        },
                    },
                    Err(e) => {
                        log_error(&e);
                        Task::none()
                    },
                }
            },
            SettingsMessage::SetTheme(thm) => {
                *theme = thm;
                Task::none()
            },
            SettingsMessage::SetScreen(scrn) => {
                *screen = scrn;
                Task::none()
            },
            SettingsMessage::OpenDataDir => {
                let handle = std::process::Command::new("xdg-open") // ISSUE: Implement executor
                    .arg(&self.data_dir)
                    .spawn();

                match handle {
                    Ok(_) => Task::none(),
                    Err(e) => {
                        log_error(&e.to_string());
                        Task::done(SettingsMessage::SetStatusMessage(StatusMessage::new(Some(e.to_string()), StatusType::Error)))
                    }
                }
            },
            SettingsMessage::SetStatusMessage(m) => {
                self.status_message = m;
                Task::none()
            }
            SettingsMessage::SetThemeSetting(thm) => {
                match set_settings(|s| s.theme = thm.clone()) {
                    Ok(_) => self.setting_list.theme = thm.clone(),
                    Err(e) => log_error(&e),
                };
                Task::done(SettingsMessage::SetTheme(thm))
            },
            SettingsMessage::SetScreenSetting(screen) => {
                match set_settings(|s| s.screen = screen.clone()) {
                    Ok(_) => self.setting_list.screen = screen,
                    Err(e) => log_error(&e),
                };
                Task::none()
            }
            SettingsMessage::SetTextSize(size) => {
                match set_settings(|s| s.text_size = size.clone()) {
                    Ok(_) => self.setting_list.text_size = size,
                    Err(e) => log_error(&e),
                };
                Task::none()
            },
            SettingsMessage::SetShell(shell) => {
                match set_settings(|s| s.shell = shell.clone()) {
                    Ok(_) => self.setting_list.shell = shell,
                    Err(e) => log_error(&e),
                };
                Task::none()
            },
            SettingsMessage::SetTerminal(terminal) => {
                match set_settings(|s| s.terminal = terminal.clone()) {
                    Ok(_) => self.setting_list.terminal = terminal,
                    Err(e) => log_error(&e),
                };
                Task::none()
            },
            SettingsMessage::SetRunMode(mode) => {
                match set_settings(|s| s.run_mode = mode.clone()) {
                    Ok(_) => self.setting_list.run_mode = mode,
                    Err(e) => log_error(&e),
                };
                Task::none()
            },
            SettingsMessage::ToggleKeepOpen(open) => {
                match set_settings(|s| s.keep_open = open.clone()) {
                    Ok(_) => self.setting_list.keep_open = open,
                    Err(e) => log_error(&e),
                };
                Task::none()
            },
        }
    }
}
