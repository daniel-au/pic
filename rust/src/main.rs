use rand::distr::{Alphanumeric, SampleString};
use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::sync::LazyLock;

const PREFIX_LENGTH: u32 = 6;

static FILE_NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?<prefix>.+)_(?<index>\d+)\.(?<extension>\w+)$").unwrap()
});

/// Create the HashSet of image extensions
fn _get_image_extensions() -> HashSet<String> {
    HashSet::from(["cr2", "gif", "jpg", "jpeg", "mov", "mp4", "nef", "png"].map(String::from))
}

/// read a line from stdin, strips the leading and trailing whitespace
fn _read_input() -> Result<String, io::Error> {
    let mut buffer = String::new();
    io::stdin().lock().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

/// prompts the user for a new prefix, and returns it
///
/// if the user enters in `.`, read the directory name as the new prefix
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

/// Returns the file extension
fn _get_file_extension(filename: &str) -> String {
    match Path::new(filename).extension() {
        Some(ext) => ext.to_str().unwrap_or("").to_string(),
        None => String::new(),
    }
}

/// Grabs all the images in the current directory, filters out the ones that aren't images
///
/// Sorts the files alphabetically
fn _get_images(img_extensions: &HashSet<String>) -> Vec<String> {
    let mut image_files: Vec<String> = Vec::new();
    let entries: fs::ReadDir = match fs::read_dir(".") {
        Ok(entries) => entries,
        Err(e) => panic!("Error: failed to read directory: {}", e),
    };
    for entry_result in entries {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(e) => panic!("Error: failed to read directory entry: {}", e),
        };
        let entry_md: fs::Metadata = match entry.metadata() {
            Ok(md) => md,
            Err(e) => panic!("Error: failed to read directory entry metadata: {}", e),
        };
        if entry_md.is_dir() {
            continue;
        }
        let filename: String = entry.file_name().into_string().unwrap();
        let extension: String = _get_file_extension(&filename);
        if img_extensions.contains(&extension.to_lowercase()) {
            image_files.push(filename);
        }
    }
    image_files.sort();
    image_files
}

/// Renames a single image to a new prefix and index
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

/// Renames all given photos to a new prefix and index
fn _rename_all_photos(image_files: Vec<String>, new_prefix: &str, starting_index: &str) {
    let mut index: u32 = starting_index.parse().unwrap();
    for image in image_files {
        _rename_image(&image, new_prefix, index);
        index += 1;
    }
}

/// Generates a random prefix of a given length
fn _generate_random_prefix(length: u32) -> String {
    Alphanumeric.sample_string(&mut rand::rng(), length as usize)
}

/// Renames all photo and video files in the current directory.
///
/// Prompts the user for either the new photo title or a `.` to represent the current direction,
/// and the starting index. Uses that as a prefix and incrementally renames each photo and video to
/// a random prefix, and then renames it all to have the new prefix, an underscore, the index, and
/// original extension.
fn rename() {
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

/// given a filename, extract out the prefix, file number, and extension
fn _parse_filename(filename: &str) -> (String, u32, String) {
    let caps = FILE_NAME_REGEX.captures(filename).unwrap();
    let prefix = caps["prefix"].to_string();
    let file_number = caps["index"].parse().unwrap();
    let extension = caps["extension"].to_string();
    (prefix, file_number, extension)
}

/// Copies the images in the current directory.
fn copy() {
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

    // Create the "Good Ones" directory if it doesn't exist
    let dest_dir = "Good Ones";
    if !Path::new(dest_dir).exists() {
        if let Err(e) = fs::create_dir_all(dest_dir) {
            panic!("Error: failed to create directory '{}': {}", dest_dir, e);
        }
    }

    // Read the file and process each line
    let file = match File::open(&file_name) {
        Ok(file) => file,
        Err(e) => panic!("Error: failed to open file '{}': {}", file_name, e),
    };

    let reader = BufReader::new(file);
    let image_extensions = _get_image_extensions();
    let available_images = _get_images(&image_extensions);

    // Create a HashSet of the numbers in the file
    let mut file_numbers: HashSet<u32> = HashSet::new();
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line.trim().to_string(),
            Err(e) => panic!("Error: failed to read line from file: {}", e),
        };

        if line.is_empty() {
            continue;
        }

        // Parse the line as a number
        let file_number: u32 = match line.parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Warning: skipping invalid line '{}'", line);
                continue;
            }
        };
        file_numbers.insert(file_number);
    }

    // Iterate through the available images and copy the ones that should be copied
    // Easier to do this way rather than constructing the full image name from the index
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

// Prints a usage message
fn _usage() {
    println!(
        "This program is used for your batch of photos. It takes in one command line argument - \
        either `rename` or `copy` to rename a batch of photos or copy the ones enumerated in a \
        text file. Example: `$ ./pic copy`"
    );
}

/// read command line args and execute the correct function
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Too few command line arguments.");
        _usage();
    } else if args.len() == 2 {
        if args[1] == "rename" {
            rename()
        } else if args[1] == "copy" {
            copy()
        } else {
            println!("Unknown command line argument.");
            _usage();
        }
    } else {
        println!("Too many command line arguments.");
        _usage();
    }
}
