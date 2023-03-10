use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn read_from_file(file_path: &std::path::Path) -> Result<String, Box<dyn std::error::Error>> {
    let md = fs::read_to_string(file_path.to_str().unwrap())?;
    Ok(md)
}

pub fn write_data_to_file(
    file_path: &std::path::Path,
    content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(file_path)?;
    write!(file, "{}", content)?;
    Ok(())
}

pub fn write_json_export_file(file_name: String, content: String) {
    let file_path = format!("./../assets/exports/{}", file_name);
    let file_path_obj = Path::new(&file_path);
    let result = write_data_to_file(file_path_obj, &content);
    if let Ok(_) = result {
        println!("File {} written successfully", file_name);
    }
}

pub fn generate_html_from_md(markdown: String) -> String {
    let mut options = Options::empty();
    // Strikeouts are not part of the CommonMark standard and must be explicitly enabled.
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&markdown, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
