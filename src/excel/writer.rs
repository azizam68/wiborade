use umya_spreadsheet::*;

/// Ecrase les valeurs d'une ligne existante.
pub fn update_row(
    file_path: &str,
    row_index: usize,
    data: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut book = reader::xlsx::read(file_path)?;

    let sheet = book
        .sheet_mut(0)
        .map_err(|_| "Aucune feuille dans le classeur")?;

    let excel_row = row_index as u32;

    for (i, value) in data.iter().enumerate() {
        let column = (i + 1) as u32;
        sheet.cell_mut((column, excel_row)).set_value(value);
    }

    writer::xlsx::write(&book, file_path)?;

    Ok(())
}

/// Insère une nouvelle ligne à l'index donné.
pub fn insert_row(
    file_path: &str,
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
