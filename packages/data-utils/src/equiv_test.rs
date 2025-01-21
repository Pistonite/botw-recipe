//! Exhaustively Test 2 actors are equivalent in cooking for WMC purposes
//! That is, for all recipes:
//! - The output HP is the same
//! - The output price is the same
//! - The crit_rng_hp flag is the same
//!
//! .., when replacing the actor with the other

use std::sync::mpsc;
use std::time::Instant;

use anyhow::bail;

use botw_recipe_sys::{actor_count, group_count, Group, Actor, ActorMnr};
use botw_recipe_cook::CookResult;

use crate::util;

const CHUNK_SIZE: u64 = 28000;

pub fn check_equiv(actor_names: &[String]) -> anyhow::Result<()> {
    let mut actors = Vec::with_capacity(actor_names.len());
    for name in actor_names {
        let Some(actor) = Actor::from_actor_name(name) else {
            bail!("Invalid actor name: {}", name);
        };
        actors.push(actor);
    }
    check_equiv_actors(&actors)
}

pub fn check_all_groups() -> anyhow::Result<()> {
    let start_time = Instant::now();
    let mut to_check = Vec::new();
    // skip 0 - the None group
    for i in 1u8..group_count!() {
        let group = Group::from_u8(i).unwrap();
        let actors = group.actors();
        if actors.len() == 1 {
            continue;
        }
        to_check.push(actors);
    }
    let total = to_check.len();
    for (i, actors) in to_check.into_iter().enumerate() {
        let name = actors.iter().map(|x| x.name()).collect::<Vec<_>>().join(", ");
        println!("Checking group [{}/{}]: {}", i, total, name);
        check_equiv_actors(actors)?;
    }

    let elapsed = start_time.elapsed().as_secs_f32();

    println!("Successfully verified all groups in {:.2}s", elapsed);

    Ok(())
}

pub fn check_equiv_actors(actors: &[Actor]) -> anyhow::Result<()> {
    if actors.is_empty() {
        bail!("No actors to check");
    }
    let actor_a = actors[0];
    for actor_b in actors.iter().skip(1) {
        check_actor_pair(actor_a, *actor_b)?;
    }
    Ok(())
}

pub fn check_actor_pair(actor_a: Actor, actor_b: Actor) -> anyhow::Result<()> {
    let mnr = ActorMnr::<4>::default();
    let pool = crate::thread_pool();

    let len = mnr.len();
    let mut start = 0;
    let total = if len % CHUNK_SIZE == 0 {
        len / CHUNK_SIZE + 1
    } else {
        len / CHUNK_SIZE + 1 + 1
    };

    let progress = spp::printer(total as usize, format!("Verify {} === {}", actor_a.name(), actor_b.name()));
    let (send, recv) = mpsc::channel();

    {
        let send = send.clone();
        pool.execute(move || {
            let result = check_actor_pair_in_321(actor_a, actor_b);
            let _ = send.send(result.map_err(|x|x.to_string()));
        });
    }

    while start < len {
        let end = (start + CHUNK_SIZE).min(len);
        let mnr = mnr.clone();
        let send = send.clone();

        pool.execute(move || {
            let result = check_actor_pair_in_4(actor_a, actor_b, mnr, start, end);
            let _ = send.send(result.map_err(|x|x.to_string()));
        });

        start = end;
    }
    drop(send);
    let mut errors = Vec::new();
    for (i, result) in recv.into_iter().enumerate() {
        progress.update(i);
        match result {
            Ok(_) => {}
            Err(e) => {
                errors.push((i, e));
            }
        }
    }
    progress.done();
    util::check_errors(&errors)?;

    Ok(())
}

fn check_actor_pair_in_4(
    actor_a: Actor,
    actor_b: Actor,
    mnr: ActorMnr<4>,
    start: u64,
    end: u64
) -> anyhow::Result<()> {
    let mut input_a = [actor_a, Actor::None, Actor::None, Actor::None, Actor::None];
    let mut input_b = [actor_b, Actor::None, Actor::None, Actor::None, Actor::None];
    let mut actors = [Actor::None; 4];
    for id in start..end {
        if !mnr.to_actors(id, &mut actors) {
            bail!("Failed to get actors for id {}", id);
        }

        input_a[1..].copy_from_slice(&actors);
        input_b[1..].copy_from_slice(&actors);

        let result_a = botw_recipe_cook::cook_actors_unchecked(&input_a);
        let result_b = botw_recipe_cook::cook_actors_unchecked(&input_b);
        if let Err(e) = compare_cook_results(&result_a, &result_b) {
            return Err(e.context(format!("recipe: {:?}", input_a)));
        }
    }

    Ok(())
}
fn check_actor_pair_in_321(
    actor_a: Actor,
    actor_b: Actor,
) -> anyhow::Result<()> {
    let mnr = ActorMnr::<3>::default();
    check_actor_pair_in_3(actor_a, actor_b, mnr, 0, mnr.len())?;
    let mnr = ActorMnr::<2>::default();
    check_actor_pair_in_2(actor_a, actor_b, mnr, 0, mnr.len())?;
    check_actor_pair_in_1(actor_a, actor_b)
}

fn check_actor_pair_in_3(
    actor_a: Actor,
    actor_b: Actor,
    mnr: ActorMnr<3>,
    start: u64,
    end: u64
) -> anyhow::Result<()> {
    let mut inputs = [
        [actor_a, actor_a, Actor::None, Actor::None, Actor::None],
        [actor_a, actor_b, Actor::None, Actor::None, Actor::None],
        [actor_b, actor_b, Actor::None, Actor::None, Actor::None],
    ];
    let mut actors = [Actor::None; 3];
    let mut results = Vec::with_capacity(3);
    for id in start..end {
        if !mnr.to_actors(id, &mut actors) {
            bail!("Failed to get actors for id {}", id);
        }

        for input_mut in inputs.iter_mut() {
            input_mut[2..].copy_from_slice(&actors);
        }

        results.clear();
        for input in inputs.iter() {
            results.push(botw_recipe_cook::cook_actors_unchecked(input));
        }

        if let Err(e) = compare_cook_results_slice(&results) {
            return Err(e.context(format!("recipe: {:?}", inputs[0])));
        }
    }

    Ok(())
}

fn check_actor_pair_in_2(
    actor_a: Actor,
    actor_b: Actor,
    mnr: ActorMnr<2>,
    start: u64,
    end: u64
) -> anyhow::Result<()> {
    let mut inputs = [
        [actor_a, actor_a, actor_a, Actor::None, Actor::None],
        [actor_a, actor_b, actor_a, Actor::None, Actor::None],
        [actor_b, actor_b, actor_a, Actor::None, Actor::None],
        [actor_b, actor_b, actor_b, Actor::None, Actor::None],
    ];
    let mut actors = [Actor::None; 2];
    let mut results = Vec::with_capacity(4);
    for id in start..end {
        if !mnr.to_actors(id, &mut actors) {
            bail!("Failed to get actors for id {}", id);
        }

        for input_mut in inputs.iter_mut() {
            input_mut[3..].copy_from_slice(&actors);
        }

        results.clear();
        for input in inputs.iter() {
            results.push(botw_recipe_cook::cook_actors_unchecked(input));
        }

        if let Err(e) = compare_cook_results_slice(&results) {
            return Err(e.context(format!("recipe: {:?}", inputs[0])));
        }
    }

    Ok(())
}

fn check_actor_pair_in_1(
    actor_a: Actor,
    actor_b: Actor,
) -> anyhow::Result<()> {
    let mut inputs = [
        [actor_a, actor_a, actor_a, actor_a, Actor::None],
        [actor_a, actor_b, actor_a, actor_a, Actor::None],
        [actor_b, actor_b, actor_a, actor_a, Actor::None],
        [actor_b, actor_b, actor_b, actor_a, Actor::None],
        [actor_b, actor_b, actor_b, actor_b, Actor::None],
    ];
    let mut results = Vec::with_capacity(5);
    for id in 0u8..actor_count!() {
        let actor = Actor::from_u8(id).unwrap();
        for input_mut in inputs.iter_mut() {
            input_mut[4] = actor;
        }

        results.clear();
        for input in inputs.iter() {
            results.push(botw_recipe_cook::cook_actors_unchecked(input));
        }

        if let Err(e) = compare_cook_results_slice(&results) {
            return Err(e.context(format!("recipe: {:?}", inputs[0])));
        }
    }

    Ok(())
}

fn compare_cook_results_slice(s: &[CookResult]) -> anyhow::Result<()> {
    let first = &s[0];
    for result in s.iter().skip(1) {
        compare_cook_results(first, result)?;
    }
    Ok(())
}

#[inline]
fn compare_cook_results(a: &CookResult, b: &CookResult) -> anyhow::Result<()> {
    let a = a.get_wmc_data();
    let b = b.get_wmc_data();
    if a.crit != b.crit {
        bail!("crit type mismatch: {:?} != {:?}", a.crit, b.crit);
    }
    if a.hp != b.hp {
        bail!("hp mismatch: {:?} != {:?}", a.hp, b.hp);
    }
    if a.price != b.price {
        bail!("price mismatch: {:?} != {:?}", a.price, b.price);
    }
    Ok(())
}
