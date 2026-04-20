pub const TASK_PATH: &str = "tasks.json";

use std::{fs, path::Path};

use crate::category::Category;

pub fn save(path: &str, categories: &Vec<Category>) {
    let json = serde_json::to_string_pretty(categories)
        .expect("Could not serialize categories.");

    fs::write(path, json)
        .expect("Could not write to file.");
}

pub fn load(path: &str) -> Vec<Category> {
        if !Path::new(path).exists() {
            let categories: Vec<Category> = vec![];

            return categories;
        } 
        let data = fs::read_to_string(path).unwrap_or_else(|_| {
            println!("Failed to create file, creating new one.");
            return String::new();
        });

        if data.trim().is_empty() {
            return vec![];
        }

        serde_json::from_str(&data).unwrap_or_else(|_| {
            println!("Corrupted file, couldn't read.");
            vec![]
        })
    }