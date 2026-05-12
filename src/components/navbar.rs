use iced::widget::{button, container, row, text, text_input};
use iced::{Alignment, Element, Length};

#[derive(Debug, Clone, PartialEq)]
pub enum NavbarMessage {
    // sent
    GotoHome,
    GotoSettings,
    ToggleDarkMode,
    // sent, recieved
    SearchChanged(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Navbar {
    search_query: String,
    search_results: Vec<String>,
}

const APP_NAME: &str = "Shelf";

impl Navbar {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
            search_results: Vec::new(),
        }
    }

    pub fn update(&mut self, message: &NavbarMessage) {
        match message {
            NavbarMessage::SearchChanged(query) => {
                self.search_query = query.clone();
                self.search_results = Vec::from(["temp".to_string()]);
            }
            _ => {}
        }
    }

    pub fn view(&self) -> Element<'_, NavbarMessage> {
        row![
            // App name
            button(text(APP_NAME).size(24))
                .on_press(NavbarMessage::GotoHome),

            // Search bar
            container(
                text_input("Search...", &self.search_query)
                    .on_input(|v| {
                        NavbarMessage::SearchChanged(v)
                    })
            )
            .width(Length::Fill)
            .padding(5),

            // Shortcuts
            row![
                button("⚙ Settings")
                    .on_press(NavbarMessage::GotoSettings),

                button("☀ Dark Mode")
                    .on_press(NavbarMessage::ToggleDarkMode),
            ]
            .spacing(10),
        ]
        .align_y(Alignment::Center)
        .spacing(20)
        .padding(10)
        .into()
    }
}
