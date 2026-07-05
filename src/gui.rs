use calamine::{Data, Error, RangeDeserializerBuilder, Reader, Xlsx};
use calamine::{open_workbook, open_workbook_auto};
use iced::widget::text_editor::Content;
use iced::widget::{button, column, image, row, scrollable, text, text_editor, text_input, space};
use iced::{Background, Color, ContentFit, Element, Length, Task, Theme};
use std::fs;
use std::path::PathBuf;
use std::ptr::null_mut;
use std::time::SystemTime;
use umya_spreadsheet::*;

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
            current_index_str: "".into(),
            load_generation: 0,
            current_dir: Some(std::env::home_dir().unwrap()),
            current_dir_path: "".into(),
            files: Vec::new(),
            last_modified: None,
            excel_rows: Vec::new(),
            selected_file: "".into(),
            excel_opened_file: "".into(),
            current_row: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    LoadPressed,
    ImageStageLoaded(u64, Option<image::Handle>),
    ChooseFolder,
    FolderChosen(Option<PathBuf>),
    ChangePicture(String),
    LoadXlsx(String),
    ExcelUpdate(usize, String),
    ExcelSubmit,
    ExcelUpdated(Result<(), String>)
}

#[derive(Debug)]
enum LoadResult {
    Unchanged,
    Loaded {
        rows: Vec<Vec<String>>,
        modified: SystemTime,
    },
}

impl Gui {
    pub fn view(&self) -> Element<Message> {
        let input = text_input("choisir le dossier ...", &self.current_dir_path)
            .on_submit(Message::LoadPressed);

        let load_button = button("Charger").on_press(Message::ChooseFolder);

        let mut content = column![row![input, load_button]].spacing(10);

        let mut ligne = row![];

        let mut fileListColumn: iced::widget::Column<'_, Message, iced::Theme, iced::Renderer> =
            column![];

        for file in &self.files {
            let message = match std::path::Path::new(&file.0)
                .extension()
                .and_then(|ext| ext.to_str())
            {
                Some(ext) if ext.eq_ignore_ascii_case("xlsx") => {
                    Message::LoadXlsx(file.0.to_string())
                }
                _ => Message::ChangePicture(file.0.to_string()),
            };

            let current_file = self.selected_file.clone();

            fileListColumn = fileListColumn.push(button(text(&file.0)).on_press(message).style(
                move |theme: &Theme, status| {
                    let mut style = button::text(theme, status);
                    if current_file == file.0 {
                        style.background = Some(Background::Color(Color::from_rgb(0.2, 0.5, 0.9)));
                        style.text_color = Color::WHITE;
                    }
                    style
                },
            ));
        }

        ligne = ligne.push(scrollable(fileListColumn));

        if let Some(handle) = &self.image_handle {
            ligne = ligne.push(
                image(handle.clone())
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .content_fit(ContentFit::Contain),
            );
            if self.current_row.len() > 0 {
                ligne = ligne.push(
                    column![
                        row![
                            field("Ligne Excel :", &self.current_index_str, 0),
                            field("Page :", &self.current_row[6], 7),
                        ],
                        field("Type :", &self.current_row[0], 1),
                        field("Date :", &self.current_row[1], 2),
                        field("Lieu :", &self.current_row[2], 3),
                        field("Correspondant :", &self.current_row[3], 4),
                        field("Note :", &self.current_row[4], 5),
                        field("Langue :", &self.current_row[5], 6),
                        fieldBig("Contenu :", &self.content),
                        row![
                            space::horizontal(),
                            button("reset"), //.on_press(Message::Decrement),
                            button("save")   //.on_press(Message::Decrement)
                        ]
                        .spacing(10)
                    ]
                    .padding(10)
                    .spacing(8),
                );
            }
        } else {
            let mut excel_lines: iced::widget::Column<'_, Message, iced::Theme, iced::Renderer> =
                column![];

            for row_data in &self.excel_rows {
                let line = row_data.iter().fold(row![].spacing(12), |r, cell| {
                    r.push(text(cell).width(Length::Shrink))
                });
                excel_lines = excel_lines.push(scrollable(line));
            }

            ligne = ligne.push(scrollable(excel_lines));
        }

        content = content.push(ligne);
        content.into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ExcelUpdate(index, new_val) => {
                if index > 0 {
                    self.current_row[index - 1] = new_val;
                }
                Task::none()
            }
            Message::ExcelSubmit => {
                if self.current_row.clone().get(6) == self.excel_rows.get((self.current_index-1) as usize).unwrap().get(6)
                {
                    println!("rien a modifier dans le excel");
                    Task::none()
                }
                else {
                    println!("modifier le excel");
                    dbg!(self.current_row.clone().get(6).unwrap());
                    dbg!(self.excel_rows.get(self.current_index as usize).unwrap().get(6).unwrap());

                let output_path = self.excel_opened_file.clone();
                let row_index = self.current_index_str.parse().unwrap_or(0); // à adapter selon ton state
                let values = self.current_row.clone();

                Task::perform(
                    async move {
                        insert_row_in_excel(&output_path, row_index, values)
                            .map_err(|e| e.to_string())
                    },
                    Message::ExcelUpdated,
                )
            }
            }
            Message::ExcelUpdated(result) => {
                println!("done with message");
                Task::none()
            }
            Message::ChooseFolder => Task::perform(
                async 
                {
                    rfd::AsyncFileDialog::new()
                        .pick_folder()
                        .await
                        .map(|handle| handle.path().to_path_buf())
                }, Message::FolderChosen),
            Message::LoadXlsx(xlsx_path) => {
                self.image_handle = None;
                self.selected_file = self
                    .current_dir
                    .clone()
                    .unwrap_or_default()
                    .join(&xlsx_path)
                    .to_string_lossy()
                    .to_string();

                match load_xlsx_rows(&self.selected_file, self.last_modified) {
                    Ok(LoadResult::Unchanged) => {
                        // rien à faire, on garde les données actuelles
                        println!("Fichier inchangé, pas de rechargement");
                    }
                    Ok(LoadResult::Loaded { rows, modified }) => {
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
            Message::ChangePicture(picture_path) => {
                dbg!(&picture_path);

                match self.get_row_for_file(&picture_path) {
                    Ok(row) => {
                        let mut current_row = row.clone(); // la référence n'est plus nécessaire après




                        if current_row.get(6)==None {
                            current_row.push(String::from("1"));
                        }


                        self.current_row = current_row;

                        
                        println!("Ligne trouvée: {:?}", self.current_row);

                        self.content = Content::with_text(&self.current_row[6]);
                    }
                    Err(e) => {
                        eprintln!("Erreur: {e}");
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

                let path_medium = picture_path_complete.clone();
                let path_full = picture_path_complete.clone();

                self.load_generation += 1;
                let generation = self.load_generation;

                Task::batch([
                    Task::perform(
                        async move { (generation, medium_quality(&path_medium)) },
                        |(g, h)| Message::ImageStageLoaded(g, h),
                    ),
                    Task::perform(
                        async move { (generation, full_quality(&path_full)) },
                        |(g, h)| Message::ImageStageLoaded(g, h),
                    ),
                ])
            }
            Message::FolderChosen(folder) => {
                match folder {
                    Some(dir) => {
                        self.current_dir = Some(dir.clone());
                        self.current_dir_path = dir.to_string_lossy().to_string();
                        self.files = get_files(&self.current_dir.as_ref().unwrap());
                        dbg!(&self.current_dir_path);
                        return Task::done(Message::LoadPressed); //Message::LoadPressed;
                    }
                    None => {
                        self.current_dir = std::env::home_dir();
                    }
                }

                Task::none()
            }
            Message::LoadPressed => {
                self.load_generation += 1;
                let generation = self.load_generation;

                if let Some(current_dir) = &self.current_dir {
                    let picture_path = self
                        .files
                        .iter()
                        .find(|(name, _)| {
                            matches!(
                                std::path::Path::new(name)
                                    .extension()
                                    .and_then(|e| e.to_str())
                                    .map(|e| e.to_ascii_lowercase())
                                    .as_deref(),
                                Some(
                                    "jpg"
                                        | "jpeg"
                                        | "png"
                                        | "bmp"
                                        | "gif"
                                        | "webp"
                                        | "tif"
                                        | "tiff"
                                )
                            )
                        })
                        .and_then(|(name, _)| {
                            current_dir.join(name).to_str().map(ToOwned::to_owned)
                        })
                        .unwrap_or_default();

                    let path_exif = picture_path.clone();
                    let path_medium = picture_path.clone();
                    let path_full = picture_path.clone();

                    Task::batch([
                        Task::perform(
                            async move { (generation, medium_quality(&path_medium)) },
                            |(g, h)| Message::ImageStageLoaded(g, h),
                        ),
                        Task::perform(
                            async move { (generation, full_quality(&path_full)) },
                            |(g, h)| Message::ImageStageLoaded(g, h),
                        ),
                    ])
                } else {
                    Task::none()
                }
            }
            Message::ImageStageLoaded(generation, handle) => {
                if generation == self.load_generation && handle.is_some() {
                    self.image_handle = handle;
                }
                Task::none()
            }
        }
    }

    pub fn get_row_for_file(&mut self, filename: &str) -> Result<&Vec<String>, String> {
        let index = self
            .files
            .iter()
            .position(|(name, _)| name == filename)
            .ok_or_else(|| format!("Fichier '{filename}' introuvable dans files"))?;

        let row_index = index - 1;

        self.current_index = index as i32; // i32::try_from(x).unwrap();
        self.current_index_str = index.to_string();

        self.excel_rows
            .get(row_index)
            .ok_or_else(|| format!("Aucune ligne à l'index {row_index} dans excel_rows"))
    }
}

fn exif_thumbnail(path: &str) -> Option<image::Handle> {
    use exif::{In, Reader, Tag};
    use std::io::BufReader;

    let file = std::fs::File::open(path).ok()?;
    let mut reader = BufReader::new(file);
    let exif_data = Reader::new().read_from_container(&mut reader).ok()?;

    let offset = exif_data
        .get_field(Tag::JPEGInterchangeFormat, In::THUMBNAIL)?
        .value
        .get_uint(0)? as usize;
    let length = exif_data
        .get_field(Tag::JPEGInterchangeFormatLength, In::THUMBNAIL)?
        .value
        .get_uint(0)? as usize;

    let buf = exif_data.buf();
    let thumb_bytes = buf.get(offset..offset + length)?;

    let decoded = ::image::load_from_memory(thumb_bytes).ok()?;
    let rgba = decoded.to_rgba8();
    let (width, height) = (rgba.width(), rgba.height());
    Some(image::Handle::from_rgba(width, height, rgba.into_raw()))
}

fn medium_quality(path: &str) -> Option<image::Handle> {
    let img = ::image::open(path).ok()?;
    let resized = img.thumbnail(1000, 800);
    let rgba = resized.to_rgba8();
    let (width, height) = (rgba.width(), rgba.height());
    Some(image::Handle::from_rgba(width, height, rgba.into_raw()))
}

fn full_quality(path: &str) -> Option<image::Handle> {
    Some(image::Handle::from_path(path))
}

fn get_files(path: &PathBuf) -> Vec<(String, bool)> {
    let mut pics = Vec::default();
    let mut xlsxes = Vec::default();

    if let Ok(read_dir) = fs::read_dir(path) {
        for read in read_dir {
            if let Ok(dir_entry) = read {
                if let Some(name) = dir_entry.file_name().to_str() {
                    if name.ends_with("xlsx") {
                        xlsxes.push((name.to_string(), true));
                    } else if name.ends_with("jpg") {
                        pics.push((name.to_string(), false));
                    }
                }
            }
        }
    }

    pics.sort_by(|a, b| a.0.cmp(&b.0));
    xlsxes.append(&mut pics);
    xlsxes
}

fn field<'a>(label: &'a str, value: &'a str, index: usize) -> Element<'a, Message> {
    column![
        text(label).width(150),
        text_input("", value)
            .on_input(move |new_val| Message::ExcelUpdate(index, new_val))
            .on_submit(Message::ExcelSubmit)
            .width(Length::Fill),
    ]
    .spacing(5)
    .into()
}

fn fieldBig<'a>(label: &'a str, value: &'a text_editor::Content) -> Element<'a, Message> {
    column![
        text(label).width(150),
        text_editor(value)
            .size(16)
            .height(Length::Fixed(6.0 * 22.0)),
    ]
    .spacing(5)
    .into()
}

fn load_xlsx_rows(path: &str, last_modified: Option<SystemTime>) -> Result<LoadResult, String> {
    let metadata = fs::metadata(path).map_err(|e| e.to_string())?;
    let modified = metadata.modified().map_err(|e| e.to_string())?;

    // Early return si rien n'a changé
    if Some(modified) == last_modified {
        return Ok(LoadResult::Unchanged);
    }

    let mut workbook = open_workbook_auto(path).map_err(|e| e.to_string())?;

    let sheet_name = workbook
        .sheet_names()
        .first()
        .cloned()
        .ok_or_else(|| "Aucune feuille trouvée".to_string())?;

    let range = workbook
        .worksheet_range(&sheet_name)
        .map_err(|e| e.to_string())?;

    let rows = range
        .rows()
        .map(|row| row.iter().map(cell_to_string).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    Ok(LoadResult::Loaded { rows, modified })
}

fn cell_to_string(cell: &Data) -> String {
    match cell {
        Data::Empty => String::new(),
        Data::String(s) => s.clone(),
        Data::Float(f) => f.to_string(),
        Data::Int(i) => i.to_string(),
        Data::Bool(b) => b.to_string(),
        Data::DateTime(dt) => dt.to_string(),
        Data::DateTimeIso(s) => s.clone(),
        Data::DurationIso(s) => s.clone(),
        Data::Error(e) => format!("Erreur: {e:?}"),
    }
}

fn insert_row_in_excel(file_path: &str,
    //    sheet_name: &str,
    row_index: u32,
    values: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {

    let mut book = reader::xlsx::read(file_path)?;

    let sheet_name = book.sheet(0)?.name().to_string();
    let sheet = book.sheet_by_name_mut(&sheet_name)?;

    sheet.insert_new_row(row_index, 1);

    for (col_index, value) in values.iter().enumerate() {
        let col = (col_index + 1) as u32;
        if col_index == 6 {
            sheet.cell_mut((col, row_index)).set_value(value);
        }
    }

    writer::xlsx::write(&book, file_path)?;


    Ok(())
}
