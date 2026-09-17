use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use serde::{Deserialize, Serialize};

use crate::data::ensure_data_dir;
use crate::utils::logger::{log_debug, log_info, log_warn};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum AppTheme {
    #[default]
    Dark,
    Light,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum AppScreen {
    #[default]
    Home,
    Dashboard,
    Settings,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum TextSize {
    Small,
    #[default]
    Normal,
    Large,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ShellKind {
    #[default]
    Bash,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum TerminalKind {
    #[default]
    Gnome,
    Konsole,
    Kitty,
    Xterm,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum RunMode {
    #[default]
    NewWindow,
    ReuseSession,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SettingsInfo {
    pub theme: AppTheme,
    pub screen: AppScreen,
    pub text_size: TextSize,
    pub shell: ShellKind,
    pub terminal: TerminalKind,
    pub run_mode: RunMode,
    pub keep_open: bool,
}

impl Default for SettingsInfo {
    fn default() -> Self {
        Self {
            theme: Default::default(),
            screen: Default::default(),
            text_size: Default::default(),
            shell: Default::default(),
            terminal: Default::default(),
            run_mode: Default::default(),
            keep_open: true,
        }
    }
}

const SETTINGS_FILE: &str = "settings.json";
static SETTINGS: OnceLock<Mutex<SettingsInfo>> = OnceLock::new();


// Load settings to cache
pub fn load_settings() -> Result<(), String> {
    // Path
    let path = settings_path()?;
    log_info(&format!("Settings path: {:?}", path));

    // Settings
    let settings: SettingsInfo = match path.exists() {
        true => match fs::read_to_string(&path) {
            Ok(content) => match serde_json::from_str::<SettingsInfo>(&content) {
                Ok(settings) => {
                    log_debug("Loaded settings from disk");
                    settings
                },
                Err(e) => {
                    let _ = fs::rename(&path, path.with_extension("json.bak")); // old file
                    log_warn(&format!("Failed to parse settings from json, using default: {}", e));
                    write_default_settings()?
                }
            },
            Err(e) => {
                log_warn(&format!("Failed to read settings json, using default: {}", e));
                write_default_settings()?
            }
        },
        false => {
            log_warn("Failed to read settings from disk, using default");
            write_default_settings()?
        },
    };

    // save cache
    *cache().lock().map_err(|e| e.to_string())? = settings;
    log_debug("Saved settings to cache");

    Ok(())
}


fn settings_path() -> Result<PathBuf, String> {
    Ok(ensure_data_dir()?.join(SETTINGS_FILE))
}

fn cache() -> &'static Mutex<SettingsInfo> {
    SETTINGS.get_or_init(|| Mutex::new(SettingsInfo::default()))
}

fn write_default_settings() -> Result<SettingsInfo, String> {
    let settings = SettingsInfo::default();
    write_settings(&settings)?;
    Ok(settings)
}

fn write_settings(settings: &SettingsInfo) -> Result<(), String> {
    let path = settings_path()?;
    let raw = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("Failed to serialize settings: {e}"))?;
    fs::write(path, raw).map_err(|e| format!("Failed to write settings: {e}"))?;
    Ok(())
}

// Get all settings
pub fn get_settings() -> Result<SettingsInfo, String> {
    Ok(cache().lock().map_err(|e| e.to_string())?.clone())
}

// Set a single setting with a closure
pub fn set_settings(f: impl FnOnce(&mut SettingsInfo)) -> Result<(), String> {
    let mut guard = cache().lock().map_err(|e| e.to_string())?;
    f(&mut guard);

    log_info("Saving settings");
    write_settings(&guard)
}
