use std::collections::HashMap;

use enum_map::EnumMap;
use serde::{Deserialize, Serialize};

use super::{CookEffect, Error, Tag};
use crate::Actor;

/// Ingredient data (associated with every actor)
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Ingredient {
    /// The actor corresponding to the ingredient
    pub actor: Actor,
    /// cookSpiceBoostEffectiveTime
    pub boost_effect_time: i32,
    /// cookSpiceBoostHitPointRecover
    pub boost_hp: i32,
    /// cookSpiceBoostMaxHeartLevel
    pub boost_max_heart: i32,
    /// cookSpiceBoostStaminaLevel
    pub boost_stamina: i32,
    /// cookSpiceBoostSuccessRate
    pub boost_success_rate: i32,
    /// cureItemEffectLevel
    pub effect_level: i32,
    /// cureItemEffectType
    pub effect: CookEffect,
    /// cureItemEffectiveTime
    pub effect_time: i32,
    /// cureItemHitPointRecover
    pub hp: i32,
    /// itemBuyingPrice
    pub buy_price: i32,
    /// itemSellingPrice
    pub sell_price: i32,
    /// Tags in ActorLink that might be useful
    pub tags: Vec<Tag>,
    /// The tag that is important in matching recipes
    pub recipe_tag: Tag,
}

pub type Ingredients = EnumMap<Actor, Ingredient>;

pub fn read_ingredients() -> Result<Ingredients, Error> {
    let data = include_str!("../../../../research/output/actor-data.yaml");
    let data: HashMap<String, IngrData> = serde_yaml_ng::from_str(data)?;
    let mut error = vec![];
    let map = Ingredients::from_fn(|actor| {
        if actor == Actor::None {
            return Ingredient {
                actor,
                ..Default::default()
            };
        }
        let result = match data.get(actor.actor_name()) {
            Some(ingr) => ingr.extend(actor),
            None => Err(Error::ItemNotFound(actor.actor_name().to_string())),
        };
        match result {
            Ok(ingr) => ingr,
            Err(e) => {
                error.push(e);
                Ingredient {
                    actor,
                    ..Default::default()
                }
            }
        }
    });
    if error.is_empty() {
        Ok(map)
    } else {
        Err(Error::ReadIngr(error))
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IngrData {
    cook_spice_boost_effective_time: i32,
    cook_spice_boost_hit_point_recover: i32,
    cook_spice_boost_max_heart_level: i32,
    cook_spice_boost_stamina_level: i32,
    cook_spice_boost_success_rate: i32,
    cure_item_effect_level: i32,
    cure_item_effect_type: CookEffect,
    cure_item_effective_time: i32,
    cure_item_hit_point_recover: i32,
    item_buying_price: i32,
    item_selling_price: i32,
    tags: Vec<Tag>,
}

impl IngrData {
    pub fn extend(&self, actor: Actor) -> Result<Ingredient, Error> {
        let recipe_tags = self
            .tags
            .iter()
            .filter(|x| x.is_probably_useful())
            .collect::<Vec<_>>();
        if recipe_tags.len() > 1 {
            return Err(Error::Data(format!(
                "Actor {:?} has multiple recipe tags: {:?}",
                actor, recipe_tags
            )));
        }
        let recipe_tag = recipe_tags.first().map(|x| **x).unwrap_or_default();

        Ok(Ingredient {
            actor,
            boost_effect_time: self.cook_spice_boost_effective_time,
            boost_hp: self.cook_spice_boost_hit_point_recover,
            boost_max_heart: self.cook_spice_boost_max_heart_level,
            boost_stamina: self.cook_spice_boost_stamina_level,
            boost_success_rate: self.cook_spice_boost_success_rate,
            effect_level: self.cure_item_effect_level,
            effect: self.cure_item_effect_type,
            effect_time: self.cure_item_effective_time,
            hp: self.cure_item_hit_point_recover,
            buy_price: self.item_buying_price,
            sell_price: self.item_selling_price,
            tags: self.tags.clone(),
            recipe_tag,
        })
    }
}
