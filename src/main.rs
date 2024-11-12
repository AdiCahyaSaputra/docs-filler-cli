mod services {
    pub mod template_reader {
        pub mod excel;
        pub mod parser;
    }

    pub mod output_generator {
        pub mod excel;
    }

    pub mod args;
    pub mod config;
}

use services::{
    args,
    output_generator::excel::save_into_file,
    template_reader::{excel::process, parser::parsing_template},
};
use std::env::args;

fn main() {
    let path_string_args = args::PathStringArgs {
        template_file: args().nth(1).expect("Please specify template files"),
        json_file: args().nth(2).expect("Please specify json files"),
        output_dir: args().nth(3).expect("Please specify output directory"),
    };

    let path_args = args::PathArgs::from(&path_string_args);

    let template_file_type = path_args
        .template_file
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap();

    match template_file_type {
        "xlsx" => {
            let parsed_result = parsing_template(&path_args.json_file);

            for (idx, placeholders) in parsed_result.iter().enumerate() {
                let row_data = process(&path_args, placeholders);

                let output_file_path = format!(
                    "{}/{}.xlsx",
                    &path_args.output_dir.to_string_lossy(),
                    idx.to_string()
                );

                let _result = save_into_file(row_data.unwrap(), output_file_path);
            }
        }
        _ => {
            println!("this template file doesn't match any criteria");
        }
    };
}
