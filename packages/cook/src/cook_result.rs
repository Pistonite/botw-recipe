use botw_recipe_sys::CookItem;

use super::CookData;

/// Result of cooking
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
            item: CookItem::rock_hard(),
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
            item: CookItem::dubious_food(),
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

    /// Special result returned by CookMgr
    /// when no ingredients are provided
    pub fn no_ingredients() -> Self {
        Self {
            item: CookItem::dubious_food(),
            data: CookData {
                health_recover: 4,
                effect_duration: 0,
                sell_price: 1,
                effect_id: -1.0,
                effect_level: 0.0,
                crit_chance: 0,
            },
            crit_rng_hp: false,
        }
    }

    /// Check if the result is from an invalid cook input
    pub fn is_from_invalid_input(&self) -> bool {
        // normally it's impossible to get a sell price of 1
        return self.data.sell_price < 2
    }
}

pub enum HpCritRngType {
    /// There is no randomness in the HP of the output
    NoRng,
    /// Regular (non-monster) crit RNG, where HP += 12
    /// if not hearty and += 4 if hearty, when crit
    Regular,

    /// Monster crit mode, where hearty does not have crit RNG
    Monster,
}
