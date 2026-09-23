mod settings;
mod navigator;
mod setting_list;

use crate::components::status_bar::StatusMessage;
use crate::data::settings::{AppScreen, AppTheme, RunMode, SettingsInfo, ShellKind, TerminalKind, TextSize};


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
