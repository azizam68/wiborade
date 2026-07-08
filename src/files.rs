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

    pics.sort_by(|a, b| a.0.cmp(&b.0));
    xlsxes.append(&mut pics);
    xlsxes
}
