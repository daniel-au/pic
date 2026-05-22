mod copy;
mod images;
mod input;
mod rename;

use std::env;

fn usage() {
    println!(
        "This program is used for your batch of photos. It takes in one command line argument - \
        either `rename` or `copy` to rename a batch of photos or copy the ones enumerated in a \
        text file. Example: `$ ./pic copy`"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Too few command line arguments.");
        usage();
    } else if args.len() == 2 {
        if args[1] == "rename" {
            rename::rename()
        } else if args[1] == "copy" {
            copy::copy()
        } else {
            println!("Unknown command line argument.");
            usage();
        }
    } else {
        println!("Too many command line arguments.");
        usage();
    }
}
