//use core::generate_personnage;
#[cfg(feature = "cli")]
use cli;
#[cfg(any(feature = "web", feature = "desktop"))]
use gui;


fn main() {
    //println!("Hello !");
    #[cfg(feature = "cli")]
    cli::main();

    #[cfg(any(feature = "web", feature = "desktop"))]
    gui::main();
}