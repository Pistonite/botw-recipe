use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum Tag {
 CookEMedicine,
 CookEnemy,
 CookFailure,
 CookFish,
 CookFruit,
 CookInsect,
 CookLowPrice,
 CookMaterial,
 CookMeat,
 CookMushroom,
 CookOre,
 CookPlant,
 CookSpice,
 CookValue1,
 DrinkItem,
 EnemyMaterial,
 Fairy,
 Material,
 RoastFish,
 RoastFruit,
 RoastItem,
 RoastMeat,
 RoastMushroom,
 RoastPlant,
 RoastVegetable,
 TypeInsect,
    #[default]
    None,
}

impl Tag {
    pub fn is_probably_useful(&self) -> bool {
        match self {
            Self::CookOre |
            Self::CookInsect |
            Self::CookEnemy |
            Self::CookMeat |
            Self::CookFish |
            Self::CookFruit |
            Self::CookMushroom |
            Self::CookPlant |
            Self::CookSpice => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
