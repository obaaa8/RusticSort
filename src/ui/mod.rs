pub mod styles;
pub mod widgets;

use iced::widget::{button, checkbox, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Element, Length, Task};
use crate::engine::rules::{SortingRule, default_rules};
use crate::engine::organizer::{scan_directory, organize_file};
use crate::config;

#[derive(Debug, Clone)]
pub enum Message {
    SelectSourceDir,
    SourceDirSelected(Option<std::path::PathBuf>),
    StartOrganizing,
    OrganizeCompleted(Result<usize, String>),

    // Rule management
    AddRule,
    ToggleRule(usize, bool),
    UpdateRuleExtension(usize, String),
    UpdateRuleCategory(usize, String),
    UpdateRuleTarget(usize, String),
    RemoveRule(usize),
    SaveRules,
}

pub struct RusticSortApp {
    source_dir: Option<std::path::PathBuf>,
    rules: Vec<SortingRule>,
    is_processing: bool,
    status_message: String,
}

impl Default for RusticSortApp {
    fn default() -> Self {
        // Try loading saved rules, fallback to defaults
        let rules = match config::load_rules() {
            Ok(r) if !r.is_empty() => r,
            _ => default_rules(),
        };

        // Default source directory: user's Downloads folder
        let source_dir = dirs::download_dir();

        Self {
            source_dir,
            rules,
            is_processing: false,
            status_message: "Ready".to_string(),
        }
    }
}

impl RusticSortApp {
    pub fn new() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    pub fn title(&self) -> String {
        String::from("RusticSort")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectSourceDir => {
                Task::perform(
                    async { rfd::AsyncFileDialog::new().pick_folder().await.map(|h| h.path().to_path_buf()) },
                    Message::SourceDirSelected,
                )
            }
            Message::SourceDirSelected(path_opt) => {
                if let Some(path) = path_opt {
                    self.source_dir = Some(path);
                    self.status_message = "Source directory selected.".to_string();
                }
                Task::none()
            }
            Message::StartOrganizing => {
                if let Some(source) = self.source_dir.clone() {
                    if !self.is_processing {
                        self.is_processing = true;
                        self.status_message = "Organizing files...".to_string();
                        let rules = self.rules.clone();

                        return Task::perform(
                            async move {
                                let mut moved_count: usize = 0;
                                let files = scan_directory(&source);
                                for file in files {
                                    match organize_file(&file, &source, &rules) {
                                        Ok(true) => moved_count += 1,
                                        Ok(false) => {},
                                        Err(e) => return Err(e.to_string()),
                                    }
                                }
                                Ok(moved_count)
                            },
                            Message::OrganizeCompleted,
                        );
                    }
                }
                Task::none()
            }
            Message::OrganizeCompleted(result) => {
                self.is_processing = false;
                match result {
                    Ok(count) if count > 0 => {
                        self.status_message = format!("Done! {} file(s) moved.", count);
                    }
                    Ok(_) => {
                        self.status_message = "Done! No files matched the enabled rules.".to_string();
                    }
                    Err(e) => self.status_message = format!("Error: {}", e),
                }
                Task::none()
            }
            Message::AddRule => {
                self.rules.push(SortingRule::new("", "", "New"));
                Task::none()
            }
            Message::ToggleRule(index, enabled) => {
                if let Some(rule) = self.rules.get_mut(index) {
                    rule.enabled = enabled;
                }
                Task::none()
            }
            Message::UpdateRuleExtension(index, ext) => {
                if let Some(rule) = self.rules.get_mut(index) {
                    rule.extension = ext;
                }
                Task::none()
            }
            Message::UpdateRuleCategory(index, cat) => {
                if let Some(rule) = self.rules.get_mut(index) {
                    rule.category_name = cat;
                }
                Task::none()
            }
            Message::UpdateRuleTarget(index, target) => {
                if let Some(rule) = self.rules.get_mut(index) {
                    rule.target_path = target;
                }
                Task::none()
            }
            Message::RemoveRule(index) => {
                if index < self.rules.len() {
                    self.rules.remove(index);
                }
                Task::none()
            }
            Message::SaveRules => {
                if let Err(e) = config::save_rules(&self.rules) {
                    self.status_message = format!("Failed to save rules: {}", e);
                } else {
                    self.status_message = "Rules saved!".to_string();
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let title = text("RusticSort - File Organizer")
            .size(28);

        // Source directory row
        let source_label = text(
            self.source_dir
                .as_ref()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| "No source directory selected".to_string())
        )
        .width(Length::Fill);

        let source_row = row![
            text("Source: ").size(16),
            source_label,
            button("Change").on_press(Message::SelectSourceDir)
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        // Rules header
        let rules_header = row![
            text("On").width(Length::Fixed(30.0)),
            text("Ext").width(Length::Fixed(80.0)),
            text("Target Folder").width(Length::Fixed(150.0)),
            text("Category").width(Length::Fill),
            text("").width(Length::Fixed(70.0)),
        ]
        .spacing(10);

        // Rules list
        let mut rules_column = column![].spacing(5);

        for (i, rule) in self.rules.iter().enumerate() {
            rules_column = rules_column.push(
                row![
                    checkbox("", rule.enabled)
                        .on_toggle(move |val| Message::ToggleRule(i, val))
                        .width(Length::Fixed(30.0)),
                    text_input("ext", &rule.extension)
                        .on_input(move |ext| Message::UpdateRuleExtension(i, ext))
                        .width(Length::Fixed(80.0)),
                    text_input("folder", &rule.target_path)
                        .on_input(move |target| Message::UpdateRuleTarget(i, target))
                        .width(Length::Fixed(150.0)),
                    text_input("category", &rule.category_name)
                        .on_input(move |cat| Message::UpdateRuleCategory(i, cat))
                        .width(Length::Fill),
                    button("X").on_press(Message::RemoveRule(i))
                        .width(Length::Fixed(30.0)),
                ]
                .spacing(10)
                .align_y(Alignment::Center)
            );
        }

        let rule_controls = row![
            button("+ Add Rule").on_press(Message::AddRule),
            button("Save Rules").on_press(Message::SaveRules)
        ]
        .spacing(10);

        let start_btn = if self.is_processing {
            button("Working...")
        } else {
            button("Start Organizing").on_press(Message::StartOrganizing)
        };

        let action_row = row![
            start_btn,
            text(&self.status_message).width(Length::Fill)
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let content = column![
            title,
            source_row,
            rules_header,
            scrollable(rules_column).height(Length::Fill),
            rule_controls,
            action_row
        ]
        .spacing(15)
        .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
