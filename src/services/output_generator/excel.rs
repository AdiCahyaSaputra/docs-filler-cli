use rust_xlsxwriter::*;

use crate::services::config::Config;

pub type RowDataFormat = Vec<Vec<String>>;

pub fn save_into_file(row_data: RowDataFormat, output_file_path: String) -> Result<(), XlsxError> {
    let config: Config = Config::get().unwrap();

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    for (row, cols) in row_data.iter().enumerate() {
        for (col_idx, col) in cols.iter().enumerate() {
            worksheet.write(row as u32, col_idx as u16, col)?;
        }
    }

    worksheet.set_name(config.excel.sheet_name)?;
    workbook.save(output_file_path)?;

    Ok(())
}
