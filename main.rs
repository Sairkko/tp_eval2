use rocket::launch;
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;
use std::sync::Mutex;

// Importation des modules
mod models;
mod routes;
mod forms;

// Ré-exportation des types pour les rendre disponibles dans le crate
pub use models::{Pokemon, TypePokemon, Genre, Elevage, NOMS_POKEMON};
pub use forms::{EntrainementForm, ReproductionForm, AjoutPokemonForm};

// Structure pour encapsuler l'élevage dans un Mutex pour un accès thread-safe
pub struct AppState {
    pub elevage: Mutex<Elevage>,
}

#[launch]
fn rocket() -> Rocket<Build> {
    // Essayer de charger l'élevage depuis le fichier
    let elevage = match Elevage::charger("elevage.json") {
        Ok(elevage_charge) => elevage_charge,
        Err(_) => {
            // Si le fichier n'existe pas ou s'il y a une erreur, créer un nouvel élevage
            let mut nouvel_elevage = Elevage::new();
            nouvel_elevage.initialiser_pokemons();
            nouvel_elevage
        }
    };

    rocket::build()
        .mount("/", routes::get_routes())
        .attach(Template::fairing())
        .manage(AppState {
            elevage: Mutex::new(elevage),
        })
} 