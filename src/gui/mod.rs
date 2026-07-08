mod update;
mod view;
mod subscription;

use iced::widget::{image, text_editor};
use std::path::PathBuf;
use std::time::SystemTime;

pub struct Gui {
    image_handle: Option<image::Handle>,
    load_generation: u64,
    content: text_editor::Content,
    current_dir: Option<PathBuf>,
    current_index: i32,
    current_index_str: String,
    current_dir_path: String,
    last_modified: Option<SystemTime>,
    excel_rows: Vec<Vec<String>>,
    files: Vec<(String, bool)>,
    selected_file: String,
    excel_opened_file: String,
    current_row: Vec<String>,
}

impl Default for Gui {
    fn default() -> Self {
        Gui {
            image_handle: None,
            content: text_editor::Content::new(),
            current_index: -1,
            current_index_str: String::new(),
            load_generation: 0,
            current_dir: std::env::home_dir(),
            current_dir_path: String::new(),
            files: Vec::new(),
            last_modified: None,
            excel_rows: Vec::new(),
            selected_file: String::new(),
            excel_opened_file: String::new(),
            current_row: Vec::new(),
        }
    }
}

impl Gui {
    /// Retrouve la ligne excel correspondant à un fichier image donné,
    /// et met à jour l'index courant au passage.
    fn get_row_for_file(&mut self, filename: &str) -> Result<&Vec<String>, String> {
        let index = self
            .files
            .iter()
            .position(|(name, _)| name == filename)
            .ok_or_else(|| format!("Fichier '{filename}' introuvable dans files"))?;

        let row_index = index - 1;

        self.current_index = index as i32;
        self.current_index_str = index.to_string();

        self.excel_rows
            .get(row_index)
            .ok_or_else(|| format!("Aucune ligne à l'index {row_index} dans excel_rows"))
    }
}
