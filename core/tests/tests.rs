use core::structs::{self, Parameters, Age, Education};
use core::generate_personnage;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;



    #[test]
    fn test_generate_personnage() {
        let params: Parameters = Parameters {
            education: Some("diplomatie".to_string()),
            level: Some(4) ,
            age: Some(16)
        };

        let pers = generate_personnage(params);

        assert_eq!(pers.age, Age(16));
        assert_eq!(pers.education.name, "diplomatie".to_string());
        assert_eq!(pers.education.level, 4);
    }
}