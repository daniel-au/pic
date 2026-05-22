use regex::Regex;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::LazyLock;

use crate::images::{_get_image_extensions, _get_images};
use crate::input::_read_input;

static FILE_NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?<prefix>.+)_(?<index>\d+)\.(?<extension>\w+)$").unwrap()
});

fn _parse_filename(filename: &str) -> (String, u32, String) {
    let caps = FILE_NAME_REGEX.captures(filename).unwrap();
    let prefix = caps["prefix"].to_string();
    let file_number = caps["index"].parse().unwrap();
    let extension = caps["extension"].to_string();
    (prefix, file_number, extension)
}

pub fn copy() {
    println!("Which file contains the list of numbers to copy? Type `.` for the default file.");
    let file_name = match _read_input() {
        Ok(input) => {
            if input == "." {
                String::from("Good Ones.txt")
            } else {
                input
            }
        }
        Err(e) => panic!("Error: failed to read file name: {}", e),
    };

    let dest_dir = "Good Ones";
    if !Path::new(dest_dir).exists() {
        if let Err(e) = fs::create_dir_all(dest_dir) {
            panic!("Error: failed to create directory '{}': {}", dest_dir, e);
        }
    }

    let file = match File::open(&file_name) {
        Ok(file) => file,
        Err(e) => panic!("Error: failed to open file '{}': {}", file_name, e),
    };

    let reader = BufReader::new(file);
    let image_extensions = _get_image_extensions();
    let available_images = _get_images(&image_extensions);

    let mut file_numbers: HashSet<u32> = HashSet::new();
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line.trim().to_string(),
            Err(e) => panic!("Error: failed to read line from file: {}", e),
        };

        if line.is_empty() {
            continue;
        }

        let file_number: u32 = match line.parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Warning: skipping invalid line '{}'", line);
                continue;
            }
        };
        file_numbers.insert(file_number);
    }

    for image in available_images {
        let (_, index, _) = _parse_filename(&image);
        if !file_numbers.contains(&index) {
            continue;
        }
        let dest_path = format!("{}/{}", dest_dir, &image);
        println!("Copying {} to {}", &image, &dest_path);
        if let Err(e) = fs::copy(&image, &dest_path) {
            println!("Error: failed to copy '{}': {}", image, e);
        }
    }

    println!("Copy operation completed!");
}
