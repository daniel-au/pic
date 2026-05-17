use rand::distr::{Alphanumeric, SampleString};
use std::env;
use std::fs;

use crate::images::{_get_file_extension, _get_image_extensions, _get_images};
use crate::input::_read_input;

const PREFIX_LENGTH: u32 = 6;

fn _get_new_prefix() -> String {
    println!("What should the new prefix be? Type `.` for the current directory.");
    let mut new_prefix = match _read_input() {
        Ok(prefix) => prefix,
        Err(e) => panic!("Error: failed to read new prefix: {}", e),
    };
    if new_prefix == "." {
        let cwd = match env::current_dir() {
            Ok(cwd) => cwd,
            Err(e) => panic!("Error: failed to get current directory: {}", e),
        };
        let file_name = match cwd.file_name() {
            Some(file_name) => file_name,
            None => panic!("Error: failed to get the top folder from the current directory path."),
        };
        new_prefix = match file_name.to_str() {
            Some(file_name) => file_name.to_string(),
            None => panic!("Error: failed to convert top folder to a string."),
        };
    }
    new_prefix
}

fn _rename_image(image_name: &str, new_prefix: &str, index: u32) {
    if index > 9999 {
        panic!("Error: index out of range - greater than 9999.");
    }
    let extension = _get_file_extension(image_name);
    let new_image_name: String;
    if new_prefix == "" {
        new_image_name = format!("{:04}.{}", index, extension);
    } else {
        new_image_name = format!("{}_{:04}.{}", new_prefix, index, extension);
    }
    println!("Renaming {} to {}", image_name, new_image_name);
    match fs::rename(image_name, new_image_name) {
        Ok(_) => (),
        Err(e) => panic!("Error: failed to rename image: {}", e),
    };
}

fn _rename_all_photos(image_files: Vec<String>, new_prefix: &str, starting_index: &str) {
    let mut index: u32 = starting_index.parse().unwrap();
    for image in image_files {
        _rename_image(&image, new_prefix, index);
        index += 1;
    }
}

fn _generate_random_prefix(length: u32) -> String {
    Alphanumeric.sample_string(&mut rand::rng(), length as usize)
}

pub fn rename() {
    println!("Let's rename some stuff!");
    let new_prefix: String = _get_new_prefix();
    println!("What should the photo numbers start from?");
    let starting_index: String = match _read_input() {
        Ok(prefix) => prefix,
        Err(e) => panic!("Error, failed to read photo start number: {}", e),
    };
    println!(
        "Photos will be renamed with prefix: `{}` and starting index: `{}`",
        new_prefix, starting_index,
    );
    let image_extensions = _get_image_extensions();
    let mut image_files: Vec<String> = _get_images(&image_extensions);
    // rename each file to a random prefix. This ensures that if we keep the same prefix and just
    // update the numbers, nothing will be overwritten
    let rand_prefix: String = _generate_random_prefix(PREFIX_LENGTH);
    _rename_all_photos(image_files, &rand_prefix, &starting_index);
    image_files = _get_images(&image_extensions);
    _rename_all_photos(image_files, &new_prefix, &starting_index);
}
