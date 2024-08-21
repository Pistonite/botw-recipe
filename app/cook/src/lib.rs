use clap::Parser;
use enum_map::EnumMap;
use serde::{Deserialize, Serialize};
use rdata::{cook::{CookData, CookEffect}, Actor, RecipeInputs};

pub mod ingr;
use ingr::{Ingredient, Ingredients};
pub mod effect;
pub mod tag;
pub mod recipe;
use recipe::Recipes;

use crate::{effect::CookEffectData, tag::Tag};

macro_rules! reference {
    ($comment:literal, $dummy:ty, $name:ty) => {
    };
}

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
        ingr.clear();
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
}

pub struct CookingPot {
    recipes: Recipes,
    ingredients: Ingredients,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CookResult {
    pub name: String,
    pub data: CookData,
}

impl CookResult {
    pub fn new_rock_hard() -> Self {
        Self {
            name: recipe::ROCK_HARD.to_string(),
            data: CookData {
                health_recover: 1,
                effect_duration: 0,
                sell_price: 2,
                effect_id: -1.0,
                effect_level: 0.0,
                crit_chance: 0
            }
        }
    }
    pub fn new_dubious(hp: i32) -> Self {
        Self {
            name: recipe::DUBIOUS.to_string(),
            data: CookData {
                health_recover: hp.max(4),
                effect_duration: 0,
                sell_price: 2,
                effect_id: -1.0,
                effect_level: 0.0,
                crit_chance: 0
            }
        }
    }
}

/// Each item's hp is multipled by 2 in the result
const HP_MULTIPLIER: i32 = 2;
/// Price scale for number of ingredients
static PRICE_SCALE: [f32; 6] = [0.0, 1.5, 1.8, 2.1, 2.4, 2.8];
/// Base crit chance for number of unique ingredients
static BASE_CRIT_CHANCES: [i32; 5] = [5, 10, 15, 20, 25];

impl CookingPot {
    pub fn new() -> Result<Self, Error> {
        let recipes = recipe::read_recipes()?;
        let ingredients = ingr::read_ingredients()?;

        Ok(Self { recipes, ingredients })
    }
    pub fn cook<A: IntoIterator<Item=T>, T: AsRef<str>>(&self, names: A) -> Result<CookResult, Error> {
        let mut actors = Vec::with_capacity(5);
        for name in names {
            let name = name.as_ref();
            let actor = Actor::try_from(name).ok_or_else(|| Error::ItemNotFound(name.to_string()))?;
            actors.push(actor);
        }
        self.cook_actors(actors)
    }

    pub fn cook_id(&self, id: usize) -> Result<CookResult, Error> {
        let inputs = RecipeInputs::from_id(id).ok_or_else(|| Error::InvalidRecipeId(id))?;
        let mut ingr = Vec::with_capacity(5);
        for group in inputs.iter() {
            let actor = group.first_actor();
            if actor != Actor::None {
                ingr.push(actor);
            }
        }
        self.cook_actors(ingr)
    }

    pub fn cook_actors<A: IntoIterator<Item=T>, T: Into<Actor>>(&self, actors: A) -> Result<CookResult, Error> {
        let ingrs = actors
            .into_iter()
            .filter_map(|x|{ 
                let x = x.into();
                (x != Actor::None).then_some(&self.ingredients[x])
            })
            .collect::<Vec<_>>();
        if ingrs.len() > 5 {
            return Err(Error::TooManyIngr);
        }
        if ingrs.is_empty() {
            return Err(Error::TooFewIngr);
        }
        let tags = ingrs.iter().map(|x| {
            let useful_tags = x.tags.iter().filter(|x| x.is_probably_useful()).collect::<Vec<_>>();
            useful_tags.get(0).map(|x|**x).unwrap_or_default()
        }).collect::<Vec<_>>();
        let mut unique_ingrs: Vec<&Ingredient> = Vec::with_capacity(5);
        for ingr in &ingrs {
            if !unique_ingrs.iter().any(|x| x.actor == ingr.actor) {
                unique_ingrs.push(ingr);
            }
        }
        let recipe = self.recipes.find(&ingrs, &tags, unique_ingrs.len());

        if recipe.is_rock_hard() {
            return Ok(CookResult::new_rock_hard());
        }

        // handle effect
        let effect = {
            let mut effect = None;
            for ingr in &ingrs {
                if ingr.effect == CookEffect::None {
                    continue;
                }
                match effect {
                    Some(e) if e != ingr.effect => {
                        // multiple effects -> becomes none
                        effect = None;
                        break;
                    },
                    None => effect = Some(ingr.effect),
                    _ => (),
                }
            }
            effect.unwrap_or(CookEffect::None)
        };
        let effect_data: Option<CookEffectData> = effect.try_into().ok();

        // handle other properties
        let (mut time, mut hp, potency, sell_price) = 
        {
            let mut time = 0;
            let base_time = effect_data.map(|x| x.base_time).unwrap_or_default();

            let mut hp = 0;
            let mut effect_level = 0;
            let mut sell_price = 0;
            let mut buy_price = 0;
            for ingr in &ingrs {
                let is_item_effect = ingr.effect == effect;
                if effect.uses_time() {
                    // every ingredient adds 30s
                    time += 30;
                    if is_item_effect {
                        // effect time is added only if effect matches
                        time += base_time;
                    }
                }

                if is_item_effect {
                    effect_level += ingr.effect_level;
                }
                hp += ingr.hp;
                if ingr.tags.contains(&Tag::CookLowPrice) {
                    sell_price += 1;
                    buy_price += 1;
                } else {
                    sell_price += ingr.sell_price;
                    buy_price += ingr.buy_price;
                }
            }
            // handle sell price
            let sp_scale32 = sell_price as f32 * PRICE_SCALE[ingrs.len()];
            sell_price = ((sp_scale32.floor() / 10.).ceil() * 10.) as i32;

            // Selling price is capped at buying price and a limited to a min of 2
            sell_price = sell_price.max(2).min(buy_price);

            ( time, hp, effect_level, sell_price )
        };

        reference!("elixir with no effect gets turned into dubious", uking::CookingMgr::cookCalcIngredientsBoost(), is_medicine);
        if recipe.is_dubious() || (recipe.is_elixir() && effect == CookEffect::None) {
            return Ok(CookResult::new_dubious(hp));
        }

        // hp multiplier/boosts are added after dubious food
        reference!("only unique ingredients add boosts", uking::CookingMgr::cookCalcIngredientsBoost(), ingredients);
        hp *= HP_MULTIPLIER;
        for ingr in &unique_ingrs {
            hp += ingr.boost_hp;
        }

        reference!("enemy spice boosts before after fairy tonic)", uking::CookingMgr::cookCalcIngredientsBoost(), tag::CookEnemy);
        reference!("boost is only added if an effect is found", uking::CookingMgr::cookCalcIngredientsBoost(), effect_found);
        reference!("boost is only added if effect has a base time", uking::CookingMgr::cookCalcIngredientsBoost(), boost_time);
        if effect.uses_time() {
            reference!("each ingredient add boost, not just unique", uking::CookingMgr::cookCalcIngredientsBoost(), count);
            for ingr in &ingrs {
                if !ingr.tags.contains(&Tag::CookEnemy) {
                    continue;
                }
                reference!("this time boost is cleared if no effect", uking::CookingMgr::cookCalcIngredientsBoost(), effect_time);
                time += ingr.boost_effect_time;
            }
        }

        reference!("fairy tonic special case", uking::CookingMgr::cookCalcIngredientsBoost(), is_not_fairy_tonic);
        let (effect, sell_price) = if recipe.is_fairy_tonic() {
            time = 0;
            (CookEffect::None, 2)
        } else {
            (effect, sell_price)
        };

        // add time boosts
        reference!("spice boosts added after fairy tonic (i.e. ingredients boost)", uking::CookingMgr::cook(), cookCalcSpiceBoost);
        reference!("only unique ingredients add boosts", uking::CookingMgr::cookCalcSpiceBoost(), ingredients);
        for ingr in &unique_ingrs {
            // this check is relevant because CookEnemy usually have boost for elixirs
            // and those are added above
            reference!("only non-CookEnemy CookSpice adds boost", uking::CookingMgr::cookCalcSpiceBoost(), InfoData::hasTag);
            if ingr.tags.contains(&Tag::CookEnemy) || !ingr.tags.contains(&Tag::CookSpice) {
                continue;
            }
            reference!("time boost is always added, even if effect is not timed", uking::CookingMgr::cookCalcSpiceBoost(), effect_time);
            time += ingr.boost_effect_time;
        }

        reference!("recipe extra hp boost", uking::CookingMgr::cookCalcRecipeBoost(), life_recover);
        hp += recipe.get_extra_hp();

        reference!("no effect min hp to ensure food does something", uking::CookingMgr::cookAdjustItem(), life_recover);
        if effect == CookEffect::None && hp == 0 {
            hp = 1;
        }
        reference!("max life recover", uking::CookingMgr::cookAdjustItem(), life_recover_max);
        hp = hp.min(120); // 30 hearts
        reference!("max time", uking::CookingMgr::cookAdjustItem(), sead::Mathi::clamp);
        time = time.min(1800); // 30 minutes

        // handle crit chance
        let crit_chance = 
            unique_ingrs.iter()
            .map(|x| x.boost_success_rate)
            .max().unwrap_or_default();
        // note that game doesn't cap crit_chance
        let crit_chance = crit_chance + BASE_CRIT_CHANCES[unique_ingrs.len()-1];

        // handle effect level
        let effect_level = if let Some(data) = effect_data {
            if effect.uses_potency() {
                if potency >= data.potency_lv3 {
                    3
                } else if potency >= data.potency_lv2 {
                    2
                } else {
                    1
                }
            } else {
                0
            }
        } else {
            0
        };

        // handle special effect_level fields
        let effect_level = match effect {
            CookEffect::LifeMaxUp => {
                // Hearty, effect_level is 
                // number of yellow quarter-heart = potency
                // note that it's rounded down to nearest whole heart
                // hp also becomes the value
                let yellow_hearts = potency / 4;
                hp = yellow_hearts * 4;
                hp
            }
            CookEffect::GutsRecover => {
                // stamina - one wheel is 1000
            let table = [0.0, 0.2, 0.4, 0.8, 1.0, 1.4, 1.6, 1.8, 2.2, 2.4, 2.8, 3.0];
                let p = potency as usize;
                let wheels = if p >= table.len() {
                    table[table.len()-1]
                } else {
                    table[p]
                };
                (wheels * 1000.0) as i32
            },
            CookEffect::ExGutsMaxUp => {
                // endura - one wheel is 5
                match potency {
                    0 => 0,
                    1..4 => 1,
                    4..6 => 2,
                    6..8 => 3,
                    8..10 => 4,
                    10..12 => 5,
                    12..14 => 6,
                    14..16 => 7,
                    16..18 => 8,
                    18..20 => 9,
                    _ => 10
                }
            }
            _ => effect_level,
        };

        let mut name = String::new();
        if let Some(data) = effect_data {
            let effect_name = data.name;
            if !effect_name.is_empty() {
                name.push_str(effect_name);
                name.push(' ');
            }
        }
        name.push_str(recipe.name());

        let data = CookData {
            health_recover: hp,
            effect_duration: time,
            sell_price,
            effect_id: effect.game_repr_f32(),
            effect_level: effect_level as f32,
            crit_chance,
        };

        Ok(CookResult { name, data })
    }
}
