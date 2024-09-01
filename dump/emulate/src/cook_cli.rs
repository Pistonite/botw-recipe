use clap::Parser;
use enum_map::EnumMap;
use rdata::cook::{CookingPot, Error};
use rdata::{Actor, RecipeInputs};
use serde::{Deserialize, Serialize};

fn main() {
    if let Err(e) = cli() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn cli() -> Result<(), Error> {
    let options = Options::parse();
    let actors = match options.get_actors() {
        Ok(actors) => actors,
        Err(Error::AmbiguousIngr(input, actors)) => {
            println!("Ambiguous ingredient: {}", input);
            println!("Possible actors: {:#?}", actors);
            return Ok(());
        }
        Err(e) => return Err(e),
    };
    println!("Ingredients are:");
    for actor in &actors {
        println!("  - {:?}", actor);
    }
    let pot = CookingPot::new()?;
    let output = pot.cook(actors)?;
    println!("Cooked: {:#?}", output);

    Ok(())
}

/// BOTW Cooking Simulator CLI for WMC
#[derive(Parser, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Options {
    /// List of ingredients names in English, separated by ','.
    /// Abbrievations are allowed and the closest match will be used.
    pub ingredients: Vec<String>,

    /// Print Weapon Modifier Corruption data (WIP)
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

    /// Use compact chunking instead of raw chunking
    #[clap(short = 'C', long)]
    pub compact: bool,
}

impl Options {
    pub fn get_actors(&self) -> Result<Vec<Actor>, Error> {
        let id = match self.id {
            None => return self.get_actors_from_ingredients(),
            Some(id) => id,
        };
        let recipe_id = if let Some(chunk) = self.chunk {
            if self.compact {
                chunk * rdata::COMPACT_CHUNK_SIZE + id
            } else {
                chunk * rdata::CHUNK_SIZE + id
            }
        } else {
            id
        };
        let inputs =
            RecipeInputs::from_id(recipe_id).ok_or(Error::InvalidRecipeId(recipe_id))?;
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
                input
                    .chars()
                    .zip(x.name().to_lowercase().chars())
                    .take_while(|(a, b)| a == b && *a != ' ')
                    .count()
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
            return Err(Error::AmbiguousIngr(
                input.to_string(),
                max_actors.into_iter().map(|x| format!("{x:?}")).collect(),
            ));
        }
        Ok(actors)
    }
}
