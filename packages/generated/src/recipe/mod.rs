use crate::{num_ingr, Actor, CookItem, RecipeSet, Tag};
use derive_more::Deref;

mod gen;


/// Find the recipe that matches the input, or dubious food
///
/// Only the first up-to-5-non-None actors are used.
/// `is_single` MUST reflect if there is only one unique item in the input.
/// Otherwise, the result is arbitrary.
pub fn find_recipe(actors: &[Actor], is_single: bool) -> &'static Recipe {
    let input = MatchInput::new(actors);
    if is_single {
        for data in &gen::SINGLE_RECIPES {
            if data.matches(&input) {
                return &data.recipe;
            }
        }
    }

    let mut matchable_recipes = RecipeSet::default();
    for actor in input.actors() {
        matchable_recipes.union(&actor.data().matchable_recipes);
    }
    for recipe in matchable_recipes.iter() {
        if recipe.matches(&input) {
            return &recipe.recipe;
        }
    }

    &gen::DUBIOUS_RECIPE.recipe
}

pub(crate) const fn non_single_recipe_count() -> usize {
    gen::RECIPES.len()
}

pub(crate) const fn get_recipe(i: usize) -> &'static RecipeData {
    &gen::RECIPES[i]
}

/// A matched recipe result
///
/// This implements [`Deref`](std::ops::Deref) to [`CookItem`]
/// so you can access the cook item properties directly
#[derive(Debug, Clone, PartialEq, Deref)]
pub struct Recipe {
    /// The output of this recipe
    #[deref]
    pub item: CookItem,
    /// Extra hp to add to the cook result (may be negative)
    pub heart_bonus: i32,
}

impl Recipe {
    pub const fn new(item: CookItem, heart_bonus: i32) -> Self {
        Self { item, heart_bonus }
    }
}

/// Match input computed from ingredients
pub struct MatchInput {
    /// The actors
    actors: IngrArr<Actor>,
    /// Tha tags (same length as actor)
    tags: IngrArr<Tag>,

    len: usize,
}

impl MatchInput {
    pub fn new(actors: &[Actor]) -> Self {
        let mut out = Self {
            actors: [Actor::None; num_ingr!()],
            tags: [Tag::None; num_ingr!()],
            len: 0,
        };
        let mut i = 0;
        while out.len < num_ingr!() {
            match actors.get(i) {
                Some(actor) if *actor != Actor::None => {
                    out.actors[out.len] = *actor;
                    out.tags[out.len] = actor.data().recipe_tag;
                    out.len += 1;
                }
                Some(_) => {},
                _ => break,
            }
            i += 1;
        }
        out
    }

    pub fn actors(&self) -> &[Actor] {
        &self.actors[..self.len]
    }
    
    pub fn tags(&self) -> &[Tag] {
        &self.tags[..self.len]
    }
}


/// A recipe that is only used if there is only one
/// unique item in the input
pub struct SingleRecipeData {
    /// The recipe data
    pub recipe: Recipe,
    /// Tags to match for this recipe
    pub tags: SingleMatcher<Tag>,
    /// Actors to match for this recipe
    pub actors: SingleMatcher<Actor>,
}

impl SingleRecipeData {
    /// Check if this recipe matches the input
    pub fn matches(&self, input: &MatchInput) -> bool {
        // TEMP: remove monster recipes for now
        if matches!(self.recipe.item, 
            CookItem::Item_Cook_L_01 | CookItem::Item_Cook_L_02 | CookItem::Item_Cook_L_03
| CookItem::Item_Cook_L_04 | CookItem::Item_Cook_L_05
        ) {
            return false;
        }


        let mut removed = [false; num_ingr!()];
        if !self.actors.try_match(input.actors(), &mut removed) {
            return false;
        }
        if !self.tags.try_match(input.tags(), &mut removed) {
            return false;
        }
        // because there's only one unique item, we can just check if
        // the first is removed
        removed[0]
    }
}

/// A recipe that is not SingleRecipeData
pub struct RecipeData {
    /// The recipe data
    pub recipe: Recipe,
    /// Tags to match for this recipe
    pub tags: MultiMatcher<Tag>,
    /// Actors to match for this recipe
    pub actors: MultiMatcher<Actor>,
}

impl RecipeData {
    /// Check if this recipe matches the input
    pub fn matches(&self, input: &MatchInput) -> bool {
        let mut removed = [false; num_ingr!()];
        if !self.actors.try_match(input.actors(), &mut removed) {
            return false;
        }
        if !self.tags.try_match(input.tags(), &mut removed) {
            return false;
        }
        // it's ok if not all items are removed
        true
    }
}

/// Marker trait for types that can be used to match recipes (i.e. Actor and Tag)
pub trait RecipeMatch: std::fmt::Debug + PartialEq + Copy + 'static { }

/// Matcher used for SingleRecipeData
pub struct SingleMatcher<T: 'static> { 
    /// The raw recipe data
    ///
    /// The list is interpreted as [a,b,c] => [[a, b, c]].
    /// i.e. Match one group of ingredient, either a, b, or c
    data: &'static [T]
}

impl<T: RecipeMatch> SingleMatcher<T> {
    pub const fn new(data: &'static [T]) -> Self {
        Self { data }
    }

    /// Try matching the input and mark the found value as removed
    #[must_use]
    pub fn try_match(&self, value: &[T], removed: &mut IngrArr<bool>) -> bool {
        // the implementation returns false for empty, so we need to check here
        if self.data.is_empty() {
            return true;
        }
        find_first_and_remove(self.data, value, removed)
    }
}

/// Matcher used for non-single recipes
pub struct MultiMatcher<T: 'static> {
    /// The raw recipe data
    ///
    /// Flat lists in the game's data is interpreted as [a,b,c] => [[a], [b], [c]],
    /// and converted at build time
    data: &'static[&'static [T]],
}

impl<T: RecipeMatch> MultiMatcher<T> {
    pub const fn new(data: &'static[&'static [T]]) -> Self {
        Self { data }
    }

    /// Try matching the input and mark the found value as removed
    #[must_use]
    pub fn try_match(&self, value: &[T], removed: &mut IngrArr<bool>) -> bool {
        for group in self.data {
            if !find_first_and_remove(*group, value, removed) {
                return false;
            }
        }
        true
    }
}

/// Fixed-size array of ingredients
pub type IngrArr<T> = [T; num_ingr!()];

/// Find the first element in to_find that is in input,
/// then remove all occurrence of that element in the input by
/// marking it as removed in `removed`
#[inline]
fn find_first_and_remove<T: PartialEq + Copy>(to_find: &[T], input: &[T], removed: &mut IngrArr<bool>) -> bool {
    for ai in to_find {
        let ai = *ai;
        let mut found = false;
        for (i, bi) in input.iter().enumerate() {
            if *bi == ai && !removed[i] {
                removed[i] = true;
                found = true;
            }
        }
        if found {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recipe_count() {
        assert_eq!(non_single_recipe_count(), 125);
    }
}
