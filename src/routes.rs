use rocket::{get, post, routes, State, form::Form};
use rocket_dyn_templates::{Template, context};
use crate::{AppState, Pokemon, TypePokemon, Genre, AjoutPokemonForm, EntrainementForm};
use rand::Rng;

use crate::{Elevage,ReproductionForm, NOMS_POKEMON};

// Fonction pour générer un nombre aléatoire d'XP entre min et max
fn generer_xp_aleatoire(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

// Fonction pour générer un nom aléatoire de Pokémon unique
fn generer_nom_aleatoire(elevage: &Elevage) -> String {
    let mut rng = rand::thread_rng();
    let mut nom;
    let mut tentatives = 0;
    let max_tentatives = 100;

    loop {
        let index = rng.gen_range(0..NOMS_POKEMON.len());
        nom = String::from(NOMS_POKEMON[index]);
        
        // Vérifier si le nom est déjà utilisé
        let nom_existe = elevage.pokemons.iter().any(|p| p.nom == nom);
        
        if !nom_existe || tentatives >= max_tentatives {
            // Si le nom n'existe pas ou si on a atteint le nombre max de tentatives
            if tentatives >= max_tentatives {
                // Ajouter un numéro au nom pour le rendre unique
                nom = format!("{} #{}", nom, rng.gen_range(1..100));
            }
            break;
        }
        
        tentatives += 1;
    }
    
    nom
}

// Fonction pour générer un genre aléatoire
fn generer_genre_aleatoire() -> Genre {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) { Genre::Male } else { Genre::Femelle }
}

#[get("/")]
pub fn index(state: &State<AppState>) -> Template {
    let elevage = state.elevage.lock().unwrap();
    Template::render("index", context! {
        pokemons: elevage.get_pokemons()
    })
}

#[get("/stats")]
pub fn stats(state: &State<AppState>) -> Template {
    let elevage = state.elevage.lock().unwrap();
    let mut types_count = std::collections::HashMap::new();
    for pokemon in &elevage.pokemons {
        *types_count.entry(pokemon.type_pokemon).or_insert(0) += 1;
    }
    
    Template::render("stats", context! {
        total_pokemon: elevage.pokemons.len(),
        niveau_moyen: if !elevage.pokemons.is_empty() {
            elevage.pokemons.iter().map(|p| p.niveau as f32).sum::<f32>() / elevage.pokemons.len() as f32
        } else {
            0.0
        },
        types_count: types_count
    })
}

#[get("/reproduction")]
pub fn reproduction_page(state: &State<AppState>) -> Template {
    let elevage = state.elevage.lock().unwrap();
    Template::render("reproduction", context! {
        pokemons: elevage.get_pokemons(),
        message: Option::<String>::None
    })
}

#[post("/reproduction", data = "<form>")]
pub fn reproduction(state: &State<AppState>, form: Form<ReproductionForm>) -> Template {
    let mut elevage = state.elevage.lock().unwrap();
    let message = match elevage.tenter_reproduction(form.pokemon1, form.pokemon2) {
        Some(bebe) => {
            elevage.ajouter_pokemon(bebe);
            "La reproduction a réussi ! Un nouveau Pokémon est né !"
        },
        None => "La reproduction a échoué."
    };
    
    Template::render("reproduction", context! {
        pokemons: elevage.get_pokemons(),
        message: message
    })
}

#[post("/pokemons/entrainer/<index>", data = "<form>")]
pub fn entrainer_pokemon(state: &State<AppState>, index: usize, form: Form<EntrainementForm>) -> Template {
    let mut elevage = state.elevage.lock().unwrap();
    let xp = if form.xp == 0 {
        generer_xp_aleatoire(10, 100)
    } else {
        form.xp
    };
    
    if elevage.entrainer_pokemon(index, xp) {
        Template::render("pokemons", context! {
            pokemons: elevage.pokemons.clone(),
            message: format!("Le Pokémon a gagné {} points d'expérience !", xp)
        })
    } else {
        Template::render("pokemons", context! {
            pokemons: elevage.pokemons.clone(),
            message: "Pokémon non trouvé".to_string()
        })
    }
}

#[post("/pokemons/entrainer-tous", data = "<_form>")]
pub fn entrainer_tous(state: &State<AppState>, _form: Form<EntrainementForm>) -> Template {
    let mut elevage = state.elevage.lock().unwrap();
    
    // Entraîner chaque Pokémon avec un nombre aléatoire d'XP
    for i in 0..elevage.pokemons.len() {
        let xp = generer_xp_aleatoire(10, 100);
        elevage.entrainer_pokemon(i, xp);
    }
    
    Template::render("pokemons", context! {
        pokemons: elevage.pokemons.clone(),
        message: "Tous les Pokémon ont été entraînés !".to_string()
    })
}

#[get("/pokemons")]
pub fn pokemons_page(state: &State<AppState>) -> Template {
    let elevage = state.elevage.lock().unwrap();
    Template::render("pokemons", context! {
        pokemons: elevage.get_pokemons()
    })
}

#[post("/pokemons/ajouter", data = "<form>")]
pub fn ajouter_pokemon(state: &State<AppState>, form: Form<AjoutPokemonForm>) -> Template {
    let mut elevage = state.elevage.lock().unwrap();
    
    // Convertir le type de Pokémon en ignorant la casse et les accents
    let type_pokemon = match form.type_pokemon.to_lowercase().as_str() {
        "feu" => TypePokemon::Feu,
        "eau" => TypePokemon::Eau,
        "plante" => TypePokemon::Plante,
        "electrik" | "électrik" => TypePokemon::Electrik,
        "normal" => TypePokemon::Normal,
        "psy" => TypePokemon::Psy,
        "combat" => TypePokemon::Combat,
        "poison" => TypePokemon::Poison,
        "sol" => TypePokemon::Sol,
        "vol" => TypePokemon::Vol,
        _ => TypePokemon::Normal,
    };
    
    // Générer aléatoirement le genre ou le nom si non spécifié
    let nom = if form.nom.is_empty() {
        generer_nom_aleatoire(&elevage)
    } else {
        form.nom.clone()
    };
    
    let genre = if form.genre.is_empty() {
        generer_genre_aleatoire()
    } else {
        match form.genre.to_lowercase().as_str() {
            "male" | "mâle" => Genre::Male,
            "femelle" => Genre::Femelle,
            _ => Genre::Male,
        }
    };
    
    // Créer et ajouter le nouveau Pokémon
    let nouveau_pokemon = Pokemon::new(nom, type_pokemon, genre);
    elevage.ajouter_pokemon(nouveau_pokemon);
    
    Template::render("pokemons", context! {
        pokemons: elevage.get_pokemons()
    })
}

#[post("/pokemons/supprimer/<index>")]
pub fn supprimer_pokemon(state: &State<AppState>, index: usize) -> Template {
    let mut elevage = state.elevage.lock().unwrap();
    
    // Vérifier si l'index est valide
    if index < elevage.pokemons.len() {
        // Supprimer le Pokémon à l'index spécifié
        elevage.pokemons.remove(index);
    }
    
    Template::render("pokemons", context! {
        pokemons: elevage.get_pokemons()
    })
}

#[get("/pokemons/trier?<critere>")]
pub fn trier_pokemons(state: &State<AppState>, critere: &str) -> Template {
    let mut elevage = state.elevage.lock().unwrap();
    match critere {
        "niveau" => elevage.trier_par_niveau(),
        "type" => elevage.trier_par_type(),
        _ => {}
    }
    Template::render("pokemons", context! {
        pokemons: elevage.get_pokemons()
    })
}

// Route pour sauvegarder l'élevage
#[post("/pokemons/sauvegarder")]
pub fn sauvegarder_elevage(state: &State<AppState>) -> Template {
    let elevage = state.elevage.lock().unwrap();
    match elevage.sauvegarder("elevage.json") {
        Ok(_) => Template::render("pokemons", context! {
            pokemons: elevage.get_pokemons(),
            message: "Élevage sauvegardé avec succès !"
        }),
        Err(e) => Template::render("pokemons", context! {
            pokemons: elevage.get_pokemons(),
            message: format!("Erreur lors de la sauvegarde : {}", e)
        })
    }
}

// Fonction pour obtenir toutes les routes
pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        index,
        stats,
        reproduction_page,
        reproduction,
        entrainer_pokemon,
        entrainer_tous,
        pokemons_page,
        ajouter_pokemon,
        supprimer_pokemon,
        trier_pokemons,
        sauvegarder_elevage
    ]
} 