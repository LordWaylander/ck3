use dioxus::prelude::*;
use core::generate_personnage;
use core::structs::*;

#[server]
pub async fn generate(
    education: Option<String>,
    level: Option<i8>,
    age: Option<i8>
) -> Result<Personnage, ServerFnError> {

    let params = Parameters {
        education,
        level,
        age
    };

    let pers = generate_personnage(params, None);

    Ok(pers)
}
