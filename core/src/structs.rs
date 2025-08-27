use std::{collections::HashMap, fmt};
use rand::prelude::*;

#[derive(Clone, Default)]
pub struct Parameters {
    pub education: Option<String>,
    pub level: Option<i8>,
    /// @TODO !
    pub age: Option<i8>
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Default)]
pub struct Education {
    pub  name: String,
    pub level: u8,
    pub points : u16,
    pub bonus: Vec<Bonus>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Personality {
    pub name: String,
    pub points : i16,
    pub bonus: Vec<Bonus>,
    pub incompatible: Vec<String>
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct Bonus {
    pub name: String,
    pub apttitudes: i8
}

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Personnage {
    pub age: Age,
    pub education: Education,
    pub personnality: Vec<Personality>,
    pub statistiques: HashMap<String, Statistique>,
    pub points_totaux: u16
}

#[derive(Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct Age(pub i8);

impl Default for Age {
    fn default() -> Self {
        Age(25)
    }
}

impl Age {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        let age = rng.random_range(0..=70);
        Age(age)
    }
    pub fn get_score_age(&self) -> i32 {
        match self.0 {
            0..=9 => self.0 as i32 *2,
            // donc 9 = 18, pour passer à 10 on fait +4, ou *2+2
            10 => 22,
            11 => 24,
            //*2+3 */
            12 => 27,
            13 => 29,
            14 => 31,
            15 => 33,
            //+8 ?
            16 => 40,
            17 => 42,
            // autre ?
            18 => 48,
            19 => 51,
            20 => 58,
            21 => 60,
            22..=23 => 66,
            24..=28 => 67,
            29..=30 => 66,
            31 => 65,
            32 => 64,
            33 => 62,
            34 => 61,
            35 => 59,
            36 => 57,
            37 => 55,
            38 => 53,
            39 => 50,
            40 => 48,
            41 => 45,
            42 => 42,
            43 => 38,
            44 => 35,
            45 => 31,
            46 => 27,
            47 => 23,
            48 => 19,
            49 => 14,
            50..=54 => 10,
            55..=59 => 11,
            60..=69 => 6,
            70 => 0,
            _ => panic!("problème calcul age")
        }
    }
}

impl fmt::Display for Age {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

#[derive(PartialEq)]
pub enum Signe {
    Increment,
    Decrement
}


#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct Statistique {
    name: String,
    pub base: i8,
    pub bonus: i8
}

impl Statistique {
    pub fn new(name: &str) -> Statistique {
        Statistique {
            // valeur de départ de tout personnage créé de base ds le jeu
            name: String::from(name),
            base: 5,
            bonus: 0
        }
    }

    pub fn incremente_or_decremente_stats(&mut self, signe: Signe) -> i32 {
        let modifier = if signe == Signe::Decrement {-1} else {1};
        self.base = (self.base + modifier).max(0);
        self.base;

        if self.name == "prouesse" {
            Statistique::val_prouesse(self.base).into()
        } else {
            Statistique::val_stats(self.base).into()
        }

    }

    fn val_stats(val : i8) -> i8 {
        match val {
            0..=4 => 2,
            5..=8 => 4,
            9..=12 => 7,
            13..=16 => 11,
            17..=100 => 17, // a vérifier sur l'ensemble des valeurs mais flemme (regardé juqu'a 30)
            _ => 0
       }
    } 

    fn val_prouesse(val : i8) -> i8 {
        match val {
            0..=4 => 1,
            5..=8 => 2,
            9..=12 => 4,
            13..=16 => 7,
            17..=100 => 11, // a vérifier sur l'ensemble des valeurs mais flemme (regardé juqu'a 30)
            _ => 0
       }
    } 

    pub fn calcule_cout_increment(&self) -> i32 {
        if self.name == "prouesse" {
            Statistique::val_prouesse(self.base+1).into()
        } else {
            Statistique::val_stats(self.base+1).into()
        }
    }

    pub fn add_bonus_to_stats(&mut self, bonus: Bonus) {
        self.bonus += bonus.apttitudes;
    }
}