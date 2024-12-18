use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};

use rust_terminal_notepad::initialize_text_buffer;

// event::read mock structure
struct MockEventReader {
    events: Vec<Event>,
    current: usize,
}

impl MockEventReader {
    fn new(events: Vec<Event>) -> Self {
        MockEventReader { events, current: 0 }
    }

    fn read(&mut self) -> std::io::Result<Event> {
        if self.current < self.events.len() {
            let event = self.events[self.current].clone();
            self.current += 1;
            Ok(event)
        } else {
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: crossterm::event::KeyEventState::NONE,
            }))
        }
    }
}

#[test]
fn test_basic_text_input() -> std::io::Result<()> {
    let test_file = "test_output.txt";
    let test_content = "Hello, World!";

    // Preparing a sequence of events
    let events = vec![
        // Simulation of typing "Hello, World!"
        Event::Key(KeyEvent {
            code: KeyCode::Char('H'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('e'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('l'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('l'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('o'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char(','),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char(' '),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('W'),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('o'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('r'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('l'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('d'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('!'),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        // Save and exit
        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
    ];

    let mut mock_reader = MockEventReader::new(events);

    // Editor operation simulation
    let inserted_text = Arc::new(Mutex::new(String::new()));

    loop {
        match mock_reader.read()? {
            Event::Key(key_event) => {
                if key_event.kind != KeyEventKind::Release {
                    match key_event {
                        event
                            if event.code == KeyCode::Char('c')
                                && event.modifiers == KeyModifiers::CONTROL =>
                        {
                            let text = inserted_text.lock().unwrap();
                            fs::write(test_file, &*text)?;
                            break;
                        }
                        event if event.code == KeyCode::Esc => {
                            break;
                        }
                        event => {
                            if let KeyCode::Char(c) = event.code {
                                let mut text = inserted_text.lock().unwrap();
                                text.push(c);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Checking the results
    assert!(
        Path::new(test_file).exists(),
        "The file has not been created"
    );
    let saved_content = fs::read_to_string(test_file)?;
    assert_eq!(
        saved_content, test_content,
        "The file content does not match the expected content"
    );

    // Cleaning
    fs::remove_file(test_file)?;

    Ok(())
}

#[test]
fn test_backspace() -> std::io::Result<()> {
    let test_file = "test_backspace.txt";
    let expected_content = "Hello";

    let events = vec![
        Event::Key(KeyEvent {
            code: KeyCode::Char('H'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('e'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('l'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('l'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('o'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('!'),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Backspace,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
    ];

    let mut mock_reader = MockEventReader::new(events);
    let inserted_text = Arc::new(Mutex::new(String::new()));

    loop {
        match mock_reader.read()? {
            Event::Key(key_event) => {
                if key_event.kind != KeyEventKind::Release {
                    match key_event {
                        event
                            if event.code == KeyCode::Char('c')
                                && event.modifiers == KeyModifiers::CONTROL =>
                        {
                            let text = inserted_text.lock().unwrap();
                            fs::write(test_file, &*text)?;
                            break;
                        }
                        event if event.code == KeyCode::Backspace => {
                            let mut text = inserted_text.lock().unwrap();
                            text.pop();
                        }
                        event => {
                            if let KeyCode::Char(c) = event.code {
                                let mut text = inserted_text.lock().unwrap();
                                text.push(c);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Checking the results
    assert!(
        Path::new(test_file).exists(),
        "The file has not been created"
    );
    let saved_content = fs::read_to_string(test_file)?;
    assert_eq!(
        saved_content, expected_content,
        "The file content does not match the expected content"
    );

    // Cleaning
    fs::remove_file(test_file)?;

    Ok(())
}

#[test]
fn test_multiline_input() -> std::io::Result<()> {
    let test_file = "test_multiline.txt";
    let expected_content = "Line 1\nLine 2";

    let events = vec![
        Event::Key(KeyEvent {
            code: KeyCode::Char('L'),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('i'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('n'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('e'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char(' '),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('1'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('L'),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('i'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('n'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('e'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char(' '),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('2'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
    ];

    let mut mock_reader = MockEventReader::new(events);
    let inserted_text = Arc::new(Mutex::new(String::new()));

    loop {
        match mock_reader.read()? {
            Event::Key(key_event) => {
                if key_event.kind != KeyEventKind::Release {
                    match key_event {
                        event
                            if event.code == KeyCode::Char('c')
                                && event.modifiers == KeyModifiers::CONTROL =>
                        {
                            let text = inserted_text.lock().unwrap();
                            fs::write(test_file, &*text)?;
                            break;
                        }
                        event if event.code == KeyCode::Enter => {
                            let mut text = inserted_text.lock().unwrap();
                            text.push('\n');
                        }
                        event => {
                            if let KeyCode::Char(c) = event.code {
                                let mut text = inserted_text.lock().unwrap();
                                text.push(c);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Checking the results
    assert!(
        Path::new(test_file).exists(),
        "The file has not been created"
    );
    let saved_content = fs::read_to_string(test_file)?;
    assert_eq!(
        saved_content, expected_content,
        "The file content does not match the expected content"
    );

    // Cleaning
    fs::remove_file(test_file)?;

    Ok(())
}

#[test]
fn test_read_file() -> std::io::Result<()> {
    let test_file = "test_read_file.txt";
    let mut file = File::create(test_file)?;
    let expected_content = "Here is some text!\nNew Line 1";
    let some_input_text = b"Here is some text!";
    file.write_all(some_input_text)?;

    let (inserted_text, cursor_position) = initialize_text_buffer(&test_file)?;

    assert_eq!(
        cursor_position,
        (some_input_text.len() as u16, 0),
        "Cursor position calculated incorrectly"
    );

    let events = vec![
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('N'),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('e'),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('w'),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char(' '),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('L'),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('i'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('n'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('e'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char(' '),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('1'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }),
    ];

    let mut mock_reader = MockEventReader::new(events);

    loop {
        match mock_reader.read()? {
            Event::Key(key_event) => {
                if key_event.kind != KeyEventKind::Release {
                    match key_event {
                        event
                            if event.code == KeyCode::Char('c')
                                && event.modifiers == KeyModifiers::CONTROL =>
                        {
                            let text = inserted_text.lock().unwrap();
                            fs::write(test_file, &*text)?;
                            break;
                        }
                        event if event.code == KeyCode::Enter => {
                            let mut text = inserted_text.lock().unwrap();
                            text.push('\n');
                        }
                        event => {
                            if let KeyCode::Char(c) = event.code {
                                let mut text = inserted_text.lock().unwrap();
                                text.push(c);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Checking the results
    assert!(
        Path::new(test_file).exists(),
        "The file has not been created"
    );
    let saved_content = fs::read_to_string(test_file)?;
    assert_eq!(
        saved_content, expected_content,
        "The file content does not match the expected content"
    );

    // Cleaning
    fs::remove_file(test_file)?;

    Ok(())
}

#[test]
fn test_position_to_index() {
    use rust_terminal_notepad::position_to_index;
    let text = "some string\nsome line";

    // First Line Position Test
    assert_eq!(position_to_index(text, (0, 0)), 0);  // Beginning of the text
    assert_eq!(position_to_index(text, (5, 0)), 5);  // Position ' ' in "some "
    assert_eq!(position_to_index(text, (11, 0)), 11); // Position '\n'

    // Second Line Position Test
    assert_eq!(position_to_index(text, (0, 1)), 12);  // Beginning "some line"
    assert_eq!(position_to_index(text, (4, 1)), 16);  // Position 'l' w "line"
    assert_eq!(position_to_index(text, (9, 1)), 21);  // At the end of the second line

    // Test out of range (should return end of text)
    assert_eq!(position_to_index(text, (15, 1)), 21); // Beyond the length of the second line
    assert_eq!(position_to_index(text, (0, 2)), 21);  // Beyond the number of lines
}

#[test]
fn test_line_length() {
    use rust_terminal_notepad::line_length;
    let text = "some string\nsome line";

    // line lenght test
    assert_eq!(line_length(text, 0), 11);  //  "some string"
    assert_eq!(line_length(text, 1), 9);   //  "some line"

    // empty line test
    let text_with_empty_line = "some string\nsome line\n";
    assert_eq!(line_length(text_with_empty_line, 2), 0); // empty line

    // Test out of range (should return 0)
    assert_eq!(line_length(text, 2), 0);  // No line 2
}

#[test]
fn test_total_lines() {
    use rust_terminal_notepad::total_lines;
    let text = "some string\nsome line";

    // Two lines test
    assert_eq!(total_lines(text), 2);

    // three line test
    let text_with_empty_line = "some string\nsome line\n";
    assert_eq!(total_lines(text_with_empty_line), 3);

    // one line test
    let single_line_text = "just one line";
    assert_eq!(total_lines(single_line_text), 1);

    // empty text test
    let empty_text = "";
    assert_eq!(total_lines(empty_text), 1); // One blank line
}