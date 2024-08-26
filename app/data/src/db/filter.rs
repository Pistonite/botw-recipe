use serde::{Deserialize, Serialize};

use crate::wmc::WeaponModifierSet;

/// Filter for records
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub min_value: i32,
    pub max_value: i32,
    pub includes_modifier: WeaponModifierSet,
    pub excludes_modifier: WeaponModifierSet,
    pub include_crit_rng_hp: bool,
    pub include_pe_only: bool,
}

impl Filter {
    pub fn all() -> Self {
        Self {
            min_value: 0,
            max_value: i32::MAX,
            includes_modifier: WeaponModifierSet::all(),
            excludes_modifier: WeaponModifierSet::new(),
            include_crit_rng_hp: true,
            include_pe_only: true,
        }
    }
}
