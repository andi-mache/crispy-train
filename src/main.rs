// import th needed libraries
use std::{stdout, result, error::error, io};

use ratatui::{
    backend::crosstermbackend, crossterm::{
        cursor, event::{self, disablemousecapture, enablemousecapture, keycode}, terminal::{disable_raw_mode, enable_raw_mode, enteralternatescreen, leavealternatescreen}, executablecommand
    }, prelude::*, style::stylize, widgets::{block, list, listitem, paragraph}, terminal
};


enum inputmode {
    normal,
    editing,
}

// the app holds the state of the application 
struct app {
    // the current value of the input box
    input: string,
    // position of the cursor in editor area 
    character_index: usize,
    // curent input mode 
    input_mode: inputmode,
    //history of recorder messagrs will be stored in vector screens 
    messages: vec<string>,

}

impl app {
    const fn new() -> self {
        self {
            input: string::new(),
            input_mode: inputmode::normal,
            messages: vec::new(),
            character_index: 0,
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);

    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_sub(1);
        self.character_index =self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    // it returns the byte index based on the caracter position.
    //
    // since each character in a string can contain multiple bytes,
    // it's necessary to calculate the byte index oh the character,
    fn byte_index(&mut self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

     fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn submit_message(&mut self) {
        self.messages.push(self.input.clone());
        self.input.clear();
        self.reset_cursor();
    }

    
}


fn main() -> result<()> {
    stdout().execute(enteralternatescreen)?;
    enable_raw_mode()?;
    let mut terminal = terminal::new(crosstermbackend::new(stdout()))?;
    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                paragraph::new("hello ratatui! (press 'q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        })?;

        if event::poll(std::time::duration::from_millis(16))? {
            if let event::event::key(key) = event::read()? {
                if key.kind == keyeventkind::press && key.code == keycode::char('q') {
                    break;
                }
            }
        }
    }

    stdout().execute(leavealternatescreen)?;
    disable_raw_mode()?;
    ok(())
}
