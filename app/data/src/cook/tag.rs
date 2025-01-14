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
    pub fn is_used_in_recipe_matching(&self) -> bool {
        matches!(
            self,
            Self::CookOre
                | Self::CookInsect
                | Self::CookEnemy
                | Self::CookMeat
                | Self::CookFish
                | Self::CookFruit
                | Self::CookMushroom
                | Self::CookPlant
                // | Self::CookSpice
        )
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
