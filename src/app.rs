use iced::widget::{button, column, container, text};
use iced::{Element, Length, Task};

use crate::vault;

pub struct Model {
    pub vault_path: Option<std::path::PathBuf>,
    pub status_message: String,
    pub notes: Vec<vault::NoteEntry>,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenVault,
    VaultSelected(Option<std::path::PathBuf>),
    VaultScanned(Vec<vault::NoteEntry>),
}

pub fn new() -> Model {
    Model {
        vault_path: None,
        status_message: String::from("Welcome to RustyNotes. Open a vault to get started."),
        notes: Vec::new(),
    }
}

pub fn update(model: &mut Model, message: Message) -> Task<Message> {
    match message {
        Message::OpenVault => Task::perform(
            async {
                let handle = rfd::AsyncFileDialog::new()
                    .set_title("Open Vault")
                    .pick_folder()
                    .await;
                handle.map(|h| h.path().to_owned())
            },
            Message::VaultSelected,
        ),
        Message::VaultSelected(Some(path)) => {
            model.vault_path = Some(path.clone());
            model.status_message = format!("Scanning vault: {}", path.display());
            Task::perform(async move { vault::scan_vault(&path) }, Message::VaultScanned)
        }
        Message::VaultSelected(None) => Task::none(),
        Message::VaultScanned(notes) => {
            let count = notes.len();
            model.notes = notes;
            model.status_message = format!("Vault loaded: {} notes", count);
            Task::none()
        }
    }
}

pub fn view(model: &Model) -> Element<'_, Message> {
    let content = if model.vault_path.is_none() {
        column![
            text("RustyNotes").size(32),
            text(&model.status_message).size(16),
            button("Open Vault").on_press(Message::OpenVault),
        ]
        .spacing(20)
        .align_x(iced::Alignment::Center)
    } else {
        let note_list: String = model
            .notes
            .iter()
            .map(|n| format!("- {}", n.title))
            .collect::<Vec<_>>()
            .join("\n");

        column![
            text(format!("Vault: {}", model.vault_path.as_ref().unwrap().display())).size(14),
            text(&model.status_message).size(12),
            text("Notes:").size(16),
            text(note_list).size(12),
            button("Change Vault").on_press(Message::OpenVault),
        ]
        .spacing(10)
    };

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}
