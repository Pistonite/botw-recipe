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

#[cfg(feature = "actor")]
mod group;
#[cfg(feature = "actor")]
pub use group::Group;
#[cfg(feature = "actor")]
mod group_impl;
#[cfg(all(feature = "actor", feature = "multichoose"))]
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

mod recipe_set;
pub use recipe_set::RecipeSet;

/// Number of ingredients the cooking pot accepts
#[macro_export]
macro_rules! num_ingr {
    () => {
        5
    };
}

// TODO: generate these instead of hard code
mod cook_effect;
pub use cook_effect::CookEffect;
