pub mod styles;
pub mod widgets;

use iced::widget::{button, checkbox, column, container, row, scrollable, text, text_input, Space, stack, image};
use iced::{Alignment, Color, Element, Length, Task};
use crate::engine::rules::{SortingRule, default_rules};
use crate::engine::organizer::{scan_directory, organize_file, undo_moves, MoveRecord};
use crate::config;
use crate::strings::S;

// ---- Messages ----

#[derive(Debug, Clone)]
pub enum Message {
    GoToStep(usize),
    NextStep,
    PrevStep,

    SelectSourceDir,
    SourceDirSelected(Option<std::path::PathBuf>),

    AddRule,
    ToggleRule(usize, bool),
    UpdateRuleExtension(usize, String),
    UpdateRuleCategory(usize, String),
    UpdateRuleTarget(usize, String),
    RemoveRule(usize),
    SaveRules,

    Process,
    ProcessComplete(Vec<crate::engine::organizer::MoveRecord>),
    UndoMoves,
    UndoCompleted(Result<usize, String>),

    DismissError,
    GoHome,
    OpenFolder,
    OpenWebsite,
    ShowAbout,
    ShowDeveloper,
    DismissModal,
}

#[derive(Debug, Clone)]
enum Modal {
    None,
    About,
    Developer,
    Error(String),
    Processing,
}

pub struct RusticSortApp {
    step: usize,
    source_dir: Option<std::path::PathBuf>,
    rules: Vec<SortingRule>,
    move_records: Vec<MoveRecord>,
    modal: Modal,
}

impl Default for RusticSortApp {
    fn default() -> Self {
        let rules = match config::load_rules() {
            Ok(r) if !r.is_empty() => r,
            _ => default_rules(),
        };
        Self {
            step: 0,
            source_dir: dirs::download_dir(),
            rules,
            move_records: Vec::new(),
            modal: Modal::None,
        }
    }
}

impl RusticSortApp {
    pub fn new() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    pub fn title(&self) -> String {
        S.get("app", "name").to_string()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::GoToStep(s) => { self.step = s; Task::none() }
            Message::NextStep => { self.step += 1; Task::none() }
            Message::PrevStep => { if self.step > 0 { self.step -= 1; } Task::none() }
            Message::GoHome => { self.step = 0; self.move_records.clear(); Task::none() }

            Message::SelectSourceDir => {
                Task::perform(
                    async { rfd::AsyncFileDialog::new().pick_folder().await.map(|h| h.path().to_path_buf()) },
                    Message::SourceDirSelected,
                )
            }
            Message::SourceDirSelected(p) => { if let Some(path) = p { self.source_dir = Some(path); } Task::none() }

            Message::AddRule => { self.rules.push(SortingRule::new("", "", "")); Task::none() }
            Message::ToggleRule(i, v) => { if let Some(r) = self.rules.get_mut(i) { r.enabled = v; } Task::none() }
            Message::UpdateRuleExtension(i, v) => { if let Some(r) = self.rules.get_mut(i) { r.extension = v; } Task::none() }
            Message::UpdateRuleCategory(i, v) => { if let Some(r) = self.rules.get_mut(i) { r.category_name = v; } Task::none() }
            Message::UpdateRuleTarget(i, v) => { if let Some(r) = self.rules.get_mut(i) { r.target_path = v; } Task::none() }
            Message::RemoveRule(i) => { if i < self.rules.len() { self.rules.remove(i); } Task::none() }
            Message::SaveRules => {
                match config::save_rules(&self.rules) {
                    Ok(_) => {}
                    Err(e) => { self.modal = Modal::Error(format!("{}: {}", S.get("messages", "save_failed"), e)); }
                }
                Task::none()
            }

            Message::Process => {
                if let Some(source) = self.source_dir.clone() {
                    self.modal = Modal::Processing;
                    let rules = self.rules.clone();
                    Task::perform(
                        async move {
                            let mut records = Vec::new();
                            let files = scan_directory(&source);
                            for file in files {
                                match organize_file(&file, &source, &rules) {
                                    Ok(Some(record)) => { records.push(record); }
                                    Ok(None) => {}
                                    Err(e) => return Err(e.to_string()),
                                }
                            }
                            Ok(records)
                        },
                        |result| match result {
                            Ok(records) => Message::ProcessComplete(records),
                            Err(_e) => Message::ProcessComplete(vec![]), // Fallback map on error
                        }
                    )
                } else {
                    self.modal = Modal::Error(S.get("messages", "select_dir_first").to_string());
                    Task::none()
                }
            }

            Message::ProcessComplete(records) => {
                self.move_records = records;
                self.modal = Modal::None;
                self.step = 4;
                Task::none()
            }

            Message::UndoMoves => {
                self.modal = Modal::Processing;
                let records = self.move_records.clone();
                Task::perform(
                    async move {
                        undo_moves(&records).map_err(|e| e.to_string())
                    },
                    Message::UndoCompleted,
                )
            }

            Message::UndoCompleted(result) => {
                self.modal = Modal::None;
                match result {
                    Ok(count) => {
                        self.move_records.clear();
                        self.modal = Modal::Error(format!("{} - {} file(s)", S.get("messages", "undo_success"), count));
                    }
                    Err(e) => { self.modal = Modal::Error(e); }
                }
                Task::none()
            }

            Message::OpenWebsite => { let _ = open::that(S.get("links", "website")); Task::none() }
            Message::OpenFolder => { if let Some(s) = &self.source_dir { let _ = open::that(s); } Task::none() }
            Message::ShowAbout => { self.modal = Modal::About; Task::none() }
            Message::ShowDeveloper => { self.modal = Modal::Developer; Task::none() }
            Message::DismissModal | Message::DismissError => { self.modal = Modal::None; Task::none() }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let main_view: Element<Message> = match self.step {
            0 => self.view_welcome(),
            1 => self.view_source(),
            2 => self.view_rules(),
            3 => self.view_summary(),
            4 => self.view_result(),
            _ => self.view_welcome(),
        };

        match &self.modal {
            Modal::None => main_view,
            Modal::About => stack![main_view, self.view_about_modal()].into(),
            Modal::Developer => stack![main_view, self.view_developer_modal()].into(),
            Modal::Error(e) => stack![main_view, Self::view_error_modal(e)].into(),
            Modal::Processing => stack![main_view, self.view_processing_modal()].into(),
        }
    }

    // ========== STEP 0: Welcome ==========

    fn view_welcome(&self) -> Element<'_, Message> {
        let content = column![
            Space::with_height(40),
            image(image::Handle::from_bytes(include_bytes!("../../assets/icons/rusticsort-128.png").as_slice()))
                .width(Length::Fixed(160.0))
                .height(Length::Fixed(160.0)),
            Space::with_height(30),
            text(S.get("app", "name")).size(48).color(Color::from_rgb(0.15, 0.15, 0.2)),
            Space::with_height(12),
            text(S.get("app", "tagline")).size(20).color(Color::from_rgb(0.45, 0.48, 0.55)),
            Space::with_height(30),
            container(
                text(S.get("app", "description")).size(17).color(Color::from_rgb(0.4, 0.42, 0.48)).center()
            ).max_width(550),
            Space::with_height(40),
            button(text(S.get("buttons", "get_started")).size(18))
                .on_press(Message::NextStep)
                .style(styles::primary_button)
                .padding([14, 40]),
            Space::with_height(24),
            // Website & About icons
            row![
                button(text("[ Website ]").size(14))
                    .on_press(Message::OpenWebsite)
                    .style(styles::outline_button)
                    .padding([8, 16]),
                Space::with_width(12),
                button(text("[ About ]").size(14))
                    .on_press(Message::ShowAbout)
                    .style(styles::outline_button)
                    .padding([8, 16]),
            ]
            .align_y(Alignment::Center),
            Space::with_height(Length::Fill),
        ]
        .align_x(Alignment::Center)
        .spacing(0);

        self.wrap_page_with_footer(content.into())
    }

    // ========== STEP 1: Source ==========

    fn view_source(&self) -> Element<'_, Message> {
        let source_text = self.source_dir.as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "No directory selected".to_string());

        let content = column![
            self.view_step_indicator(1),
            Space::with_height(30),
            text(S.get("steps", "step1_title")).size(30).color(Color::from_rgb(0.15, 0.15, 0.2)),
            Space::with_height(12),
            text(S.get("steps", "step1_desc")).size(16).color(Color::from_rgb(0.4, 0.42, 0.48)),
            Space::with_height(40),
            container(
                column![
                    text("Current Source Directory:").size(16).color(Color::from_rgb(0.4, 0.4, 0.5)),
                    Space::with_height(10),
                    row![
                        container(text(source_text).size(16))
                            .padding([14, 20])
                            .width(Length::Fill)
                            .style(styles::card_container),
                        Space::with_width(14),
                        button(text(S.get("buttons", "browse")).size(16))
                            .on_press(Message::SelectSourceDir)
                            .style(styles::outline_button)
                            .padding([12, 24]),
                    ]
                    .align_y(Alignment::Center)
                ]
            ).max_width(600),
            Space::with_height(50),
            self.view_nav_buttons(false, true),
        ]
        .align_x(Alignment::Center)
        .spacing(0);

        self.wrap_page(content.into())
    }

    // ========== STEP 2: Rules ==========

    fn view_rules(&self) -> Element<'_, Message> {
        let enabled_count = self.rules.iter().filter(|r| r.enabled).count();

        let col_headers = row![
            Space::with_width(Length::Fixed(32.0)),
            text("Ext").size(11).width(Length::Fixed(65.0)).color(Color::from_rgb(0.45, 0.45, 0.55)),
            text("Target Folder").size(11).width(Length::Fixed(130.0)).color(Color::from_rgb(0.45, 0.45, 0.55)),
            text("Category").size(11).width(Length::Fill).color(Color::from_rgb(0.45, 0.45, 0.55)),
            Space::with_width(Length::Fixed(36.0)),
        ].spacing(6);

        let mut rules_list = column![].spacing(3);
        for (i, rule) in self.rules.iter().enumerate() {
            rules_list = rules_list.push(
                container(
                    row![
                        checkbox("", rule.enabled)
                            .on_toggle(move |v| Message::ToggleRule(i, v))
                            .width(Length::Fixed(28.0)).size(16),
                        text_input("ext", &rule.extension)
                            .on_input(move |v| Message::UpdateRuleExtension(i, v))
                            .width(Length::Fixed(65.0)).size(12),
                        text_input("folder", &rule.target_path)
                            .on_input(move |v| Message::UpdateRuleTarget(i, v))
                            .width(Length::Fixed(130.0)).size(12),
                        text_input("category", &rule.category_name)
                            .on_input(move |v| Message::UpdateRuleCategory(i, v))
                            .width(Length::Fill).size(12),
                        button(text("X").size(11))
                            .on_press(Message::RemoveRule(i))
                            .style(styles::danger_button)
                            .padding([3, 8]),
                    ].spacing(6).align_y(Alignment::Center)
                ).padding([4, 6]).style(styles::card_container)
            );
        }

        let rule_buttons = row![
            button(text(S.get("buttons", "add_rule")).size(12))
                .on_press(Message::AddRule)
                .style(styles::outline_button)
                .padding([6, 12]),
            button(text(S.get("buttons", "save_rules")).size(12))
                .on_press(Message::SaveRules)
                .style(styles::outline_button)
                .padding([6, 12]),
            Space::with_width(Length::Fill),
            text(format!("{}/{} enabled", enabled_count, self.rules.len()))
                .size(11).color(Color::from_rgb(0.5, 0.55, 0.6)),
        ].spacing(8).align_y(Alignment::Center);

        let content = column![
            self.view_step_indicator(2),
            Space::with_height(12),
            text(S.get("steps", "step2_title")).size(22).color(Color::from_rgb(0.15, 0.15, 0.2)),
            Space::with_height(6),
            text(S.get("steps", "step2_desc")).size(13).color(Color::from_rgb(0.4, 0.42, 0.48)),
            Space::with_height(12),
            col_headers,
            scrollable(rules_list).height(Length::Fill),
            Space::with_height(8),
            rule_buttons,
            Space::with_height(12),
            self.view_nav_buttons(true, true),
        ].spacing(0).width(Length::Fill);

        self.wrap_page(
            container(content).max_width(600).center_x(Length::Fill).into()
        )
    }

    // ========== STEP 3: Summary ==========

    fn view_summary(&self) -> Element<'_, Message> {
        let source_text = self.source_dir.as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "Not selected".to_string());

        let enabled_rules: Vec<&SortingRule> = self.rules.iter().filter(|r| r.enabled).collect();

        let mut preview_items = column![].spacing(2);
        let mut match_count = 0;

        if let Some(source) = &self.source_dir {
            let files = scan_directory(source);
            for file in &files {
                if let Some(ext) = file.extension().and_then(|e| e.to_str())
                    && let Some(rule) = enabled_rules.iter().find(|r| r.extension.eq_ignore_ascii_case(ext)) {
                        let name = file.file_name().unwrap_or_default().to_string_lossy().to_string();
                        if match_count < 15 {
                            preview_items = preview_items.push(
                                text(format!("  {} -> {}/", name, rule.target_path))
                                    .size(12).color(Color::from_rgb(0.3, 0.35, 0.4))
                            );
                        }
                        match_count += 1;
                    }
            }
        }

        if match_count > 15 {
            preview_items = preview_items.push(
                text(format!("  ... and {} more files", match_count - 15))
                    .size(12).color(Color::from_rgb(0.5, 0.5, 0.6))
            );
        }

        let summary_info = if match_count > 0 {
            format!("{} file(s) will be organized into folders.", match_count)
        } else {
            S.get("messages", "no_match").to_string()
        };

        let start_btn = if match_count > 0 {
            button(text(S.get("buttons", "start_organizing")).size(14))
                .on_press(Message::Process)
                .style(styles::primary_button)
                .padding([10, 28])
        } else {
            button(text(S.get("messages", "no_files")).size(14))
                .style(styles::primary_button)
                .padding([10, 28])
        };

        let content = column![
            self.view_step_indicator(3),
            Space::with_height(16),
            text(S.get("steps", "step3_title")).size(22).color(Color::from_rgb(0.15, 0.15, 0.2)),
            Space::with_height(6),
            text(S.get("steps", "step3_desc")).size(13).color(Color::from_rgb(0.4, 0.42, 0.48)),
            Space::with_height(16),
            container(
                column![
                    row![
                        text("Source: ").size(12).color(Color::from_rgb(0.4, 0.4, 0.5)),
                        text(source_text).size(12).color(Color::from_rgb(0.2, 0.2, 0.3)),
                    ],
                    row![
                        text("Enabled rules: ").size(12).color(Color::from_rgb(0.4, 0.4, 0.5)),
                        text(format!("{}", enabled_rules.len())).size(12).color(Color::from_rgb(0.2, 0.2, 0.3)),
                    ],
                    Space::with_height(4),
                    text(summary_info).size(13).color(Color::from_rgb(0.18, 0.45, 0.72)),
                ].spacing(4)
            ).padding(14).max_width(500).style(styles::card_container),
            Space::with_height(10),
            container(
                scrollable(preview_items).height(Length::Fixed(180.0))
            ).padding(10).max_width(500).style(styles::card_container),
            Space::with_height(20),
            row![
                button(text(S.get("buttons", "back")).size(13))
                    .on_press(Message::PrevStep)
                    .style(styles::outline_button)
                    .padding([9, 20]),
                Space::with_width(Length::Fill),
                start_btn,
            ].width(Length::Fill),
        ]
        .align_x(Alignment::Center)
        .spacing(0);

        self.wrap_page(content.into())
    }

    // ========== STEP 4: Result ==========

    fn view_result(&self) -> Element<'_, Message> {
        let count = self.move_records.len();

        let mut file_list = column![].spacing(2);
        for record in self.move_records.iter().take(20) {
            let name = record.original_path.file_name().unwrap_or_default().to_string_lossy().to_string();
            file_list = file_list.push(
                text(format!("  [OK] {}", name)).size(12).color(Color::from_rgb(0.2, 0.55, 0.2))
            );
        }
        if count > 20 {
            file_list = file_list.push(
                text(format!("  ... and {} more", count - 20)).size(12).color(Color::from_rgb(0.4, 0.5, 0.4))
            );
        }

        let undo_btn = if !self.move_records.is_empty() {
            button(text(S.get("messages", "undo_btn")).size(13))
                .on_press(Message::UndoMoves)
                .style(styles::danger_button)
                .padding([9, 20])
        } else {
            button(text(S.get("messages", "undo_btn")).size(13))
                .style(styles::danger_button)
                .padding([9, 20])
        };

        let content = column![
            Space::with_height(40),
            container(
                text(S.get("messages", "complete_title")).size(20).color(Color::from_rgb(0.15, 0.5, 0.15))
            ).padding([12, 18]).style(styles::success_card),
            Space::with_height(14),
            text(format!("{} file(s) moved successfully.", count))
                .size(14).color(Color::from_rgb(0.3, 0.3, 0.4)),
            Space::with_height(12),
            container(
                scrollable(file_list).height(Length::Fixed(200.0))
            ).padding(12).max_width(480).style(styles::card_container),
            Space::with_height(24),
            row![
                button(text(S.get("buttons", "back_to_home")).size(13))
                    .on_press(Message::GoHome)
                    .style(styles::outline_button)
                    .padding([9, 20]),
                Space::with_width(10),
                undo_btn,
                Space::with_width(10),
                button(text(S.get("buttons", "open_folder")).size(13))
                    .on_press(Message::OpenFolder)
                    .style(styles::primary_button)
                    .padding([9, 20]),
            ],
        ]
        .align_x(Alignment::Center)
        .spacing(0);

        self.wrap_page(content.into())
    }

    // ========== Shared: Step Indicator ==========

    fn view_step_indicator(&self, current: usize) -> Element<'_, Message> {
        let steps = ["Source", "Rules", "Organize"];
        let mut indicators = row![].spacing(4).align_y(Alignment::Center);

        for (i, label) in steps.iter().enumerate() {
            let step_num = i + 1;
            let is_active = step_num == current;
            let is_done = step_num < current;

            let color = if is_active {
                Color::from_rgb(0.18, 0.55, 0.82)
            } else if is_done {
                Color::from_rgb(0.3, 0.7, 0.4)
            } else {
                Color::from_rgb(0.7, 0.72, 0.76)
            };

            let label_text = if is_done {
                format!("[{}] {}", step_num, label)
            } else {
                format!("{}. {}", step_num, label)
            };

            indicators = indicators.push(text(label_text).size(12).color(color));
            if i < steps.len() - 1 {
                indicators = indicators.push(text("  ---  ").size(11).color(Color::from_rgb(0.78, 0.8, 0.84)));
            }
        }

        container(indicators).center_x(Length::Fill).padding([8, 0]).into()
    }

    fn view_nav_buttons(&self, show_back: bool, show_next: bool) -> Element<'_, Message> {
        let mut nav = row![].spacing(10).width(Length::Fill);

        if show_back {
            nav = nav.push(
                button(text(S.get("buttons", "back")).size(13))
                    .on_press(Message::PrevStep)
                    .style(styles::outline_button)
                    .padding([9, 20])
            );
        }
        nav = nav.push(Space::with_width(Length::Fill));
        if show_next {
            nav = nav.push(
                button(text(S.get("buttons", "next")).size(13))
                    .on_press(Message::NextStep)
                    .style(styles::primary_button)
                    .padding([9, 24])
            );
        }
        nav.into()
    }

    // ========== Page Wrapper ==========

    fn wrap_page<'a>(&self, content: Element<'a, Message>) -> Element<'a, Message> {
        let header = container(
            row![
                text(S.get("app", "name")).size(20).color(Color::WHITE),
                Space::with_width(Length::Fill),
                text(S.get("app", "tagline")).size(12).color(Color::from_rgb(0.6, 0.65, 0.7)),
            ].align_y(Alignment::Center).padding([10, 20])
        ).width(Length::Fill).style(styles::header_container);

        let body = container(
            container(content).max_width(620).center_x(Length::Fill).padding(20)
        ).width(Length::Fill).height(Length::Fill).center_x(Length::Fill).center_y(Length::Fill);

        container(column![header, body])
            .width(Length::Fill).height(Length::Fill)
            .style(styles::main_container)
            .into()
    }

    fn wrap_page_with_footer<'a>(&self, content: Element<'a, Message>) -> Element<'a, Message> {
        let header = container(
            row![
                text(S.get("app", "name")).size(20).color(Color::WHITE),
                Space::with_width(Length::Fill),
                text(S.get("app", "tagline")).size(12).color(Color::from_rgb(0.6, 0.65, 0.7)),
            ].align_y(Alignment::Center).padding([10, 20])
        ).width(Length::Fill).style(styles::header_container);

        let body = container(
            container(content).max_width(620).center_x(Length::Fill).padding(20)
        ).width(Length::Fill).height(Length::Fill).center_x(Length::Fill).center_y(Length::Fill);

        let footer = container(
            row![
                button(text(S.get("developer", "credit_label")).size(11).color(Color::from_rgb(0.5, 0.52, 0.56)))
                    .on_press(Message::ShowDeveloper)
                    .style(styles::ghost_button)
                    .padding([4, 8]),
                Space::with_width(Length::Fill),
                text(S.get("app", "version")).size(11).color(Color::from_rgb(0.6, 0.62, 0.66)),
            ].align_y(Alignment::Center).padding([6, 20])
        ).width(Length::Fill);

        container(column![header, body, footer])
            .width(Length::Fill).height(Length::Fill)
            .style(styles::main_container)
            .into()
    }

    // ========== Modals ==========

    fn view_about_modal<'a>(&self) -> Element<'a, Message> {
        let modal = container(
            column![
                row![
                    image(image::Handle::from_bytes(include_bytes!("../../assets/icons/rusticsort-64.png").as_slice()))
                        .width(Length::Fixed(64.0))
                        .height(Length::Fixed(64.0)),
                    Space::with_width(16),
                    column![
                        text(S.get("messages", "about_title")).size(20).color(Color::from_rgb(0.15, 0.15, 0.2)),
                        Space::with_height(8),
                        text(S.get("app", "name")).size(16).color(Color::from_rgb(0.18, 0.55, 0.82)),
                        text(S.get("app", "tagline")).size(13).color(Color::from_rgb(0.4, 0.42, 0.48)),
                    ]
                ].align_y(Alignment::Center),
                Space::with_height(14),
                text(S.get("app", "description")).size(12).color(Color::from_rgb(0.35, 0.37, 0.42)),
                Space::with_height(12),
                text(format!("Version: {}", S.get("app", "version"))).size(11).color(Color::from_rgb(0.5, 0.5, 0.55)),
                Space::with_height(14),
                row![
                    Space::with_width(Length::Fill),
                    button(text(S.get("buttons", "close")).size(13))
                        .on_press(Message::DismissModal)
                        .style(styles::primary_button)
                        .padding([8, 22]),
                ],
            ].spacing(2)
        ).max_width(420).padding(24).style(styles::modal_card);

        Self::modal_overlay(modal.into())
    }

    fn view_developer_modal<'a>(&self) -> Element<'a, Message> {
        let modal = container(
            column![
                text("Developer").size(20).color(Color::from_rgb(0.15, 0.15, 0.2)),
                Space::with_height(14),
                row![
                    text("Name: ").size(13).color(Color::from_rgb(0.4, 0.4, 0.5)),
                    text(S.get("developer", "name")).size(13).color(Color::from_rgb(0.15, 0.15, 0.2)),
                ],
                row![
                    text("Email: ").size(13).color(Color::from_rgb(0.4, 0.4, 0.5)),
                    text(S.get("developer", "email")).size(13).color(Color::from_rgb(0.18, 0.55, 0.82)),
                ],
                row![
                    text("Phone: ").size(13).color(Color::from_rgb(0.4, 0.4, 0.5)),
                    text(S.get("developer", "phone")).size(13).color(Color::from_rgb(0.15, 0.15, 0.2)),
                ],
                Space::with_height(14),
                row![
                    Space::with_width(Length::Fill),
                    button(text(S.get("buttons", "close")).size(13))
                        .on_press(Message::DismissModal)
                        .style(styles::primary_button)
                        .padding([8, 22]),
                ],
            ].spacing(4)
        ).max_width(380).padding(24).style(styles::modal_card);

        Self::modal_overlay(modal.into())
    }

    fn view_processing_modal<'a>(&self) -> Element<'a, Message> {
        let modal = container(
            column![
                text(S.get("messages", "organizing")).size(18).color(Color::from_rgb(0.15, 0.15, 0.2)),
                Space::with_height(8),
                text(S.get("messages", "organizing_wait")).size(13).color(Color::from_rgb(0.4, 0.4, 0.5)),
            ].align_x(Alignment::Center)
        ).max_width(360).padding(28).style(styles::modal_card);

        Self::modal_overlay(modal.into())
    }

    fn view_error_modal(error: &str) -> Element<'_, Message> {
        let modal = container(
            column![
                container(
                    text(S.get("messages", "error_title")).size(18).color(Color::from_rgb(0.75, 0.15, 0.15))
                ).padding([10, 14]).style(styles::error_card),
                Space::with_height(10),
                text(error).size(13).color(Color::from_rgb(0.35, 0.2, 0.2)),
                Space::with_height(14),
                row![
                    Space::with_width(Length::Fill),
                    button(text(S.get("buttons", "close")).size(13))
                        .on_press(Message::DismissError)
                        .style(styles::primary_button)
                        .padding([8, 22]),
                ],
            ].spacing(0)
        ).max_width(400).padding(24).style(styles::modal_card);

        Self::modal_overlay(modal.into())
    }

    fn modal_overlay(modal: Element<'_, Message>) -> Element<'_, Message> {
        container(
            container(modal).center_x(Length::Fill).center_y(Length::Fill)
        )
        .width(Length::Fill).height(Length::Fill)
        .style(styles::modal_overlay)
        .into()
    }
}
