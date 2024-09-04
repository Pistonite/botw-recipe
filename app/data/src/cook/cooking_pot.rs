use super::{
    CookData, CookEffect, CookResult, Error, Ingredient, Ingredients, RecipeData, Recipes, Tag,
};

use crate::recipe::{RecipeId, RecipeInputs};
use crate::{debugln, Actor};

macro_rules! reference {
    ($dummy:ty) => {};
    ($comment:literal, $dummy:ty) => {};
    ($comment:literal, $dummy:ty, $name:ty) => {};
}

pub struct CookingPot {
    recipes: Recipes,
    ingredients: Ingredients,
}

/// Each item's hp is multipled by 2 in the result
const HP_MULTIPLIER: i32 = 2;
/// Price scale for number of ingredients
static PRICE_SCALE: [f32; 6] = [0.0, 1.5, 1.8, 2.1, 2.4, 2.8];
/// Base crit chance for number of unique ingredients
static BASE_CRIT_CHANCES: [i32; 5] = [5, 10, 15, 20, 25];

impl CookingPot {
    pub fn new() -> Result<Self, Error> {
        let recipes = super::read_recipes()?;
        let ingredients = super::read_ingredients()?;

        Ok(Self {
            recipes,
            ingredients,
        })
    }
    pub fn cook_by_name<A: IntoIterator<Item = T>, T: AsRef<str>>(
        &self,
        names: A,
    ) -> Result<CookResult, Error> {
        let mut actors = Vec::with_capacity(5);
        for name in names {
            let name = name.as_ref();
            let actor =
                Actor::try_from(name).ok_or_else(|| Error::ItemNotFound(name.to_string()))?;
            actors.push(actor);
        }
        self.cook(actors)
    }

    pub fn cook_id(&self, id: usize) -> Result<CookResult, Error> {
        let id = RecipeId::new(id).ok_or(Error::InvalidRecipeId(id))?;
        self.cook_inputs(id)
    }

    pub fn cook_inputs<T: Into<RecipeInputs>>(&self, inputs: T) -> Result<CookResult, Error> {
        let inputs = inputs.into();
        let mut ingr = Vec::with_capacity(5);
        for group in inputs.iter() {
            let actor = group.first_actor();
            if actor != Actor::None {
                ingr.push(actor);
            }
        }
        self.cook(ingr)
    }

    pub fn cook<A: IntoIterator<Item = T>, T: Into<Actor>>(
        &self,
        actors: A,
    ) -> Result<CookResult, Error> {
        let ingrs = actors
            .into_iter()
            .filter_map(|x| {
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
        let actors = ingrs.iter().map(|x| x.actor).collect::<Vec<_>>();
        let tags = ingrs.iter().map(|x| x.recipe_tag).collect::<Vec<_>>();
        let mut unique_ingrs: Vec<&Ingredient> = Vec::with_capacity(5);
        debugln!("Debug Ingredients:");
        for ingr in &ingrs {
            if !unique_ingrs.iter().any(|x| x.actor == ingr.actor) {
                unique_ingrs.push(ingr);
                debugln!("  - {:#?}", ingr);
            }
        }

        let recipe = self.recipes.find(&actors, &tags, unique_ingrs.len())?;
        debugln!("Recipe: {:?}", recipe);

        let mut output = CookData::new();
        let (effect, is_dubious) = self.calc_ingredient_boost(&ingrs, recipe, &mut output)?;

        reference!(uking::CookingMgr::isCookFailure());
        if is_dubious || recipe.is_rock_hard() {
            reference!(uking::CookingMgr::cookFail());
            if is_dubious {
                return Ok(CookResult::new_dubious(output.health_recover));
            } else {
                return Ok(CookResult::new_rock_hard());
            }
        }

        // handle sell price
        output.sell_price = {
            if recipe.is_fairy_tonic() {
                2
            } else {
                let mut sell_price = 0;
                let mut buy_price = 0;
                for ingr in &ingrs {
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
                sell_price.max(2).min(buy_price)
            }
        };

        // Spice Boost

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
        for ingr in &unique_ingrs {
            // this check is relevant because CookEnemy usually have boost for elixirs
            // and those are added above
            reference!(
                "only non-CookEnemy CookSpice adds boost",
                uking::CookingMgr::cookCalcSpiceBoost(),
                InfoData::hasTag
            );
            if ingr.tags.contains(&Tag::CookEnemy) || !ingr.tags.contains(&Tag::CookSpice) {
                continue;
            }
            reference!(
                "time boost is always added, even if effect is not timed",
                uking::CookingMgr::cookCalcSpiceBoost(),
                effect_time
            );
            time_boost += ingr.boost_effect_time;
            hp_boost += ingr.boost_hp;
            debugln!(
                "adding {} hp_boost from {}, now {} ",
                ingr.boost_hp,
                ingr.actor,
                hp_boost
            );
        }
        output.health_recover += hp_boost;
        debugln!("hp+=hp_boost, now {}", output.health_recover);
        output.effect_duration += time_boost;

        // Recipe Boost

        reference!(
            "recipe extra hp boost",
            uking::CookingMgr::cookCalcRecipeBoost(),
            life_recover
        );
        output.health_recover += recipe.get_extra_hp();
        debugln!(
            "recipe extra hp is {}, hp is now {}",
            recipe.get_extra_hp(),
            output.health_recover
        );

        // Adjust Item

        reference!(
            "no effect min hp to ensure food does something",
            uking::CookingMgr::cookAdjustItem(),
            life_recover
        );
        if effect == CookEffect::None && output.health_recover == 0 {
            output.health_recover = 1;
            debugln!("hp is 0 and food has no effect, setting hp to 1");
        }
        reference!(
            "max life recover",
            uking::CookingMgr::cookAdjustItem(),
            life_recover_max
        );
        debugln!("hp is {}, and will be capped at 120", output.health_recover);
        output.health_recover = output.health_recover.min(120); // 30 hearts
        reference!(
            "max time",
            uking::CookingMgr::cookAdjustItem(),
            sead::Mathi::clamp
        );
        output.effect_duration = output.effect_duration.min(1800); // 30 minutes

        if effect == CookEffect::LifeMaxUp {
            reference!(
                "hearty effect",
                uking::CookingMgr::cookAdjustItem(),
                life_recover
            );
            output.health_recover = output.effect_level as i32;
            debugln!(
                "hearty effect, hp is set to number of yellow quarter-hearts, which is {}",
                output.health_recover
            );
        }

        // We handle crit at the end, so we can know what the final hp is
        debugln!("Calculating if there is HP crit rng:");
        let crit_rng_hp = Self::calc_crit_boost(&unique_ingrs, effect, &mut output);

        Ok(CookResult {
            item: recipe.item(),
            data: output,
            crit_rng_hp,
        })
    }

    // this function is separate because it's before crit is handled
    // so we want to match the game
    // returns if changed to dubious
    reference!(uking::CookingMgr::cookCalcIngredientsBoost);
    fn calc_ingredient_boost(
        &self,
        ingrs: &[&Ingredient],
        recipe: &RecipeData,
        output: &mut CookData,
    ) -> Result<(CookEffect, bool), Error> {
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

        let base_time = effect.data().base_time;
        debugln!("The base time for effect is: {}", base_time);

        let mut hp = 0;
        let mut time = 0;
        let mut max_hp_boost = 0;
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
            if ingr.tags.contains(&Tag::CookEnemy) {
                max_hp_boost += ingr.boost_max_heart;
                time += ingr.boost_effect_time;
                stam_boost += ingr.boost_stamina;
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

        output.effect_id = effect.game_repr_f32();

        if has_multiple_effect {
            reference!(
                "clears effect if multiple effects",
                uking::CookingMgr::cookCalcIngredientsBoost(),
                effect_found
            );
            output.effect_level = 0.0;
            output.effect_duration = 0;
        } else {
            output.effect_level = if effect.uses_time() {
                output.effect_duration = time;
                if potency >= effect.data().potency_lv3 {
                    3.0
                } else if potency >= effect.data().potency_lv2 {
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
                    // note that it's rounded down to nearest whole heart
                    // hp also becomes the value later in AdjustItem
                    let yellow_hearts = potency / 4;
                    output.effect_level = (yellow_hearts * 4 + max_hp_boost) as f32;
                }
                CookEffect::GutsRecover => {
                    // stamina - one wheel is 1000
                    let table = [0.0, 0.2, 0.4, 0.8, 1.0, 1.4, 1.6, 1.8, 2.2, 2.4, 2.8, 3.0];
                    let p = potency as usize;
                    let wheels = if p >= table.len() {
                        table[table.len() - 1]
                    } else {
                        table[p]
                    };
                    output.effect_level = wheels * 1000.0;
                    if stam_boost > 0 {
                        return Err(Error::Data("GutsRecover has stamina boost".to_string()));
                    }
                    //output.effect_level += stam_boost as f32; // pretty sure this is all 0?
                }
                CookEffect::ExGutsMaxUp => {
                    // endura - one wheel is 5
                    output.effect_level = match potency {
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
                    if stam_boost > 0 {
                        return Err(Error::Data("ExGutsMaxUp has stamina boost".to_string()));
                    }
                    //output.effect_level += stam_boost as f32; // pretty sure this is all 0?
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
            output.effect_level = 0.0;
            output.effect_id = CookEffect::None.game_repr_f32();
            effect = CookEffect::None;
            output.effect_duration = 0;
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
            output.health_recover = hp;
            debugln!("result is dubious, hp*1, which is {}", hp);
        } else {
            output.health_recover = hp * HP_MULTIPLIER;
            debugln!("result is not dubious, hp={hp}*2={}", output.health_recover);
        }

        #[allow(clippy::collapsible_if)]
        if output.effect_id != -1.0 {
            if output.effect_level > effect.data().max_level as f32 {
                // game caps it, but it shouldn't happen
                return Err(Error::Data(format!(
                    "Effect level {} is too high for effect {:?}",
                    output.effect_level, effect
                )));
            }
        }

        Ok((effect, dubious))
    }

    // returns crit_rng_hp, see /dump/README.md
    // hearts are not added if guaranteed heart crit,
    // they are added when processing the raw db if crit_chance >= 100 and if crit_rng_hp is false
    fn calc_crit_boost(
        unique_ingrs: &[&Ingredient],
        effect: CookEffect,
        output: &mut CookData,
    ) -> bool {
        let mut crit_chance = unique_ingrs
            .iter()
            .map(|x| x.boost_success_rate)
            .max()
            .unwrap_or_default();
        // note that game doesn't cap crit_chance
        crit_chance += BASE_CRIT_CHANCES[unique_ingrs.len() - 1];
        output.crit_chance = crit_chance;

        if crit_chance == 0 {
            debugln!("- No crit chance, no rng crit");
            return false;
        }

        let has_rng = crit_chance < 100;

        if effect == CookEffect::None {
            debugln!("- No effect, only heart crit is possible");
            let hp_crit = (output.health_recover + 12).min(120);
            if hp_crit == output.health_recover {
                // hp is already max, so no rng involved
                debugln!("- HP is already maxed, no rng crit");
                return false;
            }
            if has_rng {
                debugln!("- Hp is not maxed and crit is not guaranteed, rng crit");
                return true;
            }
            debugln!("- Hp is not maxed and crit is guaranteed, no rng crit");
            return false;
        }

        let hp_maxed = output.health_recover >= 120;
        if hp_maxed {
            debugln!("- HP is already maxed, no rng crit");
            // no rng if hp is alredy maxed
            // fine for hearty, hearty in-game max is 100 with 5 big radish
            return false;
        }
        // only consider cases where hp is not maxed
        let effect_max = effect.data().max_level.max(1); // clampMin
        let effect_maxed = output.effect_level >= effect_max as f32;

        #[allow(clippy::needless_return)]
        match effect {
            CookEffect::None => unreachable!(),
            CookEffect::LifeMaxUp => {
                debugln!("- Hearty effect, always possible to rng crit");
                // hearty food, when crit, adds 4
                // because adjust item is after this, hp also adds 4
                // looks like it's possible to get 112 max, over the max of 108
                return true;
            }
            CookEffect::ExGutsMaxUp => {
                // max is 15, but in-game you can only get 10 with 5 endura carrots,
                // so we always can get effect boost.
                debugln!("- Endura effect, always possible to rng crit");
                return true;
            }
            CookEffect::GutsRecover => {
                if effect_maxed {
                    debugln!("- Stamina effect is maxed, only possible for heart crit");
                    // if effect is maxed, we can only get heart boost
                    if has_rng {
                        debugln!("- crit is not guaranteed, rng crit");
                        return true;
                    }
                    debugln!("- crit is guaranteed, no rng crit");
                    return false;
                }
                debugln!("- It's possible to get stamina boost, rng crit");
                return true;
            }
            _ => {
                // it's always to time crit, even if time is maxed
                debugln!("- It's possible to get time boost, rng crit");
                return true;
            }
        }
    }
}
