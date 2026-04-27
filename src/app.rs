use std::collections::HashSet;
use std::path::PathBuf;

use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Element, Length, Padding, Task};
use iced_aw::widget::context_menu::ContextMenu;

use crate::config;
use crate::vault;

const SIDEBAR_WIDTH: f32 = 250.0;

pub struct Model {
    pub vault_path: Option<PathBuf>,
    pub status_message: String,
    pub notes: Vec<vault::NoteEntry>,
    pub file_tree: Vec<vault::TreeNode>,
    pub expanded_folders: HashSet<PathBuf>,
    pub selected_note: Option<PathBuf>,
    pub show_create_input: bool,
    pub create_is_folder: bool,
    pub create_name: String,
    pub rename_target: Option<PathBuf>,
    pub rename_input: String,
    pub recent_vaults: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenVault,
    OpenRecentVault(PathBuf),
    RemoveRecentVault(PathBuf),
    CloseVault,
    VaultSelected(Option<PathBuf>),
    VaultScanned(Vec<vault::NoteEntry>),

    ToggleFolder(PathBuf),
    SelectNote(PathBuf),

    ShowCreateNote,
    ShowCreateFolder,
    CreateNameChanged(String),
    ConfirmCreate,
    CancelCreate,

    StartRename(PathBuf),
    RenameInputChanged(String),
    ConfirmRename,
    CancelRename,

    DeleteNote(PathBuf),
}

pub fn new(recent_vaults: Vec<PathBuf>) -> Model {
    Model {
        vault_path: None,
        status_message: String::from("Welcome to RustyNotes. Open a vault to get started."),
        notes: Vec::new(),
        file_tree: Vec::new(),
        expanded_folders: HashSet::new(),
        selected_note: None,
        show_create_input: false,
        create_is_folder: false,
        create_name: String::new(),
        rename_target: None,
        rename_input: String::new(),
        recent_vaults,
    }
}

pub fn update(model: &mut Model, message: Message) -> Task<Message> {
    match message {
        Message::OpenVault => Task::perform(
            async {
                rfd::AsyncFileDialog::new()
                    .set_title("Open Vault")
                    .pick_folder()
                    .await
                    .map(|h| h.path().to_owned())
            },
            Message::VaultSelected,
        ),

        Message::OpenRecentVault(path) => {
            model.vault_path = Some(path.clone());
            model.status_message = format!("Scanning vault: {}", path.display());
            model.expanded_folders.clear();
            model.selected_note = None;

            add_to_recent(&mut model.recent_vaults, &path);
            config::save_recent_vaults(&model.recent_vaults);

            Task::perform(async move { vault::scan_vault(&path) }, Message::VaultScanned)
        }

        Message::VaultSelected(Some(path)) => {
            model.vault_path = Some(path.clone());
            model.status_message = format!("Scanning vault: {}", path.display());
            model.expanded_folders.clear();
            model.selected_note = None;

            add_to_recent(&mut model.recent_vaults, &path);
            config::save_recent_vaults(&model.recent_vaults);

            Task::perform(async move { vault::scan_vault(&path) }, Message::VaultScanned)
        }
        Message::VaultSelected(None) => Task::none(),

        Message::VaultScanned(notes) => {
            model.notes = notes;
            if let Some(vault_path) = &model.vault_path {
                model.file_tree = vault::build_file_tree(vault_path);
            }
            model.status_message = format!(
                "Vault loaded: {} notes | {} folders",
                model.notes.len(),
                count_dirs(&model.file_tree)
            );
            Task::none()
        }

        Message::ToggleFolder(path) => {
            if model.expanded_folders.contains(&path) {
                model.expanded_folders.remove(&path);
            } else {
                model.expanded_folders.insert(path);
            }
            Task::none()
        }

        Message::SelectNote(path) => {
            model.selected_note = Some(path);
            model.rename_target = None;
            model.show_create_input = false;
            Task::none()
        }

        Message::ShowCreateNote => {
            model.show_create_input = true;
            model.create_is_folder = false;
            model.create_name = String::from("untitled");
            Task::none()
        }

        Message::ShowCreateFolder => {
            model.show_create_input = true;
            model.create_is_folder = true;
            model.create_name = String::from("new-folder");
            Task::none()
        }

        Message::CreateNameChanged(name) => {
            model.create_name = name;
            Task::none()
        }

        Message::ConfirmCreate => {
            let name = std::mem::take(&mut model.create_name);
            let base_path = model.vault_path.clone().unwrap();
            let is_folder = model.create_is_folder;
            model.show_create_input = false;

            Task::perform(
                async move {
                    if is_folder {
                        let path = base_path.join(&name);
                        let _ = tokio::fs::create_dir(&path).await;
                    } else {
                        let filename = if name.ends_with(".md") {
                            name
                        } else {
                            format!("{}.md", name)
                        };
                        let path = base_path.join(&filename);
                        let _ = tokio::fs::write(&path, b"").await;
                    }
                    vault::scan_vault(&base_path)
                },
                Message::VaultScanned,
            )
        }

        Message::CancelCreate => {
            model.show_create_input = false;
            model.create_name.clear();
            Task::none()
        }

        Message::StartRename(path) => {
            let current = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            model.rename_target = Some(path.clone());
            model.rename_input = current;
            Task::none()
        }

        Message::RenameInputChanged(name) => {
            model.rename_input = name;
            Task::none()
        }

        Message::ConfirmRename => {
            if let Some(old_path) = model.rename_target.take() {
                let new_name = std::mem::take(&mut model.rename_input);
                let base_path = model.vault_path.clone().unwrap();

                Task::perform(
                    async move {
                        let new_path = old_path.with_file_name(format!("{}.md", new_name));
                        let _ = tokio::fs::rename(&old_path, &new_path).await;
                        vault::scan_vault(&base_path)
                    },
                    Message::VaultScanned,
                )
            } else {
                Task::none()
            }
        }

        Message::CancelRename => {
            model.rename_target = None;
            model.rename_input.clear();
            Task::none()
        }

        Message::DeleteNote(path) => {
            let base_path = model.vault_path.clone().unwrap();
            Task::perform(
                async move {
                    let _ = tokio::fs::remove_file(&path).await;
                    vault::scan_vault(&base_path)
                },
                Message::VaultScanned,
            )
        }

        Message::CloseVault => {
            model.vault_path = None;
            model.notes.clear();
            model.file_tree.clear();
            model.expanded_folders.clear();
            model.selected_note = None;
            model.status_message =
                String::from("Welcome to RustyNotes. Open a vault to get started.");
            Task::none()
        }

        Message::RemoveRecentVault(path) => {
            model.recent_vaults.retain(|p| p != &path);
            config::save_recent_vaults(&model.recent_vaults);
            Task::none()
        }
    }
}

pub fn view(model: &Model) -> Element<'_, Message> {
    if model.vault_path.is_none() {
        build_welcome_screen(model)
    } else {
        let sidebar = build_sidebar(model);
        let center = build_center_panel(model);

        row![
            container(sidebar).width(SIDEBAR_WIDTH).height(Length::Fill),
            container(center).width(Length::Fill).height(Length::Fill),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

fn build_welcome_screen(model: &Model) -> Element<'_, Message> {
    let mut content = column![
        text("RustyNotes").size(32),
        text(&model.status_message).size(16),
        button("Open Vault").on_press(Message::OpenVault),
    ]
    .spacing(20)
    .align_x(iced::Alignment::Center);

    if !model.recent_vaults.is_empty() {
        let mut recent_column = column![text("Recent vaults:").size(18)].spacing(8);

        for path in &model.recent_vaults {
            let vault_name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let path_display = path.display().to_string();

            let row = row![
                column![
                    text(vault_name).size(14),
                    text(path_display).size(10),
                ]
                .spacing(2)
                .width(Length::Fill),
                button("Open")
                    .on_press(Message::OpenRecentVault(path.clone()))
                    .padding(4),
                button("✕")
                    .on_press(Message::RemoveRecentVault(path.clone()))
                    .padding(4),
            ]
            .spacing(8)
            .align_y(iced::Alignment::Center);

            recent_column = recent_column.push(row);
        }

        content = content.push(recent_column.width(500));
    }

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

fn build_sidebar(model: &Model) -> Element<'_, Message> {
    let header = column![
        row![
            button("+ Note")
                .on_press(Message::ShowCreateNote)
                .padding(4),
            button("+ Folder")
                .on_press(Message::ShowCreateFolder)
                .padding(4),
        ]
        .spacing(4),
        row![
            text(&model.status_message).size(11).width(Length::Fill),
            button("✕").on_press(Message::CloseVault).padding(2),
        ]
        .spacing(4),
    ]
    .spacing(4)
    .padding(4);

    let create_row: Element<'_, Message> = if model.show_create_input {
        let label = if model.create_is_folder {
            "Folder name:"
        } else {
            "Note name:"
        };
        column![
            text(label).size(12),
            text_input("name", &model.create_name)
                .on_input(Message::CreateNameChanged)
                .on_submit(Message::ConfirmCreate)
                .padding(4),
            row![
                button("Create").on_press(Message::ConfirmCreate).padding(4),
                button("Cancel").on_press(Message::CancelCreate).padding(4),
            ]
            .spacing(4),
        ]
        .spacing(4)
        .padding(4)
        .into()
    } else {
        text("").into()
    };

    let mut tree_elements: Vec<Element<'_, Message>> = Vec::new();
    collect_tree_elements(&model.file_tree, 0, model, &mut tree_elements);

    let tree_scroll = scrollable(column(tree_elements).spacing(1));

    let tree_section = column![tree_scroll].height(Length::Fill).padding(4);

    let action_bar: Element<'_, Message> = if let Some(selected) = &model.selected_note {
        let note_name = selected
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        if model.rename_target.is_some() {
            column![
                text(format!("Renaming: {}", note_name)).size(12),
                text_input("new name", &model.rename_input)
                    .on_input(Message::RenameInputChanged)
                    .on_submit(Message::ConfirmRename)
                    .padding(4),
                row![
                    button("Rename").on_press(Message::ConfirmRename).padding(4),
                    button("Cancel").on_press(Message::CancelRename).padding(4),
                ]
                .spacing(4),
            ]
            .spacing(4)
            .padding(4)
            .into()
        } else {
            column![
                text(format!("Selected: {}", note_name)).size(12),
                row![
                    button("Rename")
                        .on_press(Message::StartRename(selected.clone()))
                        .padding(4),
                    button("Delete")
                        .on_press(Message::DeleteNote(selected.clone()))
                        .padding(4),
                ]
                .spacing(4),
            ]
            .spacing(4)
            .padding(4)
            .into()
        }
    } else {
        text("").into()
    };

    container(column![header, create_row, tree_section, action_bar])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn build_center_panel(model: &Model) -> Element<'_, Message> {
    if let Some(selected) = &model.selected_note {
        let name = selected
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let note = model.notes.iter().find(|n| n.path == *selected);

        let tags = note
            .map(|n| {
                if n.tags.is_empty() {
                    "No tags".to_string()
                } else {
                    n.tags.join(", ")
                }
            })
            .unwrap_or_default();

        let links = note
            .map(|n| {
                if n.wikilinks.is_empty() {
                    "No wikilinks".to_string()
                } else {
                    n.wikilinks.join(", ")
                }
            })
            .unwrap_or_default();

        container(
            column![
                text(name).size(24),
                text(format!("Path: {}", selected.display())).size(12),
                text(format!("Tags: {}", tags)).size(14),
                text(format!("Wikilinks: {}", links)).size(14),
            ]
            .spacing(8)
            .padding(20),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    } else {
        container(
            column![
                text("Select a note to view details").size(16),
                text("Use the file explorer on the left.").size(14),
            ]
            .spacing(8)
            .padding(20),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }
}

fn collect_tree_elements<'a>(
    nodes: &'a [vault::TreeNode],
    depth: usize,
    model: &Model,
    elements: &mut Vec<Element<'a, Message>>,
) {
    for node in nodes {
        let indent = depth as f32 * 16.0;

        if node.is_dir {
            let arrow = if model.expanded_folders.contains(&node.path) {
                "▼"
            } else {
                "▶"
            };
            elements.push(
                button(
                    row![text(arrow).size(11), text(&node.name).size(13)]
                        .spacing(3)
                        .padding(Padding::new(2.0).left(indent + 4.0)),
                )
                .on_press(Message::ToggleFolder(node.path.clone()))
                .padding(0)
                .style(iced::widget::button::text)
                .into(),
            );

            if model.expanded_folders.contains(&node.path) {
                collect_tree_elements(&node.children, depth + 1, model, elements);
            }
        } else {
            let is_selected = model.selected_note.as_ref() == Some(&node.path);
            let display_name = node
                .path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            let btn = button(
                row![text("◆").size(10), text(display_name).size(13)]
                    .spacing(4)
                    .padding(Padding::new(2.0).left(indent + 4.0)),
            )
            .on_press(Message::SelectNote(node.path.clone()))
            .padding(0);

            let btn = if is_selected {
                btn.style(iced::widget::button::primary)
            } else {
                btn.style(iced::widget::button::text)
            };

            let path = node.path.clone();
            let cm = ContextMenu::new(
                btn,
                move || {
                    iced::widget::column![
                        iced::widget::button("Rename")
                            .on_press(Message::StartRename(path.clone())),
                        iced::widget::button("Delete")
                            .on_press(Message::DeleteNote(path.clone())),
                    ]
                    .spacing(2)
                    .padding(4)
                    .into()
                },
            );

            elements.push(cm.into());
        }
    }
}

fn count_dirs(nodes: &[vault::TreeNode]) -> usize {
    let mut count = 0;
    for node in nodes {
        if node.is_dir {
            count += 1 + count_dirs(&node.children);
        }
    }
    count
}

fn add_to_recent(recent: &mut Vec<PathBuf>, path: &PathBuf) {
    recent.retain(|p| p != path);
    recent.insert(0, path.clone());
    recent.truncate(10);
}
