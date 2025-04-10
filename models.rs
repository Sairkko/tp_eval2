use std::fs;
use serde::{Serialize, Deserialize};
use rand::Rng;

// Liste des noms de Pokémon pour la génération aléatoire
pub const NOMS_POKEMON: [&str; 20] = [
    "Pikachu", "Bulbizarre", "Salamèche", "Carapuce", "Évoli",
    "Mewtwo", "Dracaufeu", "Tortank", "Florizarre", "Métamorph",
    "Ronflex", "Mimiqui", "Pikachu Libre", "Pikachu Pharaon", "Pikachu Pop Star",
    "Pikachu Idole", "Pikachu Cosplay", "Pikachu Déguisé", "Pikachu Costumé", "Pikachu Déguisé"
];

// Définition des types de Pokémon
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum TypePokemon {
    Feu,
    Eau,
    Plante,
    Electrik,
    Normal,
    Psy,
    Combat,
    Poison,
    Sol,
    Vol,
}

impl TypePokemon {
    pub fn to_string(&self) -> &'static str {
        match self {
            TypePokemon::Feu => "Feu",
            TypePokemon::Eau => "Eau",
            TypePokemon::Plante => "Plante",
            TypePokemon::Electrik => "Électrik",
            TypePokemon::Normal => "Normal",
            TypePokemon::Psy => "Psy",
            TypePokemon::Combat => "Combat",
            TypePokemon::Poison => "Poison",
            TypePokemon::Sol => "Sol",
            TypePokemon::Vol => "Vol",
        }
    }
}

// Définition du genre des Pokémon
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Genre {
    Male,
    Femelle,
}

impl Genre {
    pub fn to_string(&self) -> &'static str {
        match self {
            Genre::Male => "Mâle",
            Genre::Femelle => "Femelle",
        }
    }
}

// Structure principale représentant un Pokémon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pokemon {
    pub nom: String,
    pub niveau: u32,
    pub type_pokemon: TypePokemon,
    pub experience: u32,
    pub genre: Genre,
}

impl Pokemon {
    // Constructeur pour créer un nouveau Pokémon
    pub fn new(nom: String, type_pokemon: TypePokemon, genre: Genre) -> Self {
        Pokemon {
            nom,
            niveau: 1,
            type_pokemon,
            experience: 0,
            genre,
        }
    }

    // Méthode pour gagner de l'expérience
    pub fn gagner_experience(&mut self, xp: u32) {
        self.experience += xp;
        // Vérification du niveau
        while self.experience >= self.niveau * 100 {
            self.experience -= self.niveau * 100;
            self.niveau += 1;
            println!("{} passe au niveau {}!", self.nom, self.niveau);
        }
    }

    // Méthode pour afficher les informations d'un Pokémon
    pub fn afficher(&self) {
        println!("Pokémon: {}", self.nom);
        println!("Niveau: {}", self.niveau);
        println!("Type: {}", self.type_pokemon.to_string());
        println!("Expérience: {}/{}", self.experience, self.niveau * 100);
        println!("Genre: {}", self.genre.to_string());
        println!("-------------------");
    }

    // Méthode pour vérifier si deux Pokémon peuvent se reproduire
    pub fn peut_se_reproduire(&self, autre: &Pokemon) -> bool {
        // Vérification des conditions de reproduction
        self.type_pokemon == autre.type_pokemon && // Même type
        self.genre != autre.genre && // Genres opposés
        self.niveau >= 5 && autre.niveau >= 5 // Niveau minimum requis
    }

    // Nouvelle méthode pour la reproduction
    pub fn se_reproduire(&self, autre: &Pokemon) -> Option<Pokemon> {
        if self.peut_se_reproduire(autre) {
            let mut rng = rand::thread_rng();
            let genre = if rng.gen_bool(0.5) { Genre::Male } else { Genre::Femelle };
            
            // Générer un nom aléatoire pour le bébé
            let index = rng.gen_range(0..NOMS_POKEMON.len());
            let nom = String::from(NOMS_POKEMON[index]);
            
            Some(Pokemon::new(
                nom,
                self.type_pokemon,
                genre,
            ))
        } else {
            None
        }
    }
}

// Structure pour gérer l'élevage
#[derive(Serialize, Deserialize)]
pub struct Elevage {
    pub pokemons: Vec<Pokemon>,
}

impl Elevage {
    // Constructeur
    pub fn new() -> Self {
        Elevage {
            pokemons: Vec::new(),
        }
    }

    // Ajouter un Pokémon
    pub fn ajouter_pokemon(&mut self, pokemon: Pokemon) {
        self.pokemons.push(pokemon);
    }

    // Afficher tous les Pokémon
    pub fn afficher_tous(&self) {
        println!("\n=== Liste des Pokémon de l'élevage ===");
        for (index, pokemon) in self.pokemons.iter().enumerate() {
            println!("[{}] ", index);
            pokemon.afficher();
        }
    }

    // Entraîner un Pokémon spécifique
    pub fn entrainer_pokemon(&mut self, index: usize, xp: u32) -> bool {
        if let Some(pokemon) = self.pokemons.get_mut(index) {
            pokemon.gagner_experience(xp);
            true
        } else {
            println!("Index invalide!");
            false
        }
    }

    // Entraîner tous les Pokémon
    pub fn entrainer_tous(&mut self, xp: u32) {
        println!("\n=== Entraînement de tous les Pokémon ===");
        for pokemon in &mut self.pokemons {
            pokemon.gagner_experience(xp);
        }
    }

    // Tenter une reproduction entre deux Pokémon
    pub fn tenter_reproduction(&mut self, index1: usize, index2: usize) -> Option<Pokemon> {
        if index1 >= self.pokemons.len() || index2 >= self.pokemons.len() {
            println!("Index invalide!");
            return None;
        }

        let pokemon1 = &self.pokemons[index1];
        let pokemon2 = &self.pokemons[index2];

        match pokemon1.se_reproduire(pokemon2) {
            Some(bebe) => {
                println!("{} et {} ont eu un bébé!", pokemon1.nom, pokemon2.nom);
                Some(bebe)
            }
            None => {
                println!("{} et {} ne peuvent pas se reproduire.", pokemon1.nom, pokemon2.nom);
                None
            }
        }
    }

    // Trier les Pokémon par niveau
    pub fn trier_par_niveau(&mut self) {
        self.pokemons.sort_by(|a, b| b.niveau.cmp(&a.niveau));
    }

    // Trier les Pokémon par type
    pub fn trier_par_type(&mut self) {
        self.pokemons.sort_by(|a, b| a.type_pokemon.to_string().cmp(b.type_pokemon.to_string()));
    }

    // Sauvegarder l'élevage dans un fichier
    pub fn sauvegarder(&self, chemin: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(chemin, json)
    }

    // Charger l'élevage depuis un fichier
    pub fn charger(chemin: &str) -> std::io::Result<Self> {
        let contenu = fs::read_to_string(chemin)?;
        let elevage: Elevage = serde_json::from_str(&contenu)?;
        Ok(elevage)
    }

    // Obtenir tous les Pokémon
    pub fn get_pokemons(&self) -> Vec<Pokemon> {
        self.pokemons.clone()
    }

    // Initialiser avec quelques Pokémon
    pub fn initialiser_pokemons(&mut self) {
        // Ajouter quelques Pokémon de base
        let pikachu = Pokemon::new(
            String::from("Pikachu"),
            TypePokemon::Electrik,
            Genre::Male,
        );
        self.ajouter_pokemon(pikachu);

        let bulbizarre = Pokemon::new(
            String::from("Bulbizarre"),
            TypePokemon::Plante,
            Genre::Femelle,
        );
        self.ajouter_pokemon(bulbizarre);

        let salameche = Pokemon::new(
            String::from("Salamèche"),
            TypePokemon::Feu,
            Genre::Male,
        );
        self.ajouter_pokemon(salameche);
    }
} 