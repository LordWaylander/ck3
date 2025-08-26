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

    let education = use_signal(|| None);
    let level = use_signal(|| None);
    let age = use_signal(|| None);
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


        
        // div {
        //     select { onchange: move |e| {
        //         dbg!(&e);
        //             let mut mutable_reference = params.write();
        //             mutable_reference.education = Some(e.value());
        //         },
        //         option { disabled: true, selected: true, "Choose Your Education" }
        //         option { value: "diplomatie", "Diplomatie" }
        //         option { value: "martialite", "Martialite" }
        //         option { value: "intendance", "Intendance" }
        //         option { value: "intrigue", "Intrigue" }
        //         option { value: "erudition", "Erudition"},
        //         option { value: "random", "Random"}
        //     }
        // }
        // div {
            
        // }

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