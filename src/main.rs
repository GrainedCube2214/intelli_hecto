use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // 1. Enter "Raw Mode" 
    // This tells the terminal: "Don't process keys, just send them to me raw."
    enable_raw_mode()?;

    println!("IntelliHecto (Press 'Ctrl+c' to quit)\r");

    loop {
        // 2. Read the next event from the keyboard
        // This blocks until a key is pressed.
        if let Event::Key(event) = read()? {
            
            // 3. Print what key was pressed
            // We use \r\n because in Raw Mode, \n only moves down, not back to start of line.
            print!("Key: {:?}\r\n", event);
            
            // Flush the buffer so it prints immediately
            io::stdout().flush().unwrap();

            // 4. Exit condition
            if event.modifiers.contains(KeyModifiers::CONTROL) && event.code == KeyCode::Char('c') {
                break;
            }
        }
    }

    // 5. Clean up! 
    // Essential: If you crash before this, your terminal will stay broken.
    disable_raw_mode()?;
    Ok(())
}