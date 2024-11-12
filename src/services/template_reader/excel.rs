use super::parser::ParsedPlaceholder;
use crate::services::{args::PathArgs, config::Config, output_generator::excel::RowDataFormat};
use calamine::{open_workbook, Error, Reader, Xlsx};

pub fn process(
    path_args: &PathArgs,
    placeholder_data: &Vec<ParsedPlaceholder>,
) -> Result<Vec<Vec<String>>, Error> {
    let config: Config = Config::get().unwrap();
    let placeholder_config = &config.excel.placeholder;

    let mut workbook: Xlsx<_> = open_workbook(path_args.template_file)?;
    let range = workbook.worksheet_range(&config.excel.sheet_name)?;

    let mut row_data: RowDataFormat = vec![];

    for cols in range.rows() {
        let mut col_data: Vec<String> = vec![];

        'col_for: for col in cols {
            for placeholder in placeholder_data {
                let placeholder_prefix_suffix = format!(
                    "{}{}{}",
                    placeholder_config.prefix, placeholder.name, placeholder_config.suffix
                );

                if placeholder_prefix_suffix.to_string() == col.to_string() {
                    col_data.push(placeholder.value.to_string());
                    continue 'col_for;
                }
            }

            col_data.push(col.to_string());
        }

        row_data.push(col_data);
    }

    Ok(row_data)
}
