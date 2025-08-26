use dioxus::prelude::*;
use core::structs::*;
mod server;
use server::generate;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

pub fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    let mut education = use_signal(|| None);
    let mut level = use_signal(|| None);
    let mut age = use_signal(|| None);
    let mut personnage = use_signal(|| Personnage::default());

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        h1 {
            "test from guin App function, if you see it, dioxus works !"
        }
        button {
            onclick: move |_| async move {
                match generate(
                    education.read().clone(),
                    level.read().clone(),
                    age.read().clone()
                ).await {
                    Ok(data) => personnage.set(data),
                    Err(e) => {
                        println!("Erreur lors de la génération: {:?}", e);
                        // Gère l'erreur comme tu veux
                    }
                }
            },
            "GENERATE PERSONNAGE"
        }


        
        div {
            select { onchange: move |e| {
                    let mut education_mutable_reference = education.write();
                    *education_mutable_reference = if e.value() == "random" {
                        None
                    } else {
                        Some(e.value())
                    };
                },
                option { disabled: true, selected: true, "Choose Your Education" }
                option { value: "diplomatie", "Diplomatie" }
                option { value: "martialite", "Martialite" }
                option { value: "intendance", "Intendance" }
                option { value: "intrigue", "Intrigue" }
                option { value: "erudition", "Erudition"},
                option { value: "random", "Random"}
            }
        }
        div {
            select { onchange: move |e| {
                let mut level_mutable_reference = level.write();
                *level_mutable_reference = if e.value() == "random" {
                    None
                } else {
                    Some(e.value().parse().unwrap())
                };
            },
            option { disabled: true, selected: true, "Choose Your level education" }
            option { value: 0, "0" }
            option { value: 1, "1" }
            option { value: 2, "2" }
            option { value: 3, "3" }
            option { value: 4, "4"},
            option { value: 5, "5"}
            option { value: "random", "Random"}
        }
        }

        div {
            p { "personnage généré : "}
            p {"age : {personnage.read().age}"}
            p {"education : {personnage.read().education.name}"}
            p {"education level : {personnage.read().education.level}"}

            ul {
                for personalit in personnage.read().personnality.iter() {
                    li {"personnamlité : {personalit.name}"}
                }
            }

            p {"diplomatie  : {personnage.read().statistiques.diplomatie.base + personnage.read().statistiques.diplomatie.bonus}"}
            p {"martialite  : {personnage.read().statistiques.martialite.base + personnage.read().statistiques.martialite.bonus}"}
            p {"intendance  : {personnage.read().statistiques.intendance.base + personnage.read().statistiques.intendance.bonus}"}
            p {"intrigue  : {personnage.read().statistiques.intrigue.base + personnage.read().statistiques.intrigue.bonus}"}
            p {"erudition  : {personnage.read().statistiques.erudition.base + personnage.read().statistiques.erudition.bonus}"}
            p {"prouesse  : {personnage.read().statistiques.prouesse.base + personnage.read().statistiques.prouesse.bonus}"}

        }

        // p {
        //     "{params.read().education}"
        // }

    }
}