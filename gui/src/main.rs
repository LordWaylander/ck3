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
            //option { value: 0, "0" }
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
            p {"points : {personnage.read().points_totaux}"}
            p {"age : {personnage.read().age}"}
            p {"education : {personnage.read().education.name}"}
            p {"education level : {personnage.read().education.level}"}

            ul {
                for personalit in personnage.read().personnality.iter() {
                    li {"personnalité : {personalit.name}"}
                }
            }

            div {
                for (key, statistique) in personnage.read().statistiques.clone() {
                    div {
                        class:"flex",
                        p {"{statistique.name} :"}
                        div {
                            class:"flex flex-align-items",
                            button {
                                style: "margin-left: 5px;margin-right: 5px;",
                                onclick: {
                                    let key = key.clone();
                                    move |_| {
                                        let mut p = personnage.write();
                                        let pts = p.statistiques.get(&key).unwrap().calcule_cout_decrement();
                                        p.points_totaux -= pts as u16;

                                        let s = p.statistiques.get_mut(&key).unwrap();
                                        s.base -= 1;
                                    }
                                },
                                "-"
                            }
                            p {"{statistique.base + statistique.bonus}"}
                            // p {"base {statistique.base}"}
                            // p {"bonus {statistique.bonus}"}
                            button {
                                style: "margin-left: 5px;margin-right: 5px;",
                                onclick: {
                                    let key = key.clone();
                                    move |_| {
                                        let mut p = personnage.write();
                                        let pts = p.statistiques.get(&key).unwrap().calcule_cout_increment();
                                        p.points_totaux += pts as u16;

                                        let s = p.statistiques.get_mut(&key).unwrap();
                                        s.base += 1;
                                    }
                                },
                                "+"
                            }
                        }
                    }
                }
            }
        }

        // p {
        //     "{params.read().education}"
        // }

    }
}