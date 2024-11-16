use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode,KeyModifiers},
    execute,
    style::{Print},
    terminal::{self},
};
use crossterm::event::KeyEventKind;

fn main() -> std::io::Result<()>{
    let file_name;
    if std::env::args().count() < 2 {
        file_name = "untitled.txt".to_string();
    } else {
        file_name = std::env::args().collect::<Vec<_>>()[1].clone().to_string();
    }

    // terminal init
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    // Variable to store all input from user
    let inserted_text = Arc::new(Mutex::new(String::new())); // user text buffer
    let mut cursor_position = (0,0);

    loop {

        // clear screen and show buffer
        { // put it in a separate scope to free up the mutex on the inserted_text variable
            let text = inserted_text.lock().unwrap();
            execute!(
                stdout,
                terminal::Clear(terminal::ClearType::All),
                cursor::MoveTo(0, 0),
                Print(&*text),
                cursor::MoveTo(cursor_position.0, cursor_position.1),
            )?;
        }
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind != KeyEventKind::Release { // skip releasing the button
                match key_event {
                    event::KeyEvent {
                        code: KeyCode::Char(c),
                        modifiers: KeyModifiers::NONE,
                        ..
                    } => {
                        println!("Get char");
                        let mut text = inserted_text.lock().unwrap();
                        text.push(c);
                        cursor_position.0 += 1;
                    }

                    event::KeyEvent {
                        code: KeyCode::Backspace,
                        ..
                    } => {
                        println!("Get backspace");
                        let mut text = inserted_text.lock().unwrap();
                        if !text.is_empty() {
                            text.pop();
                            if cursor_position.0 > 0 {
                                cursor_position.0 -= 1;
                            }
                        }
                    }

                    event::KeyEvent {
                        code: KeyCode::Enter,
                        ..
                    } => {
                        println!("Get eneter");
                        let mut text = inserted_text.lock().unwrap();
                        text.push('\n');
                        cursor_position.0 = 0;
                        cursor_position.1 += 1;
                    }

                    event::KeyEvent {
                        code: KeyCode::Esc,
                        ..
                    } => {
                        println!("Get esc");
                        break;
                    }
                    event::KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => {
                        println!("Get ctrl+c");
                        let text = inserted_text.lock().unwrap();
                        let mut filr = File::create(&file_name)?;
                        filr.write_all(text.as_bytes())?;
                        break;
                    }

                    _ => {}
                }
            }
        }
    }
    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
