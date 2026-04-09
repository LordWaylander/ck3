use core::structs::{Parameters, Personnage};

#[derive(Debug, Default)]
pub struct App {
    pub personnage: Personnage,
    pub exit: bool,
}

impl App {
    pub fn new(params: Parameters) -> App {
        let personnage = core::generate_personnage(params);

        App {
            personnage,
            exit: false

        }

    }
    pub fn exit(&mut self) {
        self.exit = true;
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