use std::fs;
use std::path::PathBuf;

/// Liste les fichiers xlsx et jpg d'un dossier.
/// Les xlsx sont placés en tête de liste, suivis des images triées par nom.
pub fn list(path: &PathBuf) -> Vec<(String, bool)> {
    let mut pics = Vec::default();
    let mut xlsxes = Vec::default();

    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with("xlsx") {
                    xlsxes.push((name.to_string(), true));
                } else if name.ends_with("jpg") {
                    pics.push((name.to_string(), false));
                }
            }
        }
    }

//pics.sort_by(|a, b| a.0.cmp(&b.0));

pics.sort_by(|a, b| a.0.cmp(&b.0));

if xlsxes.is_empty() {
    let dir_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "sans_nom".to_string());

    let filename = format!("list_{}.xlsx", dir_name);
    let new_xlsx_path = path.join(&filename);

    let book = umya_spreadsheet::new_file();
    match umya_spreadsheet::writer::xlsx::write(&book, &new_xlsx_path) {
        Ok(()) => {
            xlsxes.push((filename, true));
        }
        Err(e) => {
            eprintln!("Erreur lors de la création du xlsx : {:?}", e);
            // on n'ajoute rien à xlsxes dans ce cas
        }
    }
}


xlsxes.append(&mut pics);
    xlsxes
}
