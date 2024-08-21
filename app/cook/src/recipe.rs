
use rdata::Actor;
use serde::Deserialize;

use crate::{ingr::Ingredient, tag::Tag, Error};

pub struct Recipes {
    /// Recipes for when there is only one unique ingredient
    pub single: Vec<RecipeData>,
    pub multi: Vec<RecipeData>,
    pub dubious: RecipeData,
}

pub fn read_recipes() -> Result<Recipes, Error> {
    let data = include_str!("../../../research/output/recipes.yaml");
    let data: Vec<RecipeData> = serde_yaml::from_str(data)?;
    let dubious = data.iter().find(|x| x.is_dubious()).cloned()
        .ok_or_else(|| Error::ReadRecipe("Dubious food not found".to_string()))?;
    let mut single = Vec::new();
    let mut multi = Vec::new();
        for d in data {
            if d.num == 1 {
                single.push(d);
            } else {
                multi.push(d);
            }
        }


    Ok(Recipes { single, multi, dubious })
}

impl Recipes {
    pub fn find<'a>(&'a self, items: &[&Ingredient], tags: &[Tag], unique_count: usize) -> &'a RecipeData {
        let actors = items.iter().map(|x| x.actor).collect::<Vec<_>>();

        if actors.len() != tags.len() {
            panic!("Mismatched actor and tag count: {} != {}", actors.len(), tags.len());
        }

        if unique_count == 1 {
            for recipe in &self.single {
                if recipe.matches(&actors, &tags, true, false) {
                    return recipe;
                }
            }
        }
        for recipe in &self.multi {
            if recipe.matches(&actors, &tags, false, false) {
                return recipe;
            }
        }
        return &self.dubious;
    }
}


#[derive(Clone, Deserialize)]
pub struct RecipeData {
    /// Extra hp to add to the recipe
    hb: i32,
    /// Name of the cooked item
    name: String,
    /// Tags to match
    tags: AVec<Tag>,
    /// Actors to match
    actors: AVec<Actor>,
    /// ???
    num: i32,
}

pub static ROCK_HARD: &str = "Rock-Hard Food";
pub static DUBIOUS: &str = "Dubious Food";

impl RecipeData {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_dubious(&self) -> bool {
        self.name == DUBIOUS
    }

    pub fn is_rock_hard(&self) -> bool {
        self.name == ROCK_HARD
    }

    pub fn is_fairy_tonic(&self) -> bool {
        self.name == "Fairy Tonic"
    }

    pub fn is_elixir(&self) -> bool {
        self.name == "Elixir"
    }

    pub fn get_extra_hp(&self) -> i32 {
        self.hb
    }
}

#[derive(Clone, Deserialize)]
#[serde(untagged)]
pub enum AVec<T> {
    One(Vec<T>),
    Two(Vec<Vec<T>>),
}
impl<T> AVec<T> {
    fn len(&self) -> usize {
        match self {
            Self::One(v) => v.len(),
            Self::Two(v) => v.len(),
        }
    }
    fn id(&self, i: usize) -> &[T] {
        match self {
            Self::One(v) => &v,
            Self::Two(v) => &v[i],
        }
    }
}

fn inter<T: PartialEq + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let mut c = Vec::with_capacity(a.len().max(b.len()));
    for ai in a {
        if b.iter().any(|x| x == ai) {
            c.push(ai.clone())
        }
    }
    c
}

impl RecipeData {
    pub fn matches(&self, items: &[Actor], tags: &[Tag], strict: bool, verbose: bool) -> bool {
        if verbose {
            println!("-------------------------------------");
        }
        if strict {
            if verbose {
                // println!("init name {} id {} ", self.name, self.id);
                println!("strict mode");
            }
            // let mut v: Vec<String> = items.iter().map(|x| x.to_string()).collect();
            // v.sort_unstable();
            // v.dedup();
            // if v.len() != 1 {
            //     if verbose {
            //         println!("Number of unique items != 1");
            //         println!("     items: {:?}", v);
            //     }
            //     return false;
            // }
        }
        let mut items_t = items.to_vec();
        let mut tags_t = tags.to_vec();
        if verbose {
            // println!("init name {} id {} ", self.name, self.id);
            println!("     items: {:?}", items_t);
            println!("      tags: {:?}", tags_t);
            // println!("    actors: {:?}", self.actors);
        }
        if !self.matches_actors(&mut items_t, &mut tags_t, strict, verbose) {
            return false;
        };

        if verbose {
            println!("");
            println!("     items: {:?}", items_t);
            println!("      tags: {:?}", tags_t);
            // println!("recipe tags: {:?}", self.tags);
        }
        if !self.matches_tags(&mut items_t, &mut tags_t, strict, verbose) {
            return false;
        }
        if verbose {
            println!("");
            println!("     items: {:?}", items_t);
        }
        if verbose {
            println!("done: {} {:?}", self.name, items_t);
        }
        if strict {
            return items_t.len() == 0;
        }
        return true;
    }
    fn matches_actors(
        &self,
        items_t: &mut Vec<Actor>,
        tags_t: &mut Vec<Tag>,
        strict: bool,
        verbose: bool,
    ) -> bool {
        if strict {
            if self.actors.len() == 0 {
                if verbose {
                    println!("No actors, returning current values");
                }
                return true;
            }
            let v = inter(&self.actors.id(0), &items_t);
            if v.len() == 0 {
                if verbose {
                    println!("No matching actors, returning empty");
                }
                return false;
            }
            if verbose {
                println!("Found matching actors, removing from items {:?}", v);
            }
            let v = &v[0];
            let mut k = items_t.iter().position(|x| &x == &v);
            while let Some(k_value) = k {
                items_t.remove(k_value);
                tags_t.remove(k_value);
                k = items_t.iter().position(|x| &x == &v);
            }
            if verbose {
                println!(
                    "Found matching actors, removing from items {:?} {:?}",
                    items_t, tags_t
                );
            }
            return true;
        }
        let n = self.actors.len();
        if verbose {
            println!("ACTORS {n}");
        }
        for i in 0..n {
            if verbose {
                println!("{:?} {:?}", self.actors.id(i), items_t);
            }
            let v = inter(self.actors.id(i), &items_t);
            if v.len() == 0 {
                return false;
            }
            let mut k = items_t.iter().position(|x| x == &v[0]);
            while let Some(k_value) = k {
                items_t.remove(k_value);
                tags_t.remove(k_value);
                k = items_t.iter().position(|x| x == &v[0]);
            }
        }
        return true;
    }
    fn matches_tags(
        &self,
        items_t: &mut Vec<Actor>,
        tags_t: &mut Vec<Tag>,
        strict: bool,
        verbose: bool,
    ) -> bool {
        if verbose {
            println!("    item tags: {:?}", tags_t);
        }
        if strict {
            if self.tags.len() == 0 {
                return true;
            }
            let v = inter(self.tags.id(0), &tags_t);
            if v.len() == 0 {
                return false;
            }
            let mut k = tags_t.iter().position(|x| x == &v[0]);
            while let Some(k_value) = k {
                items_t.remove(k_value);
                tags_t.remove(k_value);
                k = tags_t.iter().position(|x| x == &v[0]);
            }
            return true;
        }

        let tags = match &self.tags {
            AVec::Two(v) => v,
            AVec::One(v) => {
                if v.len() == 0 {
                    return true;
                } else {
                    panic!(":( ")
                }
            }
        };
        let n = tags.len();
        for i in 0..n {
            let mut k = None;
            for j in 0..tags[i].len() {
                if verbose {
                    println!("{:?} {:?}, {} {}", tags[i], tags_t, i, j)
                }
                k = tags_t.iter().position(|x| x == &tags[i][j]);
                if k.is_some() {
                    break;
                }
            }
            let k_value = match k {
                Some(x) => x,
                None => return false,
            };

            let item = items_t[k_value].clone();
            while let Some(k_value) = k {
                items_t.remove(k_value);
                tags_t.remove(k_value);
                k = items_t.iter().position(|x| x == &item);
            }
        }
        true
    }
}
