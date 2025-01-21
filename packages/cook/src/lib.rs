mod cook_result;
pub use cook_result::*;
mod cook_data;
pub use cook_data::*;
mod cook_mgr;

mod distr;
pub use distr::*;

use botw_recipe_sys::{ Actor, ActorData, IngrVec};
use enumset::EnumSet;

#[cfg(feature = "wmcdb")]
use botw_recipe_sys::{Group, GroupMnr, num_ingr};

/// Cook using the recipe id in the recipe database.
///
/// Returns None if the id is invalid.
///
/// # Performance
/// See [`cook_id_unchecked`] if the id is guaranteed to be valid, 
/// By not wrapping the result in an Option, it's more performant
/// in perf-sensitive code.
#[cfg(feature = "wmcdb")]
pub fn cook_id(id: u64) -> Option<CookResult> {
    let result = cook_id_unchecked(id);
    if result.is_from_invalid_input() {
        None
    } else {
        Some(result)
    }
}

/// Like [`cook_id`], but returns `CookResult::no_ingredients` if id is invalid.
#[cfg(feature = "wmcdb")]
pub fn cook_id_unchecked(id: u64) -> CookResult {
    if id == 0 {
        return CookResult::no_ingredients();
    }
    let mut groups = [Group::None; num_ingr!()];
    if !GroupMnr::default().to_groups(id, &mut groups) {
        return CookResult::no_ingredients();
    }
    let mut actors = IngrVec::new();
    let mut ingrs = IngrVec::new();
    let mut unique_actors = EnumSet::default();
    for group in groups {
        let actor = group.first_actor();
        if actor == Actor::None {
            continue;
        }
        unique_actors.insert(actor);
        // max 5 because of the slice, so we don't need to check
        // if it's full
        let _ = actors.push(actor);
        let _ = ingrs.push(actor.data());
    }
    let unique_ingrs = unique_actors_to_ingrs(unique_actors);
    cook_mgr::cook_internal(actors, ingrs, unique_ingrs)
}

/// Cook using WMC groups as ingredients.
///
/// The first actor in each group is used. `None` groups are skipped.
/// Only the first 5 non-None groups are used if there are more.
///
/// If there are no non-None groups in the input, returns `None`
///
/// # Performance
/// See [`cook_groups_unchecked`] if the input is guaranteed to be valid,
/// By not wrapping the result in an Option, it's more performant
/// in perf-sensitive code.
#[cfg(feature = "wmcdb")]
pub fn cook_groups(
    groups: &[Group],
) -> Option<CookResult> {
    let result = cook_groups_unchecked(groups);
    if result.is_from_invalid_input() {
        None
    } else {
        Some(result)
    }
}

/// Like [`cook_groups`], but returns `CookResult::no_ingredients` if there are no groups.
#[cfg(feature = "wmcdb")]
pub fn cook_groups_unchecked(
    groups: &[Group],
) -> CookResult {
    let mut actors = IngrVec::new();
    let mut ingrs = IngrVec::new();
    let mut unique_actors = EnumSet::default();
    for group in groups {
        let actor = group.first_actor();
        if actor == Actor::None {
            continue;
        }
        unique_actors.insert(actor);
        if actors.push(actor).is_some() {
            break;
        }
        // checked with actors
        let _ = ingrs.push(actor.data());
    }
    if actors.is_empty() {
        return CookResult::no_ingredients();
    }
    let unique_ingrs = unique_actors_to_ingrs(unique_actors);
    cook_mgr::cook_internal(actors, ingrs, unique_ingrs)
}

/// Cook using actors (items) as input
///
/// Only the first 5 non-None actors are used if there are more.
/// If there are no non-None actors in the input, returns `None`
///
/// # Performance
/// See [`cook_actors_unchecked`] if the input is guaranteed to be valid,
/// By not wrapping the result in an Option, it's more performant
/// in perf-sensitive code.
pub fn cook_actors(actors: &[Actor]) -> Option<CookResult> {
    let result = cook_actors_unchecked(actors);
    if result.is_from_invalid_input() {
        None
    } else {
        Some(result)
    }
}

/// Like [`cook_actors`], but returns `CookResult::no_ingredients` if there are no actors.
pub fn cook_actors_unchecked(input_actors: &[Actor]) -> CookResult {
    let mut actors = IngrVec::new();
    let mut ingrs = IngrVec::new();
    let mut unique_actors = enumset::EnumSet::default();
    for actor in input_actors {
        let actor = *actor;
        if actor == Actor::None {
            continue;
        }
        unique_actors.insert(actor);
        if actors.push(actor).is_some() {
            break;
        }
        // checked with actors
        let _ = ingrs.push(actor.data());
    }
    if actors.is_empty() {
        return CookResult::no_ingredients();
    }
    let unique_ingrs = unique_actors_to_ingrs(unique_actors);
    cook_mgr::cook_internal(actors, ingrs, unique_ingrs)
}

#[inline(always)]
fn unique_actors_to_ingrs(unique_actors: EnumSet<Actor>) -> IngrVec<&'static ActorData> {
    let mut unique_ingrs = IngrVec::new();
    for actor in unique_actors {
        let _ = unique_ingrs.push(actor.data());
    }
    unique_ingrs
}
