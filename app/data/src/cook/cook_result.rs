use serde::{Deserialize, Serialize};

use botw_recipe_generated::CookItem;

use super::CookData;

/// Result of cooking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CookResult {
    /// The cooked item
    pub item: CookItem,
    /// Cooking data
    pub data: CookData,
    /// If HP has a random chance to crit
    pub crit_rng_hp: bool,
}

impl CookResult {
    pub fn new_rock_hard() -> Self {
        Self {
            item: CookItem::Item_Cook_O_02,
            data: CookData {
                health_recover: 1,
                effect_duration: 0,
                sell_price: 2,
                effect_id: -1.0,
                effect_level: 0.0,
                crit_chance: 0,
            },
            crit_rng_hp: false,
        }
    }
    pub fn new_dubious(hp: i32) -> Self {
        Self {
            item: CookItem::Item_Cook_O_01,
            data: CookData {
                health_recover: hp.max(4),
                effect_duration: 0,
                sell_price: 2,
                effect_id: -1.0,
                effect_level: 0.0,
                crit_chance: 0,
            },
            crit_rng_hp: false,
        }
    }
}
