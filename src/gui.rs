use iced::widget::{button, column, image, row, text_input, text_editor, text, scrollable};
use iced::{ContentFit, Element, Length, Task};
use std::fs;
use std::path::PathBuf;
use calamine::{ open_workbook, open_workbook_auto };
use calamine::{ Error, Xlsx, Reader, RangeDeserializerBuilder, Data};

pub struct Gui {
    image_handle: Option<image::Handle>,
    load_generation: u64,
    content: text_editor::Content,
    current_dir: Option<PathBuf>,
    current_dir_path: String,
    excel_rows: Vec<Vec<String>>,
    files: Vec<(String, bool)>,
}

impl Default for Gui {
    fn default() -> Self {
        Gui {
            image_handle: None,
            content: text_editor::Content::new(),
            load_generation: 0,
            current_dir: Some(std::env::home_dir().unwrap()),
            current_dir_path: "".into(),
            files: Vec::new(),
            excel_rows: Vec::new(),
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
        Some(ext) if ext.eq_ignore_ascii_case("xlsx") => Message::LoadXlsx(file.0.to_string()),
        _ => Message::ChangePicture(file.0.to_string()),
    };

    fileListColumn = fileListColumn.push(
        button(text(&file.0)).on_press(message),
    );
}

        ligne = ligne.push(scrollable(fileListColumn));

        if let Some(handle) = &self.image_handle {
            ligne = ligne.push(
                image(handle.clone())
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .content_fit(ContentFit::Contain),
            );
            ligne = ligne.push(
            column![
                field("Type :", "lettre"),
                field("Date :", "04/01?/1892"),
                field("Lieu :", "Clève"),
                field("Correspondant :", "E. Lisegang"),
                field("Note :", ""),
                field("Langue :", "allemand"),
                fieldBig("Contenu :", &self.content),
                row![
                    button("reset"),//.on_press(Message::Decrement),
                    button("save")//.on_press(Message::Decrement)
                ]
                .spacing(10)
            ]
            .padding(10)
            .spacing(8),
        );
        } else {
            let mut excel_lines: iced::widget::Column<'_, Message, iced::Theme, iced::Renderer> =
            column![];

             for row_data in &self.excel_rows {
                let line = row_data.iter().fold(
                    row![].spacing(12),
                    |r, cell| r.push(text(cell).width(Length::Shrink))
                );
                    excel_lines = excel_lines.push(scrollable(line));

            }

            ligne = ligne.push(scrollable(excel_lines));

        }

        content = content.push(ligne);
        content.into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ChooseFolder => Task::perform(
                async {
                    rfd::AsyncFileDialog::new()
                        .pick_folder()
                        .await
                        .map(|handle| handle.path().to_path_buf())
                },
                Message::FolderChosen,
            ),
            Message::LoadXlsx(xlsx_path) => {
                dbg!(&xlsx_path);
                self.image_handle = None;

                let res = load_xlsx_rows(&self.current_dir.clone().unwrap_or_default().join(&xlsx_path).to_string_lossy().to_string());
                self.excel_rows = res.unwrap();

                Task::none()
            }
            Message::ChangePicture(picture_path) => {


                        let dir = self.current_dir.clone();
                        let picture_path_complete = dir.unwrap_or_default().join(picture_path).to_string_lossy().to_string();

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
                        return Task::done(Message::LoadPressed);//Message::LoadPressed;
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

fn field<'a>(label: &'a str, value: &'a str) -> Element<'a, Message> {
    column![
        text(label).width(150),
        text_input("", value)
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

fn example() -> Result<(), Error> {
    let path = format!("{}/tests/temperature.xlsx", env!("CARGO_MANIFEST_DIR"));
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let range = workbook.worksheet_range("Sheet1")?;


    let mut iter = RangeDeserializerBuilder::new().from_range(&range)?;

    if let Some(result) = iter.next() {
        let (label, value): (String, f64) = result?;
        assert_eq!(label, "celsius");
        assert_eq!(value, 22.2222);
        Ok(())
    } else {
        Err(From::from("expected at least one record but got none"))
    }
}

fn load_xlsx_rows(path: &str) -> Result<Vec<Vec<String>>, String> {
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

    Ok(rows)
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