use clap::Parser;
use core::generate_personnage;

use core::structs::*;

/// Simple program to generate a ck3 player
#[derive(Parser, Debug, Default)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Possible values : [martialite, diplomatie, intrigue, intendance, erudition]
    #[arg(short, long)]
    pub education: Option<String>,
    /// Possible values : [1, 2, 3, 4, 5]
    #[arg(short, long)]
    pub level: Option<i8>,
    /// 0 to 70 years old
    #[arg(short, long)]
    pub age: Option<i8>
}

fn get_params() -> Parameters {
    let args = Args::parse();

    let params = Parameters {
        education: args.education,
        level: args.level,
        age: args.age
    };

    params
}

fn display_personnage(personnage: Personnage) {
    println!(" *** age ***");
    println!("age : {}", personnage.age);

    println!(" *** education ***");
    println!("education : {}", personnage.education.name);
    println!("level : {}", personnage.education.level);

    println!(" *** personnality ***");
    for personalit in personnage.personnality {
        println!("{}", personalit.name);
    }
    
    println!(" *** statistiques ***");
    for (key, val) in personnage.statistiques.iter() {
        println!("{key} : {}", val.base + val.bonus);
    }

    println!("points_totaux : {}", personnage.points_totaux);
}

pub fn main() {
    let params = get_params();
    let personnage = generate_personnage(params);
    display_personnage(personnage);
}
