use std::env;
use std::io::{self, BufRead};
use std::path::Path;


/// read a line from stdin
fn _read_input() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

/// prompts the user for a new prefix, and returns it
///
/// if the user enters in `.`, read the directory name as the new prefix
fn _get_new_prefix() -> String {
    println!("What should the new prefix be?");
    let mut new_prefix = _read_input();
    if new_prefix == "." {
        let cwd = env::current_dir().unwrap();
        let full_path = Path::new(&cwd);
        let file_name = full_path.file_name().unwrap();
        new_prefix = file_name.to_str().unwrap().to_string();
    }
    new_prefix
}

/// Renames all photo and video files in the current directory.
///
/// Prompts the user for either the new photo title or a `.` to represent the current direction,
/// and the starting index. Uses that as a prefix and incrementally renames each photo and video to
/// a random prefix, and then renames it all to have the new prefix, an underscore, the index, and
/// original extension.
fn rename() {
    println!("Let's rename some stuff!");
    // prompt user for new prefix
    let new_prefix: String = _get_new_prefix();
    // prompt user for starting index
    println!("What should the photo numbers start from?");
    let starting_index: String = _read_input();
    println!(
        "Photos will be renamed with prefix: `{}` and starting index: `{}`",
        new_prefix,
        starting_index,
    );
    // grab all photo and video files in the current directory
    // rename each file to a random prefix
    // rename each file to its final name
}

/// Copies the images in the current directory.
fn copy() {
    println!("Placeholder - should be copying stuff!");
    // TODO
}

/// read command line args and execute the correct function
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Too few command line arguments.");
    } else if args.len() == 2 {
        if args[1] == "rename" {
            rename()
        } else if args[1] == "copy" {
            copy()
        } else {
            println!("Unknown command line argument.");
        }
    } else {
        println!("Too many command line arguments.");
    }
}
