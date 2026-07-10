use super::Gui;
use crate::message::Message;
use iced::widget::{
    button, column, container, image, row, scrollable, slider, space, text, text_editor, text_input,
};
use iced::{Alignment, Background, Color, ContentFit, Element, Length, Theme, Window};
use iced::widget::{responsive};

impl Gui {
    pub fn view(&self) -> Element<Message> {
        let input = text_input("choisir le dossier ...", &self.current_dir_path)
            .on_submit(Message::LoadPressed);
        let load_button = button("Charger").on_press(Message::ChooseFolder);

        let mut content = column![row![input, load_button]]
            .spacing(10)
            .height(Length::Fill);
        let mut ligne = row![scrollable(self.file_list()).width(250),];

        if let Some(handle) = &self.image_handle {
let (base_w, base_h) = match &self.image_original {
                    None => (0, 0),
                    Some(img) => (img.width(), img.height()),
                };

let image_area = responsive(|size| {
    let available_width = size.width;
    let available_height = size.height;

    let (base_w, base_h) = match &self.image_original {
        Some(img) => (img.width(), img.height()),
        None => (0, 0),
    };

    let fit_zoom = if base_w > 0 && base_h > 0 {
        let zx = available_width / base_w as f32;
        let zy = available_height / base_h as f32;
        zx.min(zy)
    } else {
        1.0
    };

    let displayed_w = base_w as f32 * fit_zoom * self.zoom;
    let displayed_h = base_h as f32 * fit_zoom * self.zoom;

    println!(
        "zone={}x{}, image={}x{}, zoom={}",
        available_width,
        available_height,
        displayed_w,
        displayed_h,
        self.zoom
    );
scrollable(
        image(handle.clone())
            .width(Length::Fixed(displayed_w))
            .height(Length::Fixed(displayed_h))
            .content_fit(ContentFit::Fill)
    )
    .direction(scrollable::Direction::Both {
        vertical: scrollable::Scrollbar::default(),
        horizontal: scrollable::Scrollbar::default(),
    })
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
});

            let rotate_controls = row![
                button(text("⟲").size(20))
                    .on_press(Message::RotateCcw)
                    .padding(10),
                button(text("⟳").size(20))
                    .on_press(Message::RotateCw)
                    .padding(10),
            ]
            .spacing(10);

            let zoom_controls = row![
                text("Zoom").size(14),
                slider(0.25..=3.0, self.zoom, Message::ZoomChanged)
                    .step(0.05)
                    .width(Length::Fixed(200.0)),
                text(format!("{:.0}%", self.zoom * 100.0)).size(14),
            ]
            .spacing(12)
            .align_y(Alignment::Center);

            let image_view = column![
                row![rotate_controls, zoom_controls],
                image_area.width(Length::Fill),
            ]
            .height(Length::Fill);

            ligne = ligne.push(image_view);
            if !self.current_row.is_empty() {
                ligne = ligne.push(self.excel_form());
            }
        } else {
            ligne = ligne.push(scrollable(self.excel_preview()));
        }

        ligne = ligne.height(Length::Fill);
        content = content.push(ligne);

        let status_bar = container(
            row![
                text(format!("v{}", env!("CARGO_PKG_VERSION"))),
                space::horizontal(), // pousse tout le reste à droite
                text(format!("Files: {}", self.files.capacity())),
                text(format!("Index: {}", self.current_index)),
                text(format!("Excel capacity: {}", self.excel_rows.capacity()))
            ]
            .spacing(20),
        )
        .padding(5)
        .width(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Color::from_rgb(0.9, 0.9, 0.9).into()),
            ..Default::default()
        })
        .align_y(iced::alignment::Vertical::Bottom);

        content = content.height(Length::Fill).push(status_bar);

        content.into()
    }

    /// Colonne cliquable listant les fichiers du dossier (images + xlsx),
    /// avec surbrillance du fichier sélectionné.
    fn file_list(&self) -> iced::widget::Column<'_, Message, Theme, iced::Renderer> {
        let mut file_list_column = column![];

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
            let this_file = file.0.clone();

            file_list_column =
                file_list_column.push(button(text(&file.0)).on_press(message).style(
                    move |theme: &Theme, status| {
                        let mut style = button::text(theme, status);
                        if current_file == this_file {
                            style.background =
                                Some(Background::Color(Color::from_rgb(0.2, 0.5, 0.9)));
                            style.text_color = Color::WHITE;
                        }
                        style
                    },
                ));
        }

        file_list_column
    }

    /// Formulaire d'édition de la ligne excel associée à l'image affichée.
    fn excel_form(&self) -> Element<'_, Message> {
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
            field_big("Contenu :", &self.content),
            row![space::horizontal(), button("reset"), button("save")].spacing(10)
        ]
        .padding(10)
        .spacing(8)
        .width(400)
        .into()
    }

    /// Aperçu brut des lignes excel quand aucune image n'est encore sélectionnée.
    fn excel_preview(&self) -> iced::widget::Column<'_, Message, Theme, iced::Renderer> {
        let mut excel_lines = column![];

        for row_data in &self.excel_rows {
            let line = row_data.iter().fold(row![].spacing(12), |r, cell| {
                r.push(text(cell).width(Length::Shrink))
            });
            excel_lines = excel_lines.push(scrollable(line));
        }

        excel_lines
    }
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

fn field_big<'a>(label: &'a str, value: &'a text_editor::Content) -> Element<'a, Message> {
    column![
        text(label).width(150),
        text_editor(value)
            .size(16)
            .height(Length::Fixed(6.0 * 22.0)),
    ]
    .spacing(5)
    .into()
}
