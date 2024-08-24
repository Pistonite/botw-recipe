use clap::Parser;
use enum_map::EnumMap;
use serde::{Deserialize, Serialize};
use rdata::Actor;
use rdata::cook::{CookData, CookItem};
use rdata::recipe::RecipeInputs;

pub mod ingr;

pub mod tag;
pub mod recipe;


mod cooking_pot;
pub use cooking_pot::CookingPot;


macro_rules! debugln {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug-print")]
        println!($($arg)*);
    }
}
pub(crate) use debugln;



/// BOTW Cooking Simulator CLI for WMC
#[derive(Parser, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Options {
    /// List of ingredients names in English, separated by ','.
    /// Abbrievations are allowed and the closest match will be used.
    pub ingredients: Vec<String>,
    /// Print extra information
    #[clap(short, long)]
    pub verbose: bool,

    /// Print Weapon Modifier Corruption data
    #[clap(short, long)]
    pub wmc: bool,

    /// Use recipe id instead of ingredients
    ///
    /// When chunk id is also specified, this becomes the record in the chunk
    #[clap(short, long)]
    pub id: Option<usize>,

    /// Use chunk id instead of ingredients
    #[clap(short, long)]
    pub chunk: Option<usize>,
}

impl Options {
    pub fn get_actors(&self) -> Result<Vec<Actor>, Error> {
        let id = match self.id {
            None => return self.get_actors_from_ingredients(),
            Some(id) => id
        };
        let recipe_id = if let Some(chunk) = self.chunk {
            chunk * rdata::CHUNK_SIZE + id
        } else {
            id
        };
        let inputs = RecipeInputs::from_id(recipe_id).ok_or_else(|| Error::InvalidRecipeId(recipe_id))?;
        let mut ingr = Vec::with_capacity(5);
        for group in inputs.iter() {
            let actor = group.first_actor();
            if actor != Actor::None {
                ingr.push(actor);
            }
        }
        Ok(ingr)
    }

fn get_actors_from_ingredients(&self) -> Result<Vec<Actor>, Error> {
        let mut actors = Vec::new();
        for input in self.ingredients.join(" ").split(',') {
            let input = input.trim().to_lowercase();
            // try max-common-prefix of the first word first
            let map = EnumMap::<Actor, usize>::from_fn(|x| {
                input.chars().zip(x.name().to_lowercase().chars()).take_while(|(a, b)| a == b && *a != ' ').count()
            });
            let mut max_prefix = 0;
            let mut max_actors = Vec::new();
            for (actor, prefix) in map {
                if prefix > max_prefix {
                    max_prefix = prefix;
                    max_actors.clear();
                    max_actors.push(actor);
                } else if prefix == max_prefix {
                    max_actors.push(actor);
                }
            }
            if max_actors.len() == 1 {
                actors.push(max_actors[0]);
                continue;
            }
            if max_actors.is_empty() {
                return Err(Error::ItemNotFound(input.to_string()));
            }
            // then try levenshtein distance in the case of ties
            let mut min = usize::MAX;
            let mut min_actors = Vec::new();
            for actor in &max_actors {
                let dist = distance::levenshtein(&input, &actor.name().to_lowercase());
                if dist < min {
                    min = dist;
                    min_actors.clear();
                    min_actors.push(*actor);
                } else if dist == min {
                    min_actors.push(*actor);
                }
            }
            if min_actors.len() == 1 {
                actors.push(min_actors[0]);
                continue;
            }
            if min_actors.is_empty() {
                return Err(Error::ItemNotFound(input.to_string()));
            }
            return Err(Error::AmbiguousIngr(input.to_string(), max_actors));
        }
        Ok(actors)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("failed to read ingredients for: {0:?}")]
    ReadIngr(Vec<Actor>),
    #[error("failed to read recipes: {0}")]
    ReadRecipe(String),
    #[error("attempting to get data for CookEffect::None")]
    NoEffectData,
    #[error("cannot find ingredient: {0}.")]
    ItemNotFound(String),
    #[error("ambiguous ingredient: {0}, which can be: {1:?}")]
    AmbiguousIngr(String, Vec<Actor>),
    #[error("too many ingredients! At most 5 are allowed.")]
    TooManyIngr,
    #[error("not enough ingredients! At least 1 is required.")]
    TooFewIngr,
    #[error("invalid recipe id: {0}")]
    InvalidRecipeId(usize),
    #[error("unexpected data error: {0}")]
    Data(String),
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CookResult {
    pub item: CookItem,
    pub data: CookData,
    pub crit_rng_hp: bool,
}

impl CookResult {
    pub fn new_rock_hard() -> Self {
        Self {
            item: CookItem::Item_Cook_O_02,
            data: CookData {
                health_recover: 1,
                effect_duration: 0,
                sell_price: 2,
                effect_id: -1.0,
                effect_level: 0.0,
                crit_chance: 0
            },
            crit_rng_hp: false,
        }
    }
    pub fn new_dubious(hp: i32) -> Self {
        Self {
            item: CookItem::Item_Cook_O_01,
            data: CookData {
                health_recover: hp.max(4),
                effect_duration: 0,
                sell_price: 2,
                effect_id: -1.0,
                effect_level: 0.0,
                crit_chance: 0
            },
            crit_rng_hp: false,
        }
    }
}
