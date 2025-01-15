use enumset::EnumSet;

mod gen;

use crate::{Actor, CookEffect, Tag};

/// Actor data extracted from parameters and links
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ActorData {
    /// The actor corresponding to this data
    pub actor: Actor,
    /// The actor's tag for recipe matching, or Tag::None
    /// if doesn't have one
    pub recipe_tag: Tag,

    /// cookSpiceBoost* parameters
    pub boost: Boost,

    /// cureItemEffectType - the effect of the ingredient
    pub effect: CookEffect,
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
    #[cfg(feature = "recipe")]
    pub matchable_recipes: crate::RecipeSet,
}

static EMPTY_ACTOR_DATA: ActorData = ActorData::empty();
impl Default for &ActorData {
    fn default() -> Self {
        &EMPTY_ACTOR_DATA
    }
}

impl ActorData {
    pub const fn empty() -> Self {
        Self {
            actor: Actor::None,
            recipe_tag: Tag::None,
            boost: Boost {
                effective_time: 0,
                hit_point_recover: 0,
                max_heart_level: 0,
                stamina_level: 0,
                success_rate: 0,
            },
            effect: CookEffect::None,
            effect_level: 0,
            effect_time: 0,
            hp: 0,
            buy_price: 0,
            sell_price: 0,
            tags: EnumSet::new(),
            #[cfg(feature = "recipe")]
            matchable_recipes: crate::RecipeSet::new(0,0),
        }
    }
}

/// The cookSpiceBoost parameters
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Boost {
    /// cookSpiceBoostEffectiveTime
    pub effective_time: i32,
    /// cookSpiceBoostHitPointRecover
    pub hit_point_recover: i32,
    /// cookSpiceBoostMaxHeartLevel
    pub max_heart_level: i32,
    /// cookSpiceBoostStaminaLevel
    pub stamina_level: i32,
    /// cookSpiceBoostSuccessRate
    pub success_rate: i32,
}

impl Actor {
    /// Get the data of the actor for cooking purposes
    pub const fn data(&self) -> &'static ActorData {
        &gen::ACTOR_DATA[self.as_u8() as usize]
    }
}
