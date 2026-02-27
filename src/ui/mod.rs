pub mod styles;
pub mod widgets;

use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Element, Length, Task};
use crate::engine::rules::SortingRule;
use crate::config;

#[derive(Debug, Clone)]
pub enum Message {
    SelectSourceDir,
    SourceDirSelected(Option<std::path::PathBuf>),
    StartOrganizing,
    OrganizeCompleted(Result<bool, String>),
    
    // Rule management
    AddRule,
    UpdateRuleExtension(usize, String),
    UpdateRuleCategory(usize, String),
    SelectRuleTarget(usize),
    RuleTargetSelected(usize, Option<std::path::PathBuf>),
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
        let rules = config::load_rules().unwrap_or_default();
        Self {
            source_dir: None,
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
                // Return a task to handle folder selection
                // The actual logic will be connected in Integration stage (Stage 5)
                Task::none()
            }
            Message::SourceDirSelected(path_opt) => {
                if let Some(path) = path_opt {
                    self.source_dir = Some(path);
                    self.status_message = "Source directory selected.".to_string();
                }
                Task::none()
            }
            Message::StartOrganizing => {
                if self.source_dir.is_some() && !self.is_processing {
                    self.is_processing = true;
                    self.status_message = "Organizing files...".to_string();
                    // Connect async task in Integration stage
                }
                Task::none()
            }
            Message::OrganizeCompleted(result) => {
                self.is_processing = false;
                match result {
                    Ok(_) => self.status_message = "Organization complete!".to_string(),
                    Err(e) => self.status_message = format!("Error: {}", e),
                }
                Task::none()
            }
            Message::AddRule => {
                self.rules.push(SortingRule::new("", "", "New Category"));
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
            Message::SelectRuleTarget(_index) => {
                // To be wired with rfd in Stage 5
                Task::none()
            }
            Message::RuleTargetSelected(index, path_opt) => {
                if let Some(path) = path_opt {
                    if let Some(rule) = self.rules.get_mut(index) {
                        rule.target_path = path.to_string_lossy().to_string();
                    }
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
                    self.status_message = "Rules saved successfully!".to_string();
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        // Main view logic here
        let title = text("RusticSort - File Organizer")
            .size(32)
            .width(Length::Fill)
            .align_x(Alignment::Center);

        let source_row = row![
            text(
                self.source_dir
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| "No source directory selected".to_string())
            )
            .width(Length::Fill),
            button("Select Source").on_press(Message::SelectSourceDir)
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let mut rules_column = column![].spacing(10);
        
        for (i, rule) in self.rules.iter().enumerate() {
            rules_column = rules_column.push(
                row![
                    text_input("Extension", &rule.extension)
                        .on_input(move |ext| Message::UpdateRuleExtension(i, ext))
                        .width(Length::Fixed(80.0)),
                    text_input("Category", &rule.category_name)
                        .on_input(move |cat| Message::UpdateRuleCategory(i, cat))
                        .width(Length::Fixed(150.0)),
                    text(&rule.target_path).width(Length::Fill),
                    button("Select Target").on_press(Message::SelectRuleTarget(i)),
                    button("Remove").on_press(Message::RemoveRule(i)),
                ]
                .spacing(10)
                .align_y(Alignment::Center)
            );
        }

        let rule_controls = row![
            button("Add Rule").on_press(Message::AddRule),
            button("Save Rules").on_press(Message::SaveRules)
        ]
        .spacing(10);

        let action_row = row![
            button("Start Organizing").on_press(Message::StartOrganizing),
            text(&self.status_message).width(Length::Fill).align_y(Alignment::Center)
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let content = column![
            title,
            source_row,
            text("Sorting Rules:").size(20),
            scrollable(rules_column).height(Length::Fill),
            rule_controls,
            action_row
        ]
        .spacing(20)
        .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
