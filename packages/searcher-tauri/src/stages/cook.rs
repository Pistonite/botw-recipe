//! Cook result

use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

use log::{error, info};
use botw_recipe_sys::{Actor, CookEffect, Group, GroupMnr};
use botw_recipe_wmcdb::TempResult;
use serde::Serialize;
use tauri::{AppHandle, State};

use crate::error::Error;
use crate::{events, Global, AbortSignal};

/// Result of grouping/merging recipes with the same values
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct OptimizedRecipeData {
    /// Actors for this recipe
    ///
    /// Each Vec is a group of actors
    actors: Vec<Vec<u8>>,
    /// Values for this recipe
    #[serde(flatten)]
    values: RecipeValues,
}

struct RecipeData {
    id: u64,
    values: RecipeValues,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[serde(rename_all = "camelCase")]
struct RecipeValues {
    value: i32,
    is_hearty: bool,
    price: i32,
}

pub fn run(app: AppHandle, state: State<Global>) -> Result<(), Error> {
    info!("starting cooking results in the background.");
    let mut cooking_handle = state.cooking_handle.lock()?;
    if let Some(handle) = cooking_handle.take() {
        let _ = state.executor.abort(handle);
    }
    let limit = state.config.result_limit;
    let mut result = None;
    {
        let filter_result = state.filter_result.lock()?;
        if let Some(filter_result) = filter_result.as_ref() {
            result = Some(filter_result.clone());
        }
    }
    if result.is_none() {
        let search_result = state.search_result.lock()?;
        if let Some(search_result) = search_result.as_ref() {
            result = Some(search_result.clone());
        }
    }
    let result = match result {
        Some(result) => result,
        None => {
            return Err(Error::MissingSearchResult);
        }
    };
    let handle = state.executor.execute_abortable(move |signal| {
        let result = cook_from_result(&result, limit, signal).map(optimize_results);
        events::emit_cook_complete(&app, result);
    })?;

    *cooking_handle = Some(handle);
    Ok(())
}

fn cook_from_result(
    result: &TempResult,
    limit: usize,
    signal: AbortSignal,
) -> Result<Vec<RecipeData>, Error> {
    let mut count = 0;
    let mut results = Vec::new();
    for reader in result.iter() {
        if signal.is_aborted() {
            return Err(Error::Aborted);
        }
        for recipe_id in reader? {
            if count >= limit {
                break;
            }
            let recipe_id = recipe_id?;
            let result = botw_recipe_cook::cook_id_unchecked(recipe_id);
            let mut value = result.data.health_recover;
            let price = result.data.sell_price;
            let is_hearty = result.data.effect_id == CookEffect::LifeMaxUp.game_repr_f32();
            if result.data.crit_chance >= 100 && !result.crit_rng_hp {
                if is_hearty {
                    value += 4;
                } else {
                    value = (value + 12).min(120);
                }
            }
            let values = RecipeValues {
                value,
                is_hearty,
                price,
            };
            results.push(RecipeData {
                id: recipe_id,
                values,
            });
            count += 1;
        }
    }
    if signal.is_aborted() {
        return Err(Error::Aborted);
    }
    Ok(results)
}

/// Group results that have 4 ingredients in common
fn optimize_results(results: Vec<RecipeData>) -> Vec<OptimizedRecipeData> {
    info!("optimizing {} cooking results", results.len());

    let mnr = GroupMnr::num_ingr();

    // key -> (id, extra_group)
    let mut recipe_map = HashMap::new();
    for data in &results {
        // extract 5 keys from each data
        for exclude in 0..botw_recipe_sys::num_ingr!() {
            let mut groups = [Group::None; botw_recipe_sys::num_ingr!()];
            if !mnr.to_groups(data.id, &mut groups) {
                // invalid id - should not happen, just ignore
                error!("invalid recipe id: {}", data.id);
                continue;
            }
            let key = RecipeKey::from_inputs(&groups, exclude, data.values.clone());
            // although materials can repeat, it's ok
            // because we dedupe them later
            let entry = recipe_map.entry(key).or_insert_with(Vec::new);
            entry.push((data.id, groups[exclude]));
        }
    }

    // take out from one with most entries
    let mut keys = recipe_map.keys().collect::<Vec<_>>();
    keys.sort_by_key(|x| Reverse(recipe_map.get(x).unwrap().len()));

    let mut optimized = Vec::new();
    let mut seen = HashSet::new();
    for key in keys {
        let mut last_actors = Vec::new();
        let mut values = None;
        for (id, extra_group) in recipe_map.get(key).unwrap() {
            if !seen.insert(id) {
                continue;
            }
            for actor in extra_group.actors() {
                last_actors.push(actor.as_u8());
            }
            if values.is_none() {
                values = Some(key.recipe_values().clone());
            }
        }
        if last_actors.is_empty() {
            continue;
        }
        let mut actors = key.to_actors();
        actors.push(last_actors);
        for actors in extract_none_actor(actors) {
            optimized.push(OptimizedRecipeData {
                actors,
                values: values.clone().unwrap(),
            });
        }
    }

    let seen_len = seen.len();
    if seen_len != results.len() {
        error!(
            "seen is different from results!! algorithm is wrong: {} != {}",
            seen_len,
            results.len()
        );
    } else {
        info!("checked recipe count after optimization is the same");
    }
    info!(
        "optimized {} into {} cooking results",
        results.len(),
        optimized.len()
    );

    optimized
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RecipeKey(Group, Group, Group, Group, RecipeValues);
impl RecipeKey {
    fn from_inputs(groups: &[Group], exclude: usize, values: RecipeValues) -> Self {
        match exclude {
            0 => Self(groups[1], groups[2], groups[3], groups[4], values),
            1 => Self(groups[0], groups[2], groups[3], groups[4], values),
            2 => Self(groups[0], groups[1], groups[3], groups[4], values),
            3 => Self(groups[0], groups[1], groups[2], groups[4], values),
            _ => Self(groups[0], groups[1], groups[2], groups[3], values),
        }
    }

    fn recipe_values(&self) -> &RecipeValues {
        &self.4
    }

    fn to_actors(&self) -> Vec<Vec<u8>> {
        let mut v = Vec::with_capacity(botw_recipe_sys::num_ingr!());
        v.extend([
            self.0.actors().iter().map(|a| a.as_u8()).collect(),
            self.1.actors().iter().map(|a| a.as_u8()).collect(),
            self.2.actors().iter().map(|a| a.as_u8()).collect(),
            self.3.actors().iter().map(|a| a.as_u8()).collect(),
        ]);
        v
    }
}

/// if any of the actors is None, will be forked into multiple outputs
///
/// For example:
/// [ [none, x], [y], [z] ] => [
///     [ [y], [z] ],
///     [ [x], [y], [z] ],
/// ]
fn extract_none_actor(mut actors: Vec<Vec<u8>>) -> Vec<Vec<Vec<u8>>> {
    let mut last = match actors.pop() {
        Some(last) => last,
        None => return vec![vec![]],
    };
    let mut recur_result = extract_none_actor(actors);
    match remove_all_nones(&mut last) {
        NoneState::HadOnlyNone => {
            // if last has only none, then don't append anything
        }
        NoneState::HadNoneAndOther => {
            // last has None and other
            let result_without_last = recur_result.clone();
            // first, append last without None to all results
            for actors in recur_result.iter_mut() {
                actors.push(last.clone());
            }
            // then append None
            recur_result.extend(result_without_last);
        }
        NoneState::NoNone => {
            // last has no None, just append it to all results
            for actors in recur_result.iter_mut() {
                actors.push(last.clone());
            }
        }
    }
    recur_result
}

enum NoneState {
    HadOnlyNone,
    HadNoneAndOther,
    NoNone,
}

fn remove_all_nones(actors: &mut Vec<u8>) -> NoneState {
    let mut removed = false;
    while let Some(pos) = actors.iter().position(|x| *x == Actor::None.as_u8() ) {
        actors.swap_remove(pos);
        removed = true;
    }
    if actors.is_empty() {
        return NoneState::HadOnlyNone;
    }
    if removed {
        return NoneState::HadNoneAndOther;
    }
    NoneState::NoNone
}
