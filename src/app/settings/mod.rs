use iced::widget::{button, center_x, column, container, row, scrollable, space, text, toggler};
use iced::{Alignment, Border, Element, Length, Task, Theme};

use crate::components::status_bar::{StatusMessage, StatusType, status_bar_view};
use crate::data::ensure_data_dir;
use crate::data::settings::{AppScreen, AppTheme, RunMode, SettingsInfo, ShellKind, TerminalKind, TextSize, get_settings, load_settings, set_settings};
use crate::icon;
use crate::utils::font_size::sv;
use crate::utils::logger::log_error;


#[derive(Debug)]
pub struct Settings {
    setting_list: SettingsInfo,
    data_dir: String,
    version: String,

    status_message: StatusMessage,
}


#[derive(Debug, Clone)]
pub enum SettingsMessage {
    LoadSettings,
    SetTheme(AppTheme),
    SetScreen(AppScreen),
    OpenDataDir,
    SetStatusMessage(StatusMessage),

    SetThemeSetting(AppTheme),
    SetScreenSetting(AppScreen),
    SetTextSize(TextSize),
    SetShell(ShellKind),
    SetTerminal(TerminalKind),
    SetRunMode(RunMode),
    ToggleKeepOpen(bool),
}

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
                container(
                    row![
                        // Back button
                        button(icon::chevron_left().size(sv(20.0)))
                            .padding(2.5)
                            .style(|theme: &Theme, _| {
                                button::Style {
                                    text_color: theme.palette().text,
                                    ..Default::default()
                                }
                            })
                            .on_press(SettingsMessage::SetScreen(AppScreen::Dashboard)),

                        // Title
                        text("Settings")
                            .size(sv(18.0))
                            .style(|theme: &Theme| {
                                text::Style {
                                    color: Some(theme.palette().text),
                                    ..Default::default()
                                }
                            }),
                    ]
                    .padding(10)
                    .spacing(20)
                    .align_y(Alignment::Center),
                )
                .padding([0, 20])
                .style(|theme: &Theme| container::Style {
                    border: Border {
                        color: theme.palette().primary,
                        width: 0.5,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .width(Length::Fill),

                // Settings
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

    fn setting_list_view(&self) -> Element<'_, SettingsMessage> {
        fn section<'a: 'static>(title: &'a str, content: Element<'a, SettingsMessage>) -> Element<'static, SettingsMessage> {
            container(
                column![
                    text(title).size(sv(18.0)),
                    space().height(8),
                    content,
                ]
                .spacing(6),
            )
            .padding(16)
            .width(Length::Fill)
            .style(|theme: &Theme| container::Style {
                border: Border {
                    color: theme.palette().primary.scale_alpha(0.5),
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..Default::default()
            })
            .into()
        }

        fn setting_row<'a: 'static>(label: &'a str, setting: Element<'a, SettingsMessage>) -> Element<'static, SettingsMessage> {
            row![
                text(label)
                    .size(sv(15.0))
                    .width(Length::Fixed(160.0)),
                setting,
            ]
            .spacing(12)
            .align_y(Alignment::Center)
            .into()
        }

        container(
            column![
                // Appearance
                section(
                    "Appearance",

                    column![
                        setting_row(
                            "Theme",
                            row![
                                choice("Light", self.setting_list.theme == AppTheme::Light, SettingsMessage::SetThemeSetting(AppTheme::Light)),
                                choice("Dark", self.setting_list.theme == AppTheme::Dark, SettingsMessage::SetThemeSetting(AppTheme::Dark)),
                            ]
                            .spacing(8)
                            .into(),
                        ),

                        setting_row(
                            "Default screen",
                            row![
                                choice("Home", self.setting_list.screen == AppScreen::Home, SettingsMessage::SetScreenSetting(AppScreen::Home)),
                                choice("Dashboard", self.setting_list.screen == AppScreen::Dashboard, SettingsMessage::SetScreenSetting(AppScreen::Dashboard)),
                                choice("Settings", self.setting_list.screen == AppScreen::Settings, SettingsMessage::SetScreenSetting(AppScreen::Settings)),
                            ]
                            .spacing(8)
                            .into(),
                        ),

                        setting_row(
                            "Text size",
                            row![
                                choice("Small", self.setting_list.text_size == TextSize::Small, SettingsMessage::SetTextSize(TextSize::Small)),
                                choice("Normal", self.setting_list.text_size == TextSize::Normal, SettingsMessage::SetTextSize(TextSize::Normal)),
                                choice("Large", self.setting_list.text_size == TextSize::Large, SettingsMessage::SetTextSize(TextSize::Large)),
                            ]
                            .spacing(8)
                            .into(),
                        ),
                    ]
                    .spacing(12)
                    .into(),
                ),

                // Execution
                section(
                    "Execution",
                    column![
                        setting_row(
                            "Shell",
                            row![
                                choice("Bash", self.setting_list.shell == ShellKind::Bash, SettingsMessage::SetShell(ShellKind::Bash)),
                            ]
                            .spacing(8)
                            .into(),
                        ),

                        setting_row(
                            "Terminal",
                            row![
                                choice("Gnome", self.setting_list.terminal == TerminalKind::Gnome, SettingsMessage::SetTerminal(TerminalKind::Gnome)),
                                choice("Konsole", self.setting_list.terminal == TerminalKind::Konsole, SettingsMessage::SetTerminal(TerminalKind::Konsole)),
                                choice("Kitty", self.setting_list.terminal == TerminalKind::Kitty, SettingsMessage::SetTerminal(TerminalKind::Kitty)),
                                choice("Xterm", self.setting_list.terminal == TerminalKind::Xterm, SettingsMessage::SetTerminal(TerminalKind::Xterm)),
                            ]
                            .spacing(8)
                            .into(),
                        ),

                        setting_row(
                            "On Run",
                            row![
                                choice("New window", self.setting_list.run_mode == RunMode::NewWindow, SettingsMessage::SetRunMode(RunMode::NewWindow)),
                                choice("Reuse session", self.setting_list.run_mode == RunMode::ReuseSession, SettingsMessage::SetRunMode(RunMode::ReuseSession)),
                            ]
                            .spacing(8)
                            .into(),
                        ),

                        setting_row(
                            "Keep open",
                            toggler(self.setting_list.keep_open)
                                .on_toggle(SettingsMessage::ToggleKeepOpen)
                                .into(),
                        ),
                    ]
                    .spacing(12)
                    .into(),
                ),

                // Data
                section(
                    "Data",
                    column![
                        setting_row("Directory", text(self.data_dir.clone()).into()),
                        button("Open folder")
                            .on_press(SettingsMessage::OpenDataDir),
                    ]
                    .spacing(12)
                    .into(),
                ),

                // About
                section(
                    "About",
                    column![
                        setting_row("Version", text(self.version.clone()).into()),

                    ]
                    .spacing(12)
                    .into(),
                ),
            ]
            .spacing(14)
            .padding(20)
        )
        .width(800)
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

fn choice<'a>(
    label: &'a str,
    active: bool,
    on_press: SettingsMessage,
) -> Element<'a, SettingsMessage> {
    button(text(label).size(14))
        .on_press(on_press)
        .padding([6, 12])
        .style(move |theme: &Theme, _| {
            let p = theme.palette();
            if active {
                button::Style {
                    background: Some(p.primary.into()),
                    text_color: p.text,
                    border: Border {
                        color: p.primary,
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                }
            } else {
                button::Style {
                    background: None,
                    text_color: p.text,
                    border: Border {
                        color: p.primary.scale_alpha(0.5),
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                }
            }
        })
        .into()
}
