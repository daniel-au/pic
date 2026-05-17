use std::io::{self, BufRead};

pub fn _read_input() -> Result<String, io::Error> {
    let mut buffer = String::new();
    io::stdin().lock().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}
