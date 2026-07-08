use iced::widget::image;

/// Version redimensionnée, utilisée pour un affichage rapide.
pub fn medium_quality(path: &str) -> Option<image::Handle> {
    let img = ::image::open(path).ok()?;
    let resized = img.thumbnail(1000, 800);
    let rgba = resized.to_rgba8();
    let (width, height) = (rgba.width(), rgba.height());
    Some(image::Handle::from_rgba(width, height, rgba.into_raw()))
}

/// Version pleine résolution, chargée en second pour ne pas bloquer l'UI.
pub fn full_quality(path: &str) -> Option<image::Handle> {
    Some(image::Handle::from_path(path))
}

/// Miniature embarquée dans les métadonnées EXIF du fichier (non utilisée pour
/// l'instant dans update.rs, gardée disponible si besoin plus tard).
#[allow(dead_code)]
pub fn exif_thumbnail(path: &str) -> Option<image::Handle> {
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
