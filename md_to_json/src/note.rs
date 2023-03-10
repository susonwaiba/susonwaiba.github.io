use crate::file;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use titlecase::titlecase;

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub date: String,
    pub content: String,
    pub path: String,
    pub sort_order: i32,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct NoteItem {
    pub id: String,
    pub title: String,
    pub date: String,
    pub path: String,
    pub sort_order: i32,
}

pub fn read_note_md_single_file(path: String) -> Option<Note> {
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
        let mut date: String = "".to_string();

        for line in header_content.split("\n") {
            if let Some((key, val)) = line.clone().split_once(": ") {
                if key == "title" {
                    title = val.clone().to_string();
                }
                if key == "date" {
                    date = val.clone().to_string();
                }
            }
        }

        // If date is not present in the header, then use the current date.
        if date == "" {
            let utc_date = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
            date =
                DateTime::parse_from_str(&format!("{} +05:45", &utc_date), "%Y-%m-%d %H:%M:%S %z")
                    .unwrap_or_default()
                    .format("%Y-%m-%d")
                    .to_string();
        }

        let sort_order: i32 =
            DateTime::parse_from_str(&format!("{} 09:00:00 +05:45", date), "%Y-%m-%d %H:%M:%S %z")
                .unwrap_or_default()
                .format("%s")
                .to_string()
                .parse::<i32>()
                .unwrap_or(0);

        let content = file::generate_html_from_md(body_content.to_string());

        let current_note_path = file_name.clone().replace(".md", "").to_string().clone();

        let note = Note {
            id: file_name.to_string().clone(),
            title: title.clone(),
            date: date.to_string().clone(),
            path: current_note_path.clone(),
            content,
            sort_order,
        };

        println!("Note: {:?}", note);

        return Some(note);
    }

    None
}

pub fn read_note_md_files() -> Vec<Note> {
    let mut result: Vec<Note> = vec![];

    let path_read_result = fs::read_dir("./notes");
    if let Ok(path_list) = path_read_result {
        for path in path_list {
            let path_name = path.unwrap().path().display().to_string();
            let note = read_note_md_single_file(path_name);
            if let Some(note) = note {
                result.push(note);
            }
        }
    }

    result
}

pub fn write_note_json_files(result: Vec<NoteItem>, file_name: String) {
    let json = serde_json::to_string(&result);
    if let Ok(content) = json {
        file::write_json_export_file(file_name, content);
    }
}

pub fn write_note_json_file(note: Note, file_name: String) {
    let json = serde_json::to_string(&note);
    if let Ok(content) = json {
        file::write_json_export_file(file_name, content);
    }
}

pub fn build_note_json() {
    let mut result = read_note_md_files();
    result.sort_by(|a, b| b.sort_order.partial_cmp(&a.sort_order).unwrap());

    let mut note_items: Vec<NoteItem> = vec![];
    for note in result.clone() {
        let note_item = NoteItem {
            id: note.id.clone(),
            title: note.title.clone(),
            date: note.date.clone(),
            path: note.path.clone(),
            sort_order: note.sort_order.clone(),
        };
        note_items.push(note_item);
    }
    println!("\n");
    write_note_json_files(note_items.clone(), "notes.json".to_string());

    let first_recent_notes = note_items
        .clone()
        .into_iter()
        .take(3)
        .collect::<Vec<NoteItem>>();
    write_note_json_files(first_recent_notes, "recent_notes.json".to_string());

    result.into_iter().for_each(|note| {
        write_note_json_file(note.clone(), format!("notes/{}.json", note.path));
    });
}
