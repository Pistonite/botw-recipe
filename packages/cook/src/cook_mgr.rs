use botw_recipe_sys::{
    Actor, ActorData, 
    CookEffect, 
    IngrVec, Recipe, Tag};

use super::{CookResult, CookDataConstPart, CookDataRngPart, Discrete, Distr, distr};

macro_rules! debugln {
    ($($arg:tt)*) => {
        #[cfg(feature = "print")]
        eprintln!($($arg)*);
    }
}

// these can be generated from cook-system.yaml

/// Each item's hp is multipled by 2 in the result
const HP_MULTIPLIER: i32 = 2;
/// Price scale for number of ingredients
static PRICE_SCALE: [f32; 6] = [0.0, 1.5, 1.8, 2.1, 2.4, 2.8];
/// Base crit chance for number of unique ingredients
static BASE_CRIT_CHANCES: [i32; 5] = [5, 10, 15, 20, 25];
/// Time boost for crit
const SUPER_SUCCESS_ADD_EFFECT_TIME: i32 = 300;


macro_rules! reference {
    ($dummy:ty) => {};
    ($comment:literal, $dummy:ty) => {};
    ($comment:literal, $dummy:ty, $name:ty) => {};
}

pub(crate) fn cook_internal(
    actors: IngrVec<Actor>, 
    ingrs: IngrVec<&ActorData>,
    unique_ingrs: IngrVec<&ActorData>,
) -> CookResult {

    let recipe = botw_recipe_sys::find_recipe(actors.as_slice(), unique_ingrs.len() == 1);

    debugln!("Recipe: {:?}", recipe);

    let mut output_const = CookDataConstPart::default();
    let mut output_rng = CookDataRngPart::default();
    let is_dubious = calc_ingredient_boost(
        ingrs.as_slice(), 
        recipe, &mut output_const, &mut output_rng);

    reference!(uking::CookingMgr::isCookFailure());
    if is_dubious || recipe.is_rock_hard() {
        reference!(uking::CookingMgr::cookFail());
        if is_dubious {
            return CookResult::new_dubious(output_rng.health_recover);
        } else {
            return CookResult::new_rock_hard();
        }
    }

    // Sell price is independent of other properties
    // so we calculate it first. (It's calculated last in game code)
    output_const.sell_price = {
        if recipe.is_fairy_tonic() {
            2
        } else {
            let mut sell_price = 0;
            let mut buy_price = 0;
            for ingr in ingrs.as_slice() {
                if ingr.tags.contains(Tag::CookLowPrice) {
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
            sell_price.max(2).min(buy_price)
        }
    };

    // Even though it's inefficient to do crit first, we must do that,
    // since crit RNG depends on if HP/effect is maxed BEFORE spice and recipe
    // boost are applied.
    //
    // In other words, if a recipe has HP maxed after recipe/spice boost,
    // but not before, it could still HP crit even though the crit doesn't do anything
    let output_rng = calc_crit_boost(unique_ingrs.as_slice(), &mut output_const, output_rng);


    // Spice Boost
    // In game code, this is after crit boost. we move it to before
    // crit so the computation is less

    let mut time_boost = 0;
    let mut hp_boost = 0;
    reference!(
        "spice boosts added after fairy tonic (i.e. ingredients boost)",
        uking::CookingMgr::cook(),
        cookCalcSpiceBoost
    );
    reference!(
        "only unique ingredients add boosts",
        uking::CookingMgr::cookCalcSpiceBoost(),
        ingredients
    );
    for ingr in unique_ingrs.as_slice() {
        // this check is relevant because CookEnemy usually have boost for elixirs
        // and those are added above
        reference!(
            "only non-CookEnemy CookSpice adds boost",
            uking::CookingMgr::cookCalcSpiceBoost(),
            InfoData::hasTag
        );
        if ingr.tags.contains(Tag::CookEnemy) || !ingr.tags.contains(Tag::CookSpice) {
            continue;
        }
        reference!(
            "time boost is always added, even if effect is not timed",
            uking::CookingMgr::cookCalcSpiceBoost(),
            effect_time
        );
        time_boost += ingr.boost.effective_time;
        hp_boost += ingr.boost.hit_point_recover;
        debugln!(
            "adding {} hp_boost from {}, now {} ",
            ingr.boost.hit_point_recover,
            ingr.actor,
            hp_boost
        );
    }

    let output_rng = output_rng.map(|mut output_rng| {
        // Apply Spice Boost
        output_rng.health_recover += hp_boost;
        output_rng.effect_duration += time_boost;

        // Recipe Boost

        reference!(
            "recipe extra hp boost",
            uking::CookingMgr::cookCalcRecipeBoost(),
            life_recover
        );
        output_rng.health_recover += recipe.heart_bonus;

        // Adjust Item
        reference!(
            "no effect min hp to ensure food does something",
            uking::CookingMgr::cookAdjustItem(),
            life_recover
        );
        if output_const.effect == CookEffect::None && output_rng.health_recover == 0 {
            output_rng.health_recover = 1;
            debugln!("hp is 0 and food has no effect, setting hp to 1");
        }

        reference!(
            "max life recover",
            uking::CookingMgr::cookAdjustItem(),
            life_recover_max
        );
        debugln!("hp is {}, and will be capped at 120", output.health_recover);
        output_rng.health_recover = output_rng.health_recover.min(120); // 30 hearts
        reference!(
            "max time",
            uking::CookingMgr::cookAdjustItem(),
            sead::Mathi::clamp
        );
        output_rng.effect_duration = output_rng.effect_duration.min(1800); // 30 minutes
        
        if output_const.effect != CookEffect::None {
            // effect level could be too high due to crit, so it's capped here
            let effect_max = output_const.effect.max_level();
            if output_rng.effect_level > effect_max as f32 {
                output_rng.effect_level = effect_max as f32;
            }

            if output_const.effect == CookEffect::GutsRecover {
                output_rng.effect_level *= 200.0;
            } else if output_const.effect == CookEffect::LifeMaxUp {
                if (output_rng.effect_level as i32) % 4 != 0 {
                    // round up to whole heart
                    output_rng.effect_level = ((output_rng.effect_level as i32 + 4) & !3) as f32;
                }
                #[cfg(feature = "assertions")]
                {
                    assert!(output_rng.effect_level >= 4.0)
                }
                output_rng.health_recover = output_rng.effect_level as i32;
            }
        }

        distr::always(output_rng)
    });

    CookResult {
        item: recipe.item,
        const_data: output_const,
        rng_data: output_rng,
    }
}

// this function is separate because it's before crit is handled
// so we want to match the game
// returns if changed to dubious
reference!(uking::CookingMgr::cookCalcIngredientsBoost);
fn calc_ingredient_boost(
    ingrs: &[&ActorData],
    recipe: &Recipe,
    output_const: &mut CookDataConstPart,
    output_rng: &mut CookDataRngPart
) -> bool {
    // doesn't hurt if we calculate effect early
    let (mut effect, has_multiple_effect) = {
        let mut effect = None;
        let mut has_multiple_effect = false;
        for ingr in ingrs {
            if ingr.effect == CookEffect::None {
                continue;
            }
            match effect {
                Some(e) if e != ingr.effect => {
                    debugln!("Multiple effects found: {:?} and {:?}", e, ingr.effect);
                    debugln!("Effect will be none");
                    // multiple effects -> becomes none
                    effect = None;
                    has_multiple_effect = true;
                    break;
                }
                None => {
                    effect = Some(ingr.effect);
                    debugln!("Found effect: {:?}", effect);
                }
                _ => (),
            }
        }
        (effect.unwrap_or(CookEffect::None), has_multiple_effect)
    };

    let base_time = effect.base_time_i32();
    debugln!("The base time for effect is: {}", base_time);

    let mut hp = 0;
    let mut time = 0;
    let mut max_hp_boost = 0;

    // normally unused, but in the game's code
    #[cfg(feature = "assertions")]
    let mut stam_boost = 0;

    let mut potency = 0;

    reference!(
        "every ingredient (including duplicated ones) count",
        uking::CookingMgr::cookCalcIngredientsBoost()
    );
    for ingr in ingrs {
        reference!(
            "enemy boosts added early",
            uking::CookingMgr::cookCalcIngredientsBoost(),
            tags::CookEnemy
        );
        if ingr.tags.contains(Tag::CookEnemy) {
            max_hp_boost += ingr.boost.max_heart_level;
            time += ingr.boost.effective_time;

            // This is actually never used based on our assertions
            #[cfg(feature = "assertions")]
            {
                stam_boost += ingr.boost.stamina_level;
            }
            
            if effect.uses_time() {
                // every ingredient adds 30s
                time += 30;
            }
        } else {
            hp += ingr.hp;
            debugln!("adding {} hp from {}, now {}", ingr.hp, ingr.actor, hp);
            let is_effect_item = ingr.effect == effect && effect != CookEffect::None;
            if effect.uses_time() {
                // every ingredient adds 30s
                time += 30;
                if is_effect_item {
                    // effect time is added only if effect matches
                    time += base_time;
                }
            }
            reference!(
                "potency only added for that effect",
                uking::CookingMgr::cookCalcIngredientsBoost(),
                cureItemEffectType
            );
            if is_effect_item {
                potency += ingr.effect_level;
            }
        }
    }
    debugln!(
        "Base effect duration is {}*30 + (effect item)*{} + enemy boost = {}",
        ingrs.len(),
        base_time,
        time
    );

    output_const.effect = effect;

    if has_multiple_effect {
        reference!(
            "clears effect if multiple effects",
            uking::CookingMgr::cookCalcIngredientsBoost(),
            effect_found
        );
        output_rng.effect_level = 0.0;
        output_rng.effect_duration = 0;
    } else {
        output_rng.effect_level = if effect.uses_time() {
            output_rng.effect_duration = time;
            let (potency_lv2, potency_lv3) = effect.get_potency_thresholds();
            if potency >= potency_lv3 {
                3.0
            } else if potency >= potency_lv2 {
                2.0
            } else {
                1.0
            }
        } else {
                0.0
            };

        match effect {
            CookEffect::LifeMaxUp => {
                // Hearty, effect_level is
                // number of yellow quarter-heart = potency
                // hp also becomes the value later in AdjustItem
                output_rng.effect_level = (potency + max_hp_boost) as f32;
            }
            CookEffect::GutsRecover => {
                // stamina table, in wheels
                let table = [0.0, 0.2, 0.4, 0.8, 1.0, 1.4, 1.6, 1.8, 2.2, 2.4, 2.8, 3.0];
                let p = potency as usize;
                let wheels = if p >= table.len() {
                    table[table.len() - 1]
                } else {
                    table[p]
                };
                // ultimately, one wheel is 1000. However, it's multiplied by 200
                // in adjustItem, so here, one wheel is 5 just like endura
                output_rng.effect_level = wheels * 5.0;
                #[cfg(feature = "assertions")]
                {
                    assert!(stam_boost == 0, "GutsRecover shoult not have stamina boost")
                }
            }
            CookEffect::ExGutsMaxUp => {
                // endura - one wheel is 5
                output_rng.effect_level = match potency {
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
                    _ => 10,
                } as f32;
                #[cfg(feature = "assertions")]
                {
                    assert!(stam_boost == 0, "ExGutsMaxUp shoult not have stamina boost")
                }
            }
            _ => {}
        }
    }

    if recipe.is_fairy_tonic() && effect != CookEffect::None {
        reference!(
            "fairy tonic special case",
            uking::CookingMgr::cookCalcIngredientsBoost(),
            is_not_fairy_tonic
        );
        output_rng.effect_level = 0.0;
        output_const.effect = CookEffect::None;
        effect = CookEffect::None;
        output_rng.effect_duration = 0;
    }

    let mut dubious = recipe.is_dubious();

    if recipe.is_elixir() && effect == CookEffect::None {
        reference!(
            "elixir with no effect gets turned into dubious",
            uking::CookingMgr::cookCalcIngredientsBoost(),
            is_medicine
        );
        dubious = true;
    }

    if dubious {
        output_rng.health_recover = hp;
        debugln!("result is dubious, hp*1, which is {}", hp);
    } else {
        output_rng.health_recover = hp * HP_MULTIPLIER;
        debugln!("result is not dubious, hp={hp}*2={}", output.health_recover);
    }

    dubious
}

fn calc_crit_boost(
    unique_ingrs: &[&ActorData], 
    output_const: &mut CookDataConstPart, 
    output_rng: CookDataRngPart) -> Discrete<CookDataRngPart>
{
    // crit_chance is not used for monster extract,
    // but we calculate it anyway
    let mut crit_chance = unique_ingrs
        .iter()
        .map(|x| x.boost.success_rate)
        .max()
        .unwrap_or_default();
    // note that game doesn't cap crit_chance
    crit_chance += BASE_CRIT_CHANCES[unique_ingrs.len() - 1];
    output_const.crit_chance = crit_chance;

    for ingr in unique_ingrs {
        if ingr.actor.is_monster_extract() {
            return calc_monster_extract(output_const, output_rng);
        }
    }

    // invoke rng
    let is_crit = distr::uniform(0..100).less_than(crit_chance as u32);
    is_crit.map(|is_crit| {
        if !is_crit {
            return distr::discrete_always(output_rng.clone());
        }

        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        enum Bonus {
            Life,
            Level, // Vitality in decompe code
            Time
        }

        let bonus = if output_const.effect != CookEffect::None {
            let hp = output_rng.health_recover;
            let hp_max = CookEffect::LifeRecover.max_level(); // 120
            let is_hp_maxed = hp >= hp_max as i32;

            let level = output_rng.effect_level;
            let level_max = output_const.effect.max_level();
            let is_level_maxed = level >= level_max as f32;

            match output_const.effect {
                CookEffect::LifeMaxUp => {
                    distr::discrete_always(Bonus::Level)
                }
                CookEffect::GutsRecover | CookEffect::ExGutsMaxUp => {
                    if is_level_maxed {
                        distr::discrete_always(Bonus::Life)
                    } else if is_hp_maxed {
                        distr::discrete_always(Bonus::Level)
                    } else {
                        distr::coin_flip().map(|is_true| {
                            if is_true {
                                distr::always(Bonus::Level)
                            } else {
                                distr::always(Bonus::Life)
                            }
                        })
                    }
                }
                _ => {
                    match (is_level_maxed, is_hp_maxed) {
                        (true, true) => {
                            distr::discrete_always(Bonus::Time)
                        },
                        (true, false) => {
                            distr::coin_flip().map(|is_true| {
                                if is_true {
                                    distr::always(Bonus::Time)
                                } else {
                                    distr::always(Bonus::Life)
                                }
                            })
                        },
                        (false, true) => {
                            distr::coin_flip().map(|is_true| {
                                if is_true {
                                    distr::always(Bonus::Time)
                                } else {
                                    distr::always(Bonus::Level)
                                }
                            })
                        },
                        (false, false) => {
                            distr::uniform(0..3).map(|x| {
                                distr::always(match x {
                                    0 => Bonus::Life,
                                    1 => Bonus::Level,
                                    _ => Bonus::Time
                                })
                            })
                        }
                    }
                }
            }
        } else {
            distr::discrete_always(Bonus::Life)
        };

        bonus.map(|bonus| {
            let mut output = output_rng.clone();
            match bonus {
                Bonus::Level => {
                    #[cfg(feature = "assertions")]
                    {
                        assert!(output_const.effect != CookEffect::None);
                        if !( output_rng.effect_level == 0.0 || output_rng.effect_level >= 1.0) {
                            panic!("Invalid effect level: {}, effect: {:?}", output_rng.effect_level, output_const.effect);
                        }
                    }
                    output.effect_level += output_const.effect.super_success_amount() as f32;
                }
                Bonus::Time => {
                    output.effect_duration += SUPER_SUCCESS_ADD_EFFECT_TIME;
                }
                Bonus::Life => {
                    output.health_recover += CookEffect::LifeRecover.super_success_amount() as i32; // 12
                }
            }
            distr::always(output)
        })
    })
}

fn calc_monster_extract(
    output_const: &mut CookDataConstPart,
    output_rng: CookDataRngPart,
) -> Discrete<CookDataRngPart> {
    let effect_min = if output_rng.health_recover <= 0 || output_const.effect == CookEffect::LifeMaxUp {
        2
    } else {
        0
    };
    let effect_max = if output_const.effect == CookEffect::None {
        2
    } else {
        4
    };
    
    distr::uniform(effect_min..effect_max).map(|x| {
        let mut output = output_rng.clone();
        match x {
            0 => {
                output.health_recover += CookEffect::LifeRecover.super_success_amount() as i32; // 12
            }
            1 => {
                output.health_recover = CookEffect::LifeRecover.min_level() as i32; // 1
            }
            2 => {
                #[cfg(feature = "assertions")]
                {
                    assert!(output_const.effect != CookEffect::None);
                    assert!(output_rng.effect_level == 0.0 || output_rng.effect_level >= 1.0);
                }
                output.effect_level += output_const.effect.super_success_amount() as f32;
            }
            3 => {
                #[cfg(feature = "assertions")]
                {
                    assert!(output_const.effect != CookEffect::None);
                }
                output.effect_level = output_const.effect.min_level() as f32;
            }
            _ => {
                #[cfg(feature = "assertions")]
                {
                    panic!("Invalid monster extract effect")
                }
            }
        }
        distr::uniform(0..3).map(|x| {
            let mut output = output.clone();
            match x {
                0 => {
                    output.effect_duration = 60;
                }
                1 => {
                    output.effect_duration = 600;
                }
                _ => {
                    output.effect_duration = 1800;
                }
            }
            distr::always(output)
        })
    })
}

// // returns crit_rng_hp, see /dump/README.md
// // hearts are not added if guaranteed heart crit,
// // they are added when processing the raw db if crit_chance >= 100 and if crit_rng_hp is false
// fn calc_crit_boost_old(
//     unique_ingrs: &[&ActorData],
//     effect: CookEffect,
//     output: &mut CookData,
// ) -> bool {
//     let mut crit_chance = unique_ingrs
//         .iter()
//         .map(|x| x.boost.success_rate)
//         .max()
//         .unwrap_or_default();
//     // note that game doesn't cap crit_chance
//     crit_chance += BASE_CRIT_CHANCES[unique_ingrs.len() - 1];
//     output.crit_chance = crit_chance;
//
//     if crit_chance == 0 {
//         debugln!("- No crit chance, no rng crit");
//         return false;
//     }
//
//     let has_rng = crit_chance < 100;
//
//     if effect == CookEffect::None {
//         debugln!("- No effect, only heart crit is possible");
//         let hp_crit = (output.health_recover + 12).min(120);
//         if hp_crit == output.health_recover {
//             // hp is already max, so no rng involved
//             debugln!("- HP is already maxed, no rng crit");
//             return false;
//         }
//         if has_rng {
//             debugln!("- Hp is not maxed and crit is not guaranteed, rng crit");
//             return true;
//         }
//         debugln!("- Hp is not maxed and crit is guaranteed, no rng crit");
//         return false;
//     }
//
//     let hp_maxed = output.health_recover >= 120;
//     if hp_maxed {
//         debugln!("- HP is already maxed, no rng crit");
//         // no rng if hp is alredy maxed
//         // fine for hearty, hearty in-game max is 100 with 5 big radish
//         return false;
//     }
//     // only consider cases where hp is not maxed
//     let effect_max = effect.max_level().max(1); // clampMin
//     let effect_maxed = output.effect_level >= effect_max as f32;
//
//     #[allow(clippy::needless_return)]
//     match effect {
//         CookEffect::None => unreachable!(),
//         CookEffect::LifeMaxUp => {
//             debugln!("- Hearty effect, always possible to rng crit");
//             // hearty food, when crit, adds 4
//             // because adjust item is after this, hp also adds 4
//             // looks like it's possible to get 112 max, over the max of 108
//             return true;
//         }
//         CookEffect::ExGutsMaxUp => {
//             // max is 15, but in-game you can only get 10 with 5 endura carrots,
//             // so we always can get effect boost.
//             debugln!("- Endura effect, always possible to rng crit");
//             return true;
//         }
//         CookEffect::GutsRecover => {
//             if effect_maxed {
//                 debugln!("- Stamina effect is maxed, only possible for heart crit");
//                 // if effect is maxed, we can only get heart boost
//                 if has_rng {
//                     debugln!("- crit is not guaranteed, rng crit");
//                     return true;
//                 }
//                 debugln!("- crit is guaranteed, no rng crit");
//                 return false;
//             }
//             debugln!("- It's possible to get stamina boost, rng crit");
//             return true;
//         }
//         _ => {
//             // it's always to time crit, even if time is maxed
//             debugln!("- It's possible to get time boost, rng crit");
//             return true;
//         }
//     }
// }
