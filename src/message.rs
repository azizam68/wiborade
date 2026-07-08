use iced::widget::image;
use std::path::PathBuf;

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
    ExcelUpdated(Result<(), String>),
}
