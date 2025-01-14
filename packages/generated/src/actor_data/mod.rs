use enumset::EnumSet;

mod gen;

use crate::{RecipeSet, Actor, CookEffect, Tag};

/// Actor data extracted from parameters and links
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorData {
    /// The actor corresponding to this data
    pub actor: Actor,
    /// cureItemEffectType - the effect of the ingredient
    pub effect: CookEffect,
    /// The actor's tag for recipe matching, or Tag::None
    /// if doesn't have one
    pub recipe_tag: Tag,

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
    /// cureItemEffectiveTime
    pub effect_time: i32,
    /// cureItemHitPointRecover
    pub hp: i32,

    /// itemBuyingPrice
    pub buy_price: i32,
    /// itemSellingPrice
    pub sell_price: i32,
    /// All tags of the actor
    pub tags: EnumSet<Tag>,

    /// Indices of recipes that possible to be matched with this actor
    pub matchable_recipes: RecipeSet,
}

impl Actor {
    /// Get the data of the actor for cooking purposes
    pub const fn data(&self) -> &'static ActorData {
        &gen::ACTOR_DATA[self.as_u8() as usize]
    }
}
