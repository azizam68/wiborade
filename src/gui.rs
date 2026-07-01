use iced::widget::{button, column, image, row, text_input};
use iced::{ContentFit, Element, Length, Task};
use std::fs;
use std::path::PathBuf;

pub struct Gui {
    image_handle: Option<image::Handle>,
    load_generation: u64,
    current_dir: Option<PathBuf>,
    current_dir_path: String,
    files: Vec<(String, bool)>,
}

impl Default for Gui {
    fn default() -> Self {
        Gui {
            image_handle: None,
            load_generation: 0,
            current_dir: Some(std::env::home_dir().unwrap()),
            current_dir_path: "".into(),
            files: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    LoadPressed,
    ImageStageLoaded(u64, Option<image::Handle>),
    ChooseFolder,
    FolderChosen(Option<PathBuf>),
}

impl Gui {
    pub fn view(&self) -> Element<Message> {
        let input = text_input("choisir le dossier ...", &self.current_dir_path)
            .on_submit(Message::LoadPressed);

        let load_button = button("Charger").on_press(Message::ChooseFolder);

        let mut content = column![row![input, load_button]].spacing(10);

        if let Some(handle) = &self.image_handle {
            content = content.push(
                image(handle.clone())
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .content_fit(ContentFit::Contain),
            );
        }

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

dbg!(&path_medium);

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
                }else{Task::none()}

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
/*
/home/azizam/Desktop/DrCeresato/vol_80/IMG_20260520_160458.jpg

*/
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
