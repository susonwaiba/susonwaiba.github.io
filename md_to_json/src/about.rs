use crate::file;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use titlecase::titlecase;

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct About {
    pub id: String,
    pub year: i32,
    pub title: String,
    pub content: String,
}

pub fn read_about_md_single_file(path: String) -> Option<About> {
    let binding_path = path.clone();
    let path_obj = Path::new(&binding_path);
    let file_name = path_obj
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or("NA");

    println!("\n");
    println!("File name: {}", file_name);

    let markdown_content = file::read_from_file(path_obj.clone()).unwrap_or_default();
    if let Some((header_content, body_content)) = markdown_content.split_once("---@---@---\n") {
        let mut title: String =
            titlecase(&file_name.clone().replace(".md", "").replace("-", " ")).to_string();

        for line in header_content.split("\n") {
            if let Some((key, val)) = line.clone().split_once(": ") {
                if key == "title" {
                    title = val.clone().to_string();
                }
            }
        }

        let content = file::generate_html_from_md(body_content.to_string());
        let year = file_name
            .clone()
            .replace(".md", "")
            .to_string()
            .parse::<i32>()
            .unwrap();

        let about = About {
            id: file_name.to_string().clone(),
            year,
            title: title.clone(),
            content,
        };

        println!("About: {:?}", about);

        return Some(about);
    }

    None
}

pub fn read_about_md_files() -> Vec<About> {
    let mut result: Vec<About> = vec![];

    let path_read_result = fs::read_dir("./about");
    if let Ok(path_list) = path_read_result {
        for path in path_list {
            let path_name = path.unwrap().path().display().to_string();
            let note = read_about_md_single_file(path_name);
            if let Some(note) = note {
                result.push(note);
            }
        }
    }

    result
}

pub fn write_about_json_files(result: Vec<About>, file_name: String) {
    let json = serde_json::to_string(&result);
    if let Ok(content) = json {
        file::write_json_export_file(file_name, content);
    }
}

pub fn build_about_json() {
    let mut result = read_about_md_files();
    println!("\n");

    result.sort_by(|a, b| a.year.partial_cmp(&b.year).unwrap());
    write_about_json_files(result.clone(), "about.json".to_string());
}
