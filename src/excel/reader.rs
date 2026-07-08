use calamine::{open_workbook_auto, Data, DataType, Reader};
use std::fs;
use std::time::SystemTime;

#[derive(Debug)]
pub enum LoadResult {
    Unchanged,
    Loaded {
        rows: Vec<Vec<String>>,
        modified: SystemTime,
    },
}

/// Recharge le fichier xlsx uniquement si sa date de modification a changé
/// depuis le dernier chargement.
pub fn load_rows(path: &str, last_modified: Option<SystemTime>) -> Result<LoadResult, String> {
    let metadata = fs::metadata(path).map_err(|e| e.to_string())?;
    let modified = metadata.modified().map_err(|e| e.to_string())?;

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
    if cell.is_datetime() {
        if let Some(dt) = cell.as_datetime() {
            return dt.format("%d-%m-%Y").to_string();
        }
        if let Some(d) = cell.as_date() {
            return d.format("%d-%m-%Y").to_string();
        }
        if let Some(t) = cell.as_time() {
            return t.format("%H:%M:%S").to_string();
        }
    }

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
