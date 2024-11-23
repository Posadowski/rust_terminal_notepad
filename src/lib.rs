use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub fn initialize_text_buffer(
    file_name: &str,
) -> std::io::Result<(Arc<Mutex<String>>, (u16, u16))> {
    // Load existing file content if it exists
    let initial_content = if Path::new(file_name).exists() {
        let mut file = File::open(file_name)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        content
    } else {
        String::new()
    };

    let inserted_text = Arc::new(Mutex::new(initial_content));

    // Calculate initial cursor position
    let cursor_position = {
        let text = inserted_text.lock().unwrap();
        let last_line = text.lines().count().max(1) - 1;
        let last_line_length = text.lines().last().unwrap_or("").len();
        (last_line_length as u16, last_line as u16)
    };

    Ok((inserted_text, cursor_position))
}


/// Calculates the index into `Vec<char>` based on the cursor position (x, y).
pub fn  position_to_index(text: &str, cursor: (u16, u16)) -> usize {
    let (x, y) = cursor;
    let mut line = 0;
    let mut index = 0;

    for (_i, ch) in text.chars().enumerate() {
        if line == y {
            return index + x as usize;
        }
        index += ch.len_utf8(); // Jump over the appropriate length of the sign
        if ch == '\n' {
            line += 1;
        }
    }
    index // By default it returns the end of text
}

/// Calculates the length of a line from `y`.
pub fn line_length(text: &str, y: usize) -> u16 {
    let mut line = 0;
    let mut length = 0;

    for ch in text.chars() {
        if line == y {
            if ch == '\n' {
                break;
            }
            length += 1;
        } else if ch == '\n' {
            line += 1;
        }
    }
    length
}

/// Calculates the entire line area of text.
pub fn total_lines(text: &str) -> usize {
    text.chars().filter(|&ch| ch == '\n').count() + 1
}
