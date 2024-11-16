use rust_terminal_notepad::initialize_text_buffer;
use std::{
    fs::File,
    io::Write,
    sync::mpsc,
    thread,
    time::Duration,
};

use crossterm::event::KeyEventKind;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::Print,
    terminal::{self},
};

fn main() -> std::io::Result<()> {
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

    // Initialize text buffer and cursor position
    let (inserted_text, mut cursor_position) = initialize_text_buffer(&file_name)?;

    let mut show_cursor = true;

    // Setup communication channel for blinking cursor
    let (tx, rx) = mpsc::channel();

    // Start a thread for blinking the cursor
    let _blink_thread = thread::spawn(move || loop {
        tx.send(()).unwrap();
        thread::sleep(Duration::from_millis(500));
    });

    loop {
        // clear screen and show buffer
        {
            // put it in a separate scope to free up the mutex on the inserted_text variable
            let text = inserted_text.lock().unwrap();
            execute!(
                stdout,
                terminal::Clear(terminal::ClearType::All),
                cursor::MoveTo(0, 0),
                Print(&*text),
                cursor::MoveTo(cursor_position.0, cursor_position.1),
            )?;
            if show_cursor {
                execute!(stdout, cursor::Show)?;
            } else {
                execute!(stdout, cursor::Hide)?;
            }
        }

        if let Ok(_) = rx.try_recv() {
            show_cursor = !show_cursor;
        }

        if let Event::Key(key_event) = event::read()? {
            if key_event.kind != KeyEventKind::Release {
                // skip releasing the button
                match key_event {
                    event::KeyEvent {
                        code: KeyCode::Char(c),
                        modifiers: KeyModifiers::NONE,
                        ..
                    } => {
                        let mut text = inserted_text.lock().unwrap();
                        text.push(c);
                        cursor_position.0 += 1;
                    }

                    event::KeyEvent {
                        code: KeyCode::Char(c),
                        modifiers: KeyModifiers::SHIFT,
                        ..
                    } => {
                        let mut text = inserted_text.lock().unwrap();
                        text.push(c);
                        cursor_position.0 += 1;
                    }

                    event::KeyEvent {
                        code: KeyCode::Backspace,
                        ..
                    } => {
                        let mut text = inserted_text.lock().unwrap();
                        if !text.is_empty() {
                            text.pop();
                            if cursor_position.0 > 0 {
                                cursor_position.0 -= 1;
                            } else if  cursor_position.1 > 0 {
                                let lines: Vec<&str> = text.split('\n').collect();
                                if let Some(prev_line) = lines.get(cursor_position.1 as usize - 1) {
                                    cursor_position.1 -= 1;
                                    cursor_position.0 = prev_line.len() as u16; // Set cursor to the end of the previous line
                                }
                            }
                        }
                    }

                    event::KeyEvent {
                        code: KeyCode::Enter,
                        ..
                    } => {
                        let mut text = inserted_text.lock().unwrap();
                        text.push('\n');
                        cursor_position.0 = 0;
                        cursor_position.1 += 1;
                    }

                    event::KeyEvent {
                        code: KeyCode::Esc, ..
                    } => {
                        break;
                    }
                    event::KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => {
                        let text = inserted_text.lock().unwrap();
                        let mut file = File::create(&file_name)?;
                        file.write_all(text.as_bytes())?;
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
