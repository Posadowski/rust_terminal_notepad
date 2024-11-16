use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};

fn main() -> std::io::Result<()>{
    let file_name;
    if std::env::args().count() < 2 {
        file_name = "untitled.txt".to_string();
    } else {
        file_name = std::env::args().collect::<Vec<_>>()[1].clone().to_string();
    }
    // Variable to store all input from user
    let inserted_text = Arc::new(Mutex::new(String::new()));

    // ctrl+c handler
    {
        let file_name = file_name.clone();
        let inserted_text = Arc::clone(&inserted_text);

        ctrlc::set_handler(move || {
            let text = inserted_text.lock().unwrap();
            let mut file = File::create(&file_name).expect("Cannot create file");
            file.write_all(text.as_bytes()).expect("Failed to write to file");
            std::process::exit(0);
            }).expect("Error setting Ctrl-C handler");
    }
    loop {
        let mut ret = String::new();
        std::io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
        // Adding input to a variable stored in Arc<Mutex<String>>
        {
            let mut text = inserted_text.lock().unwrap();
            text.push_str(&ret);
        }
    }
}
