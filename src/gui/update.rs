use super::Gui;
use crate::message::Message;
use crate::{excel, files, image_ops};
use iced::widget::image;
use iced::widget::operation;
use iced::widget::text_editor::Content;
use iced::Task;
use std::path::PathBuf;

impl Gui {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::RotateCw => {
                self.angle = (self.angle + 90) % 360;
                self.refresh_handle();
                Task::none()
            }
            Message::RotateCcw => {
                self.angle = (self.angle + 270) % 360; // -90 mod 360
                self.refresh_handle();
                Task::none()
            }
            Message::ZoomChanged(z) => {
                self.zoom = z;
                Task::none()
            }
            Message::ExcelUpdate(index, new_val) => self.on_excel_update(index, new_val),
            Message::ExcelSubmit => self.on_excel_submit(),
            Message::ExcelUpdated(result) => self.on_excel_updated(result),
            Message::FocusNext => operation::focus_next(),
            Message::FocusPrevious => operation::focus_previous(),
            Message::ChooseFolder => self.on_choose_folder(),
            Message::LoadXlsx(xlsx_path) => self.on_load_xlsx(xlsx_path),
            Message::ChangePicture(picture_path) => self.on_change_picture(picture_path),
            Message::FolderChosen(folder) => self.on_folder_chosen(folder),
            Message::LoadPressed => self.on_load_pressed(),
            Message::ImageStageLoaded(generation, handle) => {
                self.on_image_stage_loaded(generation, handle)
            }
        }
    }

    fn on_excel_update(&mut self, index: usize, new_val: String) -> Task<Message> {
        if index > 0 {
            self.current_row[index - 1] = new_val;
        }
        Task::none()
    }

    fn on_excel_submit(&mut self) -> Task<Message> {
        let idx = self.current_index as usize;

        if idx <= self.excel_rows.len() {
            self.excel_rows[idx - 1] = self.current_row.clone();
        } else {
            self.excel_rows.push(self.current_row.clone());
        }

        // NOTE: dans l'original ce test faisait un `.unwrap()` sur
        // `excel_rows.get((current_index - 1) as usize)`, ce qui pouvait paniquer
        // si l'index était hors bornes. Ici on repasse par `.and_then` pour
        // rester safe (à vérifier que le comportement logique reste correct
        // pour ton cas d'usage).
        let previous_page = self
            .excel_rows
            .get((self.current_index - 1) as usize)
            .and_then(|row| row.get(6));

        let current_page = self.current_row.get(6).map(String::as_str);

        let should_insert =
            current_page.map_or(false, |p| !p.is_empty()) && current_page != Some("1")
                && current_page != previous_page.map(String::as_str);

        let output_path = self.excel_opened_file.clone();
        let row_index: usize = self.current_index_str.parse().unwrap_or(0);
        let values = self.current_row.clone();

        if should_insert {
            println!("insert dans le excel");
            Task::perform(
                async move {
                    excel::insert_row(&output_path, row_index as u32, values)
                        .map_err(|e| e.to_string())
                },
                Message::ExcelUpdated,
            )
        } else {
            println!(
                "modifier le excel ligne {row_index} de {}",
                self.excel_opened_file
            );
            Task::perform(
                async move {
                    excel::update_row(&output_path, row_index, values).map_err(|e| e.to_string())
                },
                Message::ExcelUpdated,
            )
        }
    }

    fn on_excel_updated(&mut self, _result: Result<(), String>) -> Task<Message> {
        println!("done with message");
        Task::none()
    }

    fn on_choose_folder(&self) -> Task<Message> {
        Task::perform(
            async {
                rfd::AsyncFileDialog::new()
                    .pick_folder()
                    .await
                    .map(|handle| handle.path().to_path_buf())
            },
            Message::FolderChosen,
        )
    }

    fn on_load_xlsx(&mut self, xlsx_path: String) -> Task<Message> {
        self.image_handle = None;
        self.selected_file = self
            .current_dir
            .clone()
            .unwrap_or_default()
            .join(&xlsx_path)
            .to_string_lossy()
            .to_string();

        match excel::load_rows(&self.selected_file, self.last_modified) {
            Ok(excel::LoadResult::Unchanged) => {
                println!("Fichier inchangé, pas de rechargement");
            }
            Ok(excel::LoadResult::Loaded { rows, modified }) => {
                self.excel_rows = rows;
                self.last_modified = Some(modified);
                self.excel_opened_file = self.selected_file.clone();
            }
            Err(e) => {
                eprintln!("Erreur: {e}");
            }
        }

        Task::none()
    }
fn refresh_handle(&mut self) {
        let rotated = match self.angle {
            90  => self.image_original.as_ref().unwrap().rotate90(),
            180 => self.image_original.as_ref().unwrap().rotate180(),
            270 => self.image_original.as_ref().unwrap().rotate270(),
            _   => self.image_original.as_ref().unwrap().clone(),
        };
        self.image_handle = to_handle(&rotated);
    }

    /// Utile plus tard pour écrire le tag EXIF Orientation
    pub fn exif_orientation(&self) -> u16 {
        match self.angle {
            90  => 6,
            180 => 3,
            270 => 8,
            _   => 1,
        }
    }
    fn on_change_picture(&mut self, picture_path: String) -> Task<Message> {
        dbg!(&picture_path);

        match self.get_row_for_file(&picture_path) {
            Ok(row) => {
                let mut current_row = row.clone();

                if current_row.get(6).is_none() {
                    current_row.push(String::from("1"));
                }

                self.current_row = current_row;
                println!("Ligne trouvée: {:?}", self.current_row);
                self.content = Content::with_text(&self.current_row[6]);
            }
            Err(e) => {
                eprintln!("Erreur: {e}");
                self.current_row = vec![String::new(); 7];
            }
        }

        self.image_handle = None;
        self.selected_file = picture_path.clone();

        let dir = self.current_dir.clone();
        let picture_path_complete = dir
            .unwrap_or_default()
            .join(picture_path)
            .to_string_lossy()
            .to_string();

        self.load_generation += 1;
        let generation = self.load_generation;

        load_image_task(generation, picture_path_complete)
    }

    fn on_folder_chosen(&mut self, folder: Option<PathBuf>) -> Task<Message> {
        match folder {
            Some(dir) => {
                self.current_dir = Some(dir.clone());
                self.current_dir_path = dir.to_string_lossy().to_string();
                self.files = files::list(&dir);
                dbg!(&self.current_dir_path);
                return Task::done(Message::LoadPressed);
            }
            None => {
                self.current_dir = std::env::home_dir();
            }
        }

        Task::none()
    }

    fn on_load_pressed(&mut self) -> Task<Message> {
        self.load_generation += 1;
        let generation = self.load_generation;

        let Some(current_dir) = &self.current_dir else {
            return Task::none();
        };

        let picture_path = self
            .files
            .iter()
            .find(|(name, _)| is_supported_image(name))
            .and_then(|(name, _)| current_dir.join(name).to_str().map(ToOwned::to_owned))
            .unwrap_or_default();

        load_image_task(generation, picture_path)
    }

    fn on_image_stage_loaded(
        &mut self,
        generation: u64,
        handle: Option<image::Handle>,
    ) -> Task<Message> {
        if generation == self.load_generation && handle.is_some() {
            self.image_handle = handle;
        }
        Task::none()
    }
}

/// Charge une image en 2 passes (qualité moyenne puis pleine qualité)
/// et envoie chaque étape comme un message séparé.
fn load_image_task(generation: u64, path: String) -> Task<Message> {
    let path_medium = path.clone();
    let path_full = path;

    Task::batch([
        Task::perform(
            async move { (generation, image_ops::medium_quality(&path_medium)) },
            |(g, h)| Message::ImageStageLoaded(g, h),
        ),
        Task::perform(
            async move { (generation, image_ops::full_quality(&path_full)) },
            |(g, h)| Message::ImageStageLoaded(g, h),
        ),
    ])
}

fn is_supported_image(name: &str) -> bool {
    matches!(
        std::path::Path::new(name)
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase())
            .as_deref(),
        Some("jpg" | "jpeg" | "png" | "bmp" | "gif" | "webp" | "tif" | "tiff")
    )
}
fn to_handle(img: &::image::DynamicImage) -> Option<image::Handle> {
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    Some(image::Handle::from_rgba(w, h, rgba.into_raw()))
}