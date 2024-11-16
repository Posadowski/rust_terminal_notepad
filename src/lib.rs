use std::fs::File;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::io::Read;

pub fn initialize_text_buffer(file_name: &str) -> std::io::Result<(Arc<Mutex<String>>, (u16, u16))> {
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