use std::path::Path;

pub struct PathStringArgs {
    pub template_file: String,
    pub json_file: String,
    pub output_dir: String,
}

pub struct PathArgs<'a> {
    pub template_file: &'a Path,
    pub json_file: &'a Path,
    pub output_dir: &'a Path,
}

impl PathArgs<'_> {
    pub fn from(path_string_args: &PathStringArgs) -> PathArgs {
        return PathArgs {
            template_file: Path::new(&path_string_args.template_file),
            json_file: Path::new(&path_string_args.json_file),
            output_dir: Path::new(&path_string_args.output_dir),
        };
    }
}
