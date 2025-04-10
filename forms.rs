use rocket::form::FromForm;

#[derive(Debug, FromForm)]
pub struct EntrainementForm {
    pub xp: u32,
}

#[derive(Debug, FromForm)]
pub struct ReproductionForm {
    pub pokemon1: usize,
    pub pokemon2: usize,
}

#[derive(Debug, FromForm)]
pub struct AjoutPokemonForm {
    pub nom: String,
    pub type_pokemon: String,
    pub genre: String,
} 