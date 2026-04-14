use core::structs::{Parameters, Personnage};


// #[derive(Debug)]
pub enum CurrentScreen {
    Main,
    Exit,
    Save
}

// #[derive(Debug)]
pub struct App {
    pub personnage: Personnage,
    pub exit: bool,
    pub current_screen: CurrentScreen
}

impl App {
    pub fn new(params: Parameters) -> App {
        let personnage = core::generate_personnage(params);

        App {
            personnage,
            exit: false,
            current_screen: CurrentScreen::Main

        }

    }
    pub fn exit(&mut self) {
        self.exit = true;
    }
    pub fn save(&mut self) -> Result<(), std::io::Error> {
        todo!("à implémenter plus tard");
        // use std::io::ErrorKind;
        // Err(std::io::Error::new(ErrorKind::Other, "oh no!"))
    }
    // fn display_personnage(self) {
    //     println!(" *** age ***");
    //     println!("age : {}", self.personnage.age);

    //     println!(" *** education ***");
    //     println!("education : {}", self.personnage.education.name);
    //     println!("level : {}", self.personnage.education.level);

    //     println!(" *** personnality ***");
    //     for personalit in self.personnage.personnality {
    //         println!("{}", personalit.name);
    //     }

    //     println!(" *** statistiques ***");
    //     for (key, val) in self.personnage.statistiques.iter() {
    //         println!("{key} : {}", val.base + val.bonus);
    //     }

    //     println!("points_totaux : {}", self.personnage.points_totaux);
    // }
}