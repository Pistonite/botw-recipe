use botw_recipe_data::CookData;
use cooking::{Modifier, Recipe};


pub fn convert_recipe(recipe: &Recipe) -> CookData {
    let health_recover: i32 = if recipe.effect == Modifier::LifeMaxUp {
        recipe.level * 4
    } else {
        recipe.hp as i32
    };
        let price: i32 = recipe.price;

        let (effect_id, effect_duration, effect_level): (
    f32, i32, f32
) = match recipe.effect {
            Modifier::AttackUp => {
            (10.0, recipe.time, recipe.level as f32)
        },
            Modifier::DefenseUp => {
            (11.0, recipe.time, recipe.level as f32)
        },
            Modifier::ResistCold => {
            (5.0, recipe.time, recipe.level as f32)
        },
            Modifier::ResistHot => {
            (4.0, recipe.time, recipe.level as f32)
        },
            Modifier::ResistElectric => {
            (6.0, recipe.time, recipe.level as f32)
        },
            Modifier::Fireproof => {
            ( 16.0, recipe.time, recipe.level as f32
        )},
            Modifier::MovingSpeed => {
            (13.0, recipe.time, recipe.level as f32)
        },
            Modifier::Quietness => {
            (12.0, recipe.time, recipe.level as f32)
        },
            Modifier::LifeMaxUp => {
            (2.0, 0, recipe.level as f32 * 4.0)
        },
            Modifier::GutsRecover => {
            (14.0, 0, (recipe.stamina  as f32) * 1000.0)
        },
            Modifier::ExGutsMaxUp => {
            (15.0, 0 , recipe.stamina_extra*5.0)
        },
            Modifier::LifeRecover => {
            (1.0, recipe.time, recipe.level as f32)
        },
            // _ => (-1.0, recipe.time, recipe.level as f32)
            _ => (-1.0, recipe.time, 0.0)
            // _ => (-1.0, 0, 0.0)
        };

        let crit_chance = recipe.crit_rate as i32;
        CookData {
            health_recover,
            effect_duration,
            sell_price: price,
            effect_id,
            effect_level,
            crit_chance
        }
    }
