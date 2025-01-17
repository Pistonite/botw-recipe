#[cfg(feature = "cook-item")]
mod cook_item;
#[cfg(feature = "cook-item")]
pub use cook_item::CookItem;
#[cfg(feature = "cook-item")]
mod cook_item_impl;

#[cfg(feature = "actor")]
mod actor;
#[cfg(feature = "actor")]
pub use actor::Actor;
#[cfg(feature = "actor")]
mod actor_impl;
#[cfg(all(feature = "actor", feature = "multichoose"))]
pub use actor_impl::ActorMnr;

#[cfg(feature = "actor-wmc-group")]
mod group;
#[cfg(feature = "actor-wmc-group")]
pub use group::Group;
#[cfg(feature = "actor-wmc-group")]
mod group_impl;
#[cfg(all(feature = "actor-wmc-group", feature = "multichoose"))]
pub use group_impl::GroupMnr;

#[cfg(feature = "multichoose")]
mod multichoose;
#[cfg(feature = "multichoose")]
pub use multichoose::Mnr;

#[cfg(feature = "tag")]
mod tag;
#[cfg(feature = "tag")]
pub use tag::Tag;
#[cfg(feature = "tag")]
mod tag_impl;

#[cfg(feature = "actor-data")]
mod actor_data;
#[cfg(feature = "actor-data")]
pub use actor_data::{ActorData, Boost};

#[cfg(feature = "recipe")]
mod recipe_set;
#[cfg(feature = "recipe")]
pub use recipe_set::RecipeSet;
#[cfg(feature = "recipe")]
mod recipe;
#[cfg(feature = "recipe")]
pub use recipe::{Recipe, find_recipe, IngrVec};

#[cfg(feature = "cook-effect")]
mod cook_effect;
#[cfg(feature = "cook-effect")]
pub use cook_effect::CookEffect;
#[cfg(feature = "cook-effect")]
mod cook_effect_impl;

/// Number of ingredients the cooking pot accepts
#[macro_export]
macro_rules! num_ingr {
    () => {
        5
    };
}

