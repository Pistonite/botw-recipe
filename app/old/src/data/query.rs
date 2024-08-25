use bit_set::BitSet;

use super::RecipeData;

/// Query for searching the database
pub struct Query {
    /// Only include recipes with hp>=minhp
    pub minhp: u8,
    /// Only include recipes with hp<=maxhp
    pub maxhp: u8,
    /// if none zero, only include recipes with price & include_modifiers != 0
    pub include_modifiers: u16,
    /// only include recipes with price & exclude_modifiers == 0
    pub exclude_modifiers: u16,
    /// consider heart crit
    pub crit: bool,
    /// only include recipes with bits(material) & exclude_materials == 0
    pub exclude_materials: BitSet,
}

impl Query {
    pub fn clone(&self) -> Self {
        Query {
            minhp: self.minhp,
            maxhp: self.maxhp,
            include_modifiers: self.include_modifiers,
            exclude_modifiers: self.exclude_modifiers,
            crit: self.crit,
            exclude_materials: self.exclude_materials.clone(),
        }
    }
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut result = vec![];
        if self.minhp > self.maxhp {
            result.push(String::from(
                "ERROR: minhp is greater than maxhp. There won't be any result.",
            ));
        }
        if self.include_modifiers as u16 & self.exclude_modifiers as u16 != 0 {
            result.push(String::from(
                "ERROR: Some excluded modifiers are also included. There won't be any result.",
            ));
        }

        if result.is_empty() {
            Ok(())
        } else {
            Err(result)
        }
    }
    pub fn materials_matches(&self, mut materials: BitSet) -> bool {
        // see if input materials have some in common with excluded ones
        materials.intersect_with(&self.exclude_materials);
        // 0 means there are no common, so the recipe can be matched
        materials.len() == 0
    }

    pub fn data_matches(&self, data: &RecipeData, crit_hp: u8) -> bool {
        let mut hp_ok = data.hp >= self.minhp && data.hp <= self.maxhp;
        if !hp_ok && self.crit {
            hp_ok = crit_hp >= self.minhp && crit_hp <= self.maxhp;
        }
        if !hp_ok {
            return false;
        }

        if self.include_modifiers != 0 {
            if data.price & self.include_modifiers != self.include_modifiers {
                return false;
            }
        }

        if data.price & self.exclude_modifiers != 0 {
            return false;
        }

        return true;
    }
}
