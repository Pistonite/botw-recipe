use serde::Deserialize;

use botw_recipe_generated::{Actor, CookItem};

use super::{Error, Tag};

/// Recipes for cooking
#[derive(Debug, Clone, PartialEq)]
pub struct Recipes {
    /// Recipes for when there is only one unique ingredient
    pub single: Vec<RecipeData>,
    /// Other recipes
    pub multi: Vec<RecipeData>,
    /// Dubious food recipe
    pub dubious: RecipeData,
}

pub fn read_recipes() -> Result<Recipes, Error> {
    let data = include_str!("../../../../research/output/recipes.yaml");
    let data: Vec<RecipeData> = serde_yaml_ng::from_str(data)?;
    let dubious = data
        .iter()
        .find(|x| x.is_dubious())
        .cloned()
        .ok_or_else(|| Error::ReadRecipe("Dubious food not found".to_string()))?;
    let mut single = Vec::with_capacity(data.len());
    let mut multi = Vec::with_capacity(data.len());
    for d in data {
        if d.num == 1 {
            d.actors.check_single()?;
            d.tags.check_single()?;
            single.push(d);
        } else {
            multi.push(d);
        }
    }
    single.shrink_to_fit();
    multi.shrink_to_fit();

    Ok(Recipes {
        single,
        multi,
        dubious,
    })
}

impl Recipes {
    pub fn find<'a>(
        &'a self,
        actors: &[Actor],
        tags: &[Tag],
        unique_count: usize,
    ) -> Result<&'a RecipeData, Error> {
        if actors.len() != tags.len() {
            return Err(Error::Data(format!(
                "Mismatched actor and tag count: {} != {}",
                actors.len(),
                tags.len()
            )));
        }

        if unique_count == 1 {
            for recipe in &self.single {
                if recipe.matches_single(actors, tags) {
                    return Ok(recipe);
                }
            }
        }
        for recipe in &self.multi {
            if recipe.matches(actors, tags) {
                return Ok(recipe);
            }
        }
        Ok(&self.dubious)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RecipeData {
    /// Extra hp to add to the recipe
    hb: i32,
    /// Name of the cooked item
    name: CookItem,
    /// Tags to match
    tags: Matcher<Tag>,
    /// Actors to match
    actors: Matcher<Actor>,
    /// 1 if this is a recipe for one unique item in the input
    num: i32,
}

type RemovedFlags = [bool; crate::NUM_INGR];

impl RecipeData {
    pub fn item(&self) -> CookItem {
        self.name
    }

    pub fn is_dubious(&self) -> bool {
        self.name == CookItem::Item_Cook_O_01
    }

    pub fn is_rock_hard(&self) -> bool {
        self.name == CookItem::Item_Cook_O_02
    }

    pub fn is_fairy_tonic(&self) -> bool {
        self.name == CookItem::Item_Cook_C_16
    }

    pub fn is_elixir(&self) -> bool {
        self.name == CookItem::Item_Cook_C_17
    }

    pub fn get_extra_hp(&self) -> i32 {
        self.hb
    }

    pub fn matches_single(&self, items: &[Actor], tags: &[Tag]) -> bool {
        let mut removed = [false; crate::NUM_INGR];
        if !&self.actors.matches_single(items, &mut removed) {
            return false;
        }
        if !&self.tags.matches_single(tags, &mut removed) {
            return false;
        }
        // because there's only one unique item, we can just check if
        // the first is removed
        removed[0]
    }

    pub fn matches(&self, items: &[Actor], tags: &[Tag]) -> bool {
        let mut removed = [false; crate::NUM_INGR];
        if !&self.actors.matches_multiple(items, &mut removed) {
            return false;
        }
        if !&self.tags.matches_multiple(tags, &mut removed) {
            return false;
        }
        // it's ok if not all items are removed
        true
    }
}

/// Matcher for tags or actors in recipe
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Matcher<T: std::fmt::Debug + PartialEq + Copy> {
    /// Matcher defined as a flat list
    ///
    /// Note this has different meaning when the recipe is single
    Flat(Vec<T>),
    /// Matcher defined as a nested list
    Nested(Vec<Vec<T>>),
}
impl<T: std::fmt::Debug + PartialEq + Copy> Matcher<T> {
    pub fn check_single(&self) -> Result<(), Error> {
        match self {
            Self::Flat(_) => Ok(()),
            Self::Nested(v) => Err(Error::Data(format!(
                "Unexpected multi matcher for single recipe: {:?}",
                v
            ))),
        }
    }

    /// Check if the matcher matches the values, and remove the matched values
    ///
    /// The flat list is interpreted as [a,b,c] => [[a, b, c]]
    pub fn matches_single(&self, values: &[T], removed: &mut RemovedFlags) -> bool {
        let v = match self {
            Self::Flat(v) => v,
            Self::Nested(_) => unreachable!(), // we checked this when loading
        };
        if v.is_empty() {
            return true;
        }
        find_first_and_remove(v, values, removed)
    }

    /// Check if the matcher matches the values, and remove the matched values
    ///
    /// The flat list is interpreted as [a,b,c] => [[a], [b], [c]]
    pub fn matches_multiple(&self, values: &[T], removed: &mut RemovedFlags) -> bool {
        match self {
            Self::Flat(v) => {
                for x in v {
                    if !find_first_and_remove(&[*x], values, removed) {
                        return false;
                    }
                }
            }
            Self::Nested(v) => {
                for group in v {
                    if !find_first_and_remove(group, values, removed) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

/// Find the first element in a that is in b
fn find_first_and_remove<T: PartialEq + Copy>(a: &[T], b: &[T], r: &mut RemovedFlags) -> bool {
    for ai in a {
        let ai = *ai;
        let mut found = false;
        for (i, bi) in b.iter().enumerate() {
            if *bi == ai && !r[i] {
                r[i] = true;
                found = true;
            }
        }
        if found {
            return true;
        }
    }
    false
}
