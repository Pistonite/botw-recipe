use botw_recipe_sys::{CookEffect, CookItem};

use crate::{distr, Discrete};

use super::CookData;

/// Result of cooking
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CookResult {
    /// The cooked item
    pub item: CookItem,
    /// Constant part of the cooking data
    pub const_data: CookDataConstPart,
    /// Cooking data
    pub rng_data: Discrete<CookDataRngPart>,
}

impl CookResult {
    pub fn new_rock_hard() -> Self {
        Self {
            item: CookItem::rock_hard(),
            const_data: CookDataConstPart {
                sell_price: 2,
                effect_id: -1.0,
                crit_chance: 0,
            },
            rng_data: distr::discrete_always(CookDataRngPart {
                health_recover: 1,
                effect_duration: 0,
                effect_level: 0.0,
            }),
        }
    }
    pub fn new_dubious(hp: i32) -> Self {
        Self {
            item: CookItem::dubious_food(),
            const_data: CookDataConstPart {
                sell_price: 2,
                effect_id: -1.0,
                crit_chance: 0,
            },
            rng_data: distr::discrete_always(CookDataRngPart {
                health_recover: hp.max(4),
                effect_duration: 0,
                effect_level: 0.0,
            }),
        }
    }

    /// Special result returned by CookMgr
    /// when no ingredients are provided
    pub fn no_ingredients() -> Self {
        Self {
            item: CookItem::dubious_food(),
            const_data: CookDataConstPart {
                sell_price: 1,
                effect_id: -1.0,
                crit_chance: 0,
            },
            rng_data: distr::discrete_always(CookDataRngPart {
                health_recover: 4,
                effect_duration: 0,
                effect_level: 0.0,
            }),
        }
    }

    /// Check if the result is from an invalid cook input
    pub fn is_from_invalid_input(&self) -> bool {
        // normally it's impossible to get a sell price of 1
        return self.const_data.sell_price < 2;
    }

    /// Get cook data relevant to WMC
    pub fn get_wmc_data(&self) -> WmcCookData {
        let (hp, crit) = self.analyze_hp_crit_values();
        WmcCookData {
            hp,
            price: self.const_data.sell_price,
            crit,
        }
    }

    /// Get the base data and HP crit type
    ///
    /// The base type is the smallest data sorted by:
    /// - HP
    /// - effect level
    /// - effect duration
    pub fn get_base_data(&self) -> (CookData, HpCritRngType) {
        let (_, crit) = self.analyze_hp_crit_values();
        let rng_data = self.rng_data.iter().min();
        #[cfg(feature = "assertions")]
        {
            if rng_data.is_none() {
                panic!("No RNG data found");
            }
        }
        let data = match rng_data {
            None => {
                CookData {
                    health_recover: 0,
                    effect_duration: 0,
                    sell_price: self.const_data.sell_price,
                    effect_id: self.const_data.effect_id,
                    effect_level: -1.0,
                    crit_chance: self.const_data.crit_chance,
                }
            }
            Some(data) => {
                CookData {
                    health_recover: data.health_recover,
                    effect_duration: data.effect_duration,
                    sell_price: self.const_data.sell_price,
                    effect_id: self.const_data.effect_id,
                    effect_level: data.effect_level,
                    crit_chance: self.const_data.crit_chance,
                }
            }
        };

        (data, crit)
    }

    /// Get the HP value and crit type
    /// 
    /// For Monster RNG, return 121 as the HP value, as it's
    /// not possible to backtrack which is the base value
    /// and which is the crit value as this point.
    fn analyze_hp_crit_values(&self) -> (i32, HpCritRngType) {
        // there should be at most 3 possibilities for hp
        let mut hps = [0; 3];
        let mut hps_len = 0;
        for data in self.rng_data.iter() {
            let mut found = false;
            for i in 0..hps_len {
                if hps[i] == data.health_recover {
                    found = true;
                    break;
                }
            }
            if !found {
                #[cfg(feature = "assertions")]
                {
                    if hps_len == 3 {
                        panic!("Too many unique HP values");
                    }
                }
                hps[hps_len] = data.health_recover;
                hps_len += 1;
            }
        }
        match hps_len {
            1 => {
                // it's impossible for monster extract to not have RNG
                // because there are always 3 cases:
                // - stay the same
                // - set to min (1 non-hearty, 4 hearty)
                // - add ssa (12 non-hearty, 4 hearty)
                // if 2 of the cases are the same, then the 3rd one is definitely
                // different
                #[cfg(feature = "assertions")]
                {
                    if self.item.is_monster_food() {
                        panic!("Monster extract should not have 1 unique HP value");
                    }
                }
                (hps[0], HpCritRngType::NoRng)
            }
            2 => {
                // sort
                if hps[0] > hps[1] {
                    hps.swap(0, 1);
                }
                let is_hearty = self.const_data.effect_id == CookEffect::LifeMaxUp.game_repr_f32();
                let regular_diff = if is_hearty {
                    4
                } else {
                    (hps[0] + 12).min(120) - hps[0]
                };
                if hps[1] - hps[0] == regular_diff {
                    (hps[0], HpCritRngType::Regular)
                } else {
                    // special value for monster RNG
                    #[cfg(feature = "assertions")]
                    {
                        if !self.item.is_monster_food() {
                            panic!("monster extract RNG detected for non-monster food");
                        }
                        if is_hearty {
                            if hps[0] != 4 {
                                panic!("monster extract RNG for hearty food should have 4 has min value");
                            }
                        } else {
                            if hps[0] != 1 {
                                panic!("monster extract RNG for non-hearty food should have 1 has min value");
                            }
                        }
                    }
                    (121, HpCritRngType::Monster)
                }
            }
            3 => {
                #[cfg(feature = "assertions")]
                {
                    if !self.item.is_monster_food() {
                        panic!("monster extract RNG detected for non-monster food");
                    }
                }
                (121, HpCritRngType::Monster)
            }
            _ => {
                #[cfg(feature = "assertions")]
                {
                    panic!("0 or more than 3 unique HP values");
                }
                (0, HpCritRngType::NoRng)
            }
        }
    }
}

/// CookData that is not affected by randomness
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CookDataConstPart {
    pub sell_price: i32,
    pub effect_id: f32,
    pub crit_chance: i32,
}

/// CookData that is affected by randomness
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CookDataRngPart {
    pub health_recover: i32,
    pub effect_duration: i32,
    pub effect_level: f32,
}

impl Eq for CookDataRngPart {}

impl Ord for CookDataRngPart {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

/// CookData relevant to WMC
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WmcCookData {
    /// HP value
    /// - For monster extract RNG, this is 121
    /// - For guaranteed heart crit, this is the HP after crit is applied
    /// - For non-guaranteed heart crit, this is the HP before crit is applied
    pub hp: i32,
    pub price: i32,
    pub crit: HpCritRngType,
}

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum HpCritRngType {
    /// There is no randomness in the HP of the output
    NoRng = 0,
    /// Regular (non-monster) crit RNG:
    /// - Non-hearty: where HP += 12
    /// - Hearty: where HP += 4
    Regular = 1,

    /// Monster crit mode:
    /// - Non-hearty: HP += 12 or set to 1
    /// - Hearty: HP += 4 or set to 4
    Monster = 3,
}

impl HpCritRngType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::NoRng),
            1 => Some(Self::Regular),
            3 => Some(Self::Monster),
            _ => None,
        }
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }
}
