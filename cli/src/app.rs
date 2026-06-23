use core::structs::{Parameters, Personnage};
use std::fs::File;


// #[derive(Debug)]
pub enum CurrentScreen {
    Main,
    Exit,
    Save,
    Load
}

// #[derive(Debug)]
pub struct App {
    pub personnage: Personnage,
    pub exit: bool,
    pub current_screen: CurrentScreen,
    pub filename: String,
    pub character_index: usize
}

impl App {
    pub fn new(params: Parameters) -> App {
        let personnage = core::generate_personnage(params);

        App {
            personnage,
            exit: false,
            current_screen: CurrentScreen::Main,
            filename: String::new(),
            character_index: 0
        }
    }
    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn load(&mut self) -> Result<(), std::io::Error> {
        let filename = format!("{}.json", self.filename);
        let file = File::open(filename)?;
        self.personnage = serde_json::from_reader(file)?;
        self.current_screen = CurrentScreen::Main;
        self.reset_cursor();
        self.filename = String::new();
        Ok(())
    }

    pub fn save(&mut self) -> Result<(), std::io::Error> {
        let filename = format!("{}.json", self.filename);
        let file = File::create(filename)?;
        serde_json::to_writer_pretty(file, &self.personnage)?;

        self.current_screen = CurrentScreen::Main;
        self.reset_cursor();
        self.filename = String::new();
        Ok(())
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.filename.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.filename
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.filename.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.filename.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.filename.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.filename = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.filename.chars().count())
    }

    pub const fn reset_cursor(&mut self) {
        self.character_index = 0;
    }
}