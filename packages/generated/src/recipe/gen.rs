//! Automatically generated.
//!
//! DO NOT EDIT. See packages/generated/README.md for more information.

use super::{MultiMatcher, Recipe, RecipeData, SingleMatcher, SingleRecipeData};
use crate::{Actor, CookItem, Tag};

pub(crate) static RECIPES: [RecipeData; 125] = [
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_C_16, -12),
        actors: MultiMatcher::new(&[&[Actor::Animal_Insect_F]]),
        tags: MultiMatcher::new(&[&[Tag::CookOre], &[Tag::CookInsect], &[Tag::CookEnemy]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_C_16, -12),
        actors: MultiMatcher::new(&[&[Actor::Animal_Insect_F]]),
        tags: MultiMatcher::new(&[&[Tag::CookInsect], &[Tag::CookEnemy]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_C_16, -12),
        actors: MultiMatcher::new(&[&[Actor::Animal_Insect_F]]),
        tags: MultiMatcher::new(&[&[Tag::CookOre, Tag::CookEnemy, Tag::CookInsect]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_O_02, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[&[Tag::CookOre]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_C_17, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[&[Tag::CookEnemy], &[Tag::CookInsect]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_O_01, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[&[Tag::CookEnemy, Tag::CookInsect]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_N_02, 4),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Fruit_A, Actor::Item_Fruit_B],
            &[
                Actor::Item_Fruit_B,
                Actor::Item_Fruit_C,
                Actor::Item_Fruit_F,
                Actor::Item_Fruit_H,
                Actor::Item_Fruit_D,
                Actor::Item_Fruit_G,
                Actor::Item_Fruit_A,
            ],
            &[Actor::Item_Material_07],
            &[Actor::Item_Material_01],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_N_01, 8),
        actors: MultiMatcher::new(&[
            &[Actor::Item_FishGet_F, Actor::Item_FishGet_G],
            &[Actor::Item_FishGet_K],
            &[Actor::Item_Material_03],
            &[Actor::Item_Material_06],
            &[Actor::Item_Ore_H],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_L_05, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Material_03], &[Actor::Item_Material_02], &[]]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_L_04, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Material_03], &[Actor::Item_Ore_H], &[]]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_L_03, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Material_07],
            &[Actor::Item_Material_01],
            &[],
            &[Actor::Item_Material_06],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_L_02, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Material_07],
            &[Actor::Item_Material_06],
            &[Actor::Item_Material_05],
            &[],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_L_01, 0),
        actors: MultiMatcher::new(&[&[]]),
        tags: MultiMatcher::new(&[&[Tag::CookMeat], &[Tag::CookFish]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_F_04, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_PlantGet_B, Actor::Item_PlantGet_C],
            &[Actor::Item_Fruit_C],
            &[Actor::Item_Fruit_F],
            &[Actor::Item_Material_05],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_K_04, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_FishGet_K],
            &[Actor::Item_Material_07],
            &[Actor::Item_Material_05],
            &[Actor::Item_Material_06],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_K_03, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Fruit_J],
            &[Actor::Item_Material_07],
            &[Actor::Item_Material_05],
            &[Actor::Item_Material_06],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_K_05, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Meat_11, Actor::Item_Meat_12],
            &[Actor::Item_Material_07],
            &[Actor::Item_Material_05],
            &[Actor::Item_Material_06],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_K_02, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Meat_02, Actor::Item_Meat_07],
            &[Actor::Item_Material_07],
            &[Actor::Item_Material_05],
            &[Actor::Item_Material_06],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_K_01, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Meat_01, Actor::Item_Meat_06],
            &[Actor::Item_Material_07],
            &[Actor::Item_Material_05],
            &[Actor::Item_Material_06],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_J_09, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Meat_11],
            &[Actor::Item_Material_03],
            &[Actor::Item_Material_02],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_J_08, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Meat_12],
            &[Actor::Item_Material_03],
            &[Actor::Item_Material_02],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_J_07, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Meat_02],
            &[Actor::Item_Material_03],
            &[Actor::Item_Material_02],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_J_05, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Meat_07],
            &[Actor::Item_Material_03],
            &[Actor::Item_Material_02],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_J_06, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Meat_01],
            &[Actor::Item_Material_03],
            &[Actor::Item_Material_02],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_J_04, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Meat_06],
            &[Actor::Item_Material_03],
            &[Actor::Item_Material_02],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_J_03, 0),
        actors: MultiMatcher::new(&[
            &[
                Actor::Item_FishGet_K,
                Actor::Item_FishGet_F,
                Actor::Item_FishGet_G,
            ],
            &[Actor::Item_Material_03],
            &[Actor::Item_Material_02],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_J_02, 0),
        actors: MultiMatcher::new(&[
            &[
                Actor::Item_Fruit_J,
                Actor::Item_PlantGet_M,
                Actor::Item_PlantGet_Q,
            ],
            &[Actor::Item_Material_03],
            &[Actor::Item_Material_02],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_I_06, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Fruit_J],
            &[Actor::Item_Material_07],
            &[Actor::Item_Material_01],
            &[Actor::Item_Material_06],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_I_05, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_PlantGet_M, Actor::Item_PlantGet_Q],
            &[Actor::Item_Material_07],
            &[Actor::Item_Material_01],
            &[Actor::Item_Material_06],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_I_11, 16),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Material_07],
            &[Actor::Item_Material_05],
            &[Actor::Item_Material_04],
            &[Actor::Item_Material_01],
            &[Actor::Item_Fruit_B],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_I_17, 4),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Material_07],
            &[Actor::Item_Material_05],
            &[Actor::Item_Material_04],
            &[Actor::Item_Material_01],
            &[Actor::BeeHome],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_I_10, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Material_07],
            &[Actor::Item_Material_05],
            &[Actor::Item_Material_04],
            &[Actor::Item_Material_01],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_I_02, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Fruit_A],
            &[Actor::Item_Material_07],
            &[Actor::Item_Material_06],
            &[Actor::Item_Material_01],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_I_12, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Material_07],
            &[Actor::Item_Material_06],
            &[Actor::Item_Material_01],
            &[Actor::Item_Fruit_L, Actor::Item_Fruit_K],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_I_03, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Material_04],
            &[Actor::Item_Material_07],
            &[Actor::Item_Material_06],
            &[Actor::Item_Material_01],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_I_14, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Material_04],
            &[Actor::Item_Material_05],
            &[Actor::Item_Material_01],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_I_13, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Fruit_H],
            &[Actor::Item_Material_07],
            &[Actor::Item_Material_01],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_I_01, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Material_07],
            &[Actor::Item_Material_06],
            &[Actor::Item_Material_01],
        ]),
        tags: MultiMatcher::new(&[&[Tag::CookFruit]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_I_04, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Material_07],
            &[Actor::Item_Material_06],
            &[Actor::Item_Ore_H],
        ]),
        tags: MultiMatcher::new(&[&[Tag::CookMeat]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_I_15, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Material_07],
            &[Actor::Item_Material_06],
            &[Actor::Item_Ore_H],
        ]),
        tags: MultiMatcher::new(&[&[Tag::CookFish]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_H_03, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_FishGet_I],
            &[Actor::Item_Material_06],
            &[Actor::Item_Material_07],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_G_14, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_FishGet_I],
            &[Actor::Item_Material_03],
            &[Actor::Item_Ore_H],
            &[Actor::Item_Material_06],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_G_17, 0),
        actors: MultiMatcher::new(&[
            &[
                Actor::Item_InsectGet_K,
                Actor::Item_InsectGet_O,
                Actor::Item_InsectGet_Z,
            ],
            &[Actor::Item_Material_03],
            &[Actor::Item_Ore_H],
            &[Actor::Item_Material_06],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_G_13, 0),
        actors: MultiMatcher::new(&[
            &[
                Actor::Item_Fruit_J,
                Actor::Item_PlantGet_M,
                Actor::Item_PlantGet_Q,
            ],
            &[Actor::Item_Material_03],
            &[Actor::Item_Ore_H],
            &[Actor::Item_Material_06],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_G_12, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Material_03],
            &[Actor::Item_Ore_H],
            &[Actor::Item_Material_06],
        ]),
        tags: MultiMatcher::new(&[&[Tag::CookMushroom]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_K_06, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Ore_H], &[Actor::Item_Material_05]]),
        tags: MultiMatcher::new(&[&[Tag::CookMushroom], &[Tag::CookVegetable, Tag::CookPlant]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_F_03, 0),
        actors: MultiMatcher::new(&[
            &[
                Actor::Item_Fruit_J,
                Actor::Item_PlantGet_M,
                Actor::Item_PlantGet_Q,
            ],
            &[Actor::Item_Ore_H],
            &[Actor::Item_Material_05],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_F_01, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Ore_H], &[Actor::Item_Material_05]]),
        tags: MultiMatcher::new(&[&[Tag::CookMeat], &[Tag::CookVegetable, Tag::CookPlant]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_F_02, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Ore_H], &[Actor::Item_Material_05]]),
        tags: MultiMatcher::new(&[&[Tag::CookFish], &[Tag::CookVegetable, Tag::CookPlant]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_K_07, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Ore_H], &[Actor::Item_Material_05]]),
        tags: MultiMatcher::new(&[&[Tag::CookVegetable, Tag::CookPlant]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_K_08, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_PlantGet_M, Actor::Item_PlantGet_Q],
            &[Actor::Item_Material_06],
            &[Actor::Item_Material_05],
            &[Actor::Item_Material_07],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_N_04, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Material_04],
            &[Actor::Item_Ore_H],
            &[Actor::Item_Material_06],
        ]),
        tags: MultiMatcher::new(&[&[Tag::CookMushroom]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_G_16, 0),
        actors: MultiMatcher::new(&[
            &[
                Actor::Item_InsectGet_K,
                Actor::Item_InsectGet_O,
                Actor::Item_InsectGet_Z,
            ],
            &[Actor::Item_Material_04],
            &[Actor::Item_Material_03],
            &[Actor::Item_Ore_H],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_E_03, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Meat_12],
            &[Actor::Item_Material_04],
            &[Actor::Item_Material_03],
            &[Actor::Item_Material_06],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_E_02, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Meat_07],
            &[Actor::Item_Material_04],
            &[Actor::Item_Material_03],
            &[Actor::Item_Material_06],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_E_01, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Meat_06],
            &[Actor::Item_Material_04],
            &[Actor::Item_Material_03],
            &[Actor::Item_Material_06],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_N_03, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Material_04],
            &[Actor::Item_Material_06],
            &[Actor::Item_Ore_H],
        ]),
        tags: MultiMatcher::new(&[&[Tag::CookVegetable, Tag::CookPlant]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_H_02, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_FishGet_F, Actor::Item_FishGet_G],
            &[Actor::Item_Material_06],
            &[Actor::Item_Material_07],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_H_01, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Material_06], &[Actor::Item_Material_07]]),
        tags: MultiMatcher::new(&[&[Tag::CookFish]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_G_10, 0),
        actors: MultiMatcher::new(&[
            &[
                Actor::Item_FishGet_K,
                Actor::Item_FishGet_F,
                Actor::Item_FishGet_G,
            ],
            &[Actor::Item_Material_03],
            &[Actor::Item_Ore_H],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_G_11, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Material_02],
            &[Actor::Item_Material_03],
            &[Actor::Item_Material_06],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_G_09, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Meat_11],
            &[Actor::Item_Material_03],
            &[Actor::Item_Ore_H],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_G_06, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Meat_02],
            &[Actor::Item_Material_03],
            &[Actor::Item_Ore_H],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_G_05, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Meat_01],
            &[Actor::Item_Material_03],
            &[Actor::Item_Ore_H],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_E_04, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Material_03], &[Actor::Item_Material_04]]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_G_15, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Material_03]]),
        tags: MultiMatcher::new(&[&[Tag::CookMeat]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_G_02, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Material_03]]),
        tags: MultiMatcher::new(&[&[Tag::CookFish]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_G_04, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Material_03]]),
        tags: MultiMatcher::new(&[&[Tag::CookMushroom]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_G_03, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Material_03]]),
        tags: MultiMatcher::new(&[&[Tag::CookPlant, Tag::CookVegetable]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_I_07, 4),
        actors: MultiMatcher::new(&[&[Actor::Item_Fruit_A], &[Actor::Item_Material_06]]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_20, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Fruit_J]]),
        tags: MultiMatcher::new(&[&[Tag::CookMeat]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_A_12, 0),
        actors: MultiMatcher::new(&[&[Actor::BeeHome]]),
        tags: MultiMatcher::new(&[&[Tag::CookMeat]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_A_13, 0),
        actors: MultiMatcher::new(&[&[Actor::BeeHome]]),
        tags: MultiMatcher::new(&[&[Tag::CookFish]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_A_11, 0),
        actors: MultiMatcher::new(&[&[Actor::BeeHome]]),
        tags: MultiMatcher::new(&[&[Tag::CookMushroom]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_A_14, 0),
        actors: MultiMatcher::new(&[&[Actor::BeeHome]]),
        tags: MultiMatcher::new(&[&[Tag::CookPlant, Tag::CookVegetable]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_J_01, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Material_03], &[Actor::Item_Material_02]]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_I_08, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Fruit_A], &[Actor::BeeHome]]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_I_09, 0),
        actors: MultiMatcher::new(&[&[Actor::BeeHome]]),
        tags: MultiMatcher::new(&[&[Tag::CookFruit]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_P_05, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Meat_11], &[Actor::Item_Material_02]]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_P_04, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Meat_02], &[Actor::Item_Material_02]]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_P_03, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Meat_01], &[Actor::Item_Material_02]]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_P_01, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Material_02]]),
        tags: MultiMatcher::new(&[&[Tag::CookMushroom]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_P_02, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Material_02]]),
        tags: MultiMatcher::new(&[&[Tag::CookPlant, Tag::CookVegetable]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_D_06, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Meat_11, Actor::Item_Meat_12],
            &[Actor::Item_Ore_H],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_D_05, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Meat_07, Actor::Item_Meat_02],
            &[Actor::Item_Ore_H],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_D_04, 0),
        actors: MultiMatcher::new(&[
            &[Actor::Item_Meat_01, Actor::Item_Meat_06],
            &[Actor::Item_Ore_H],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_D_10, 0),
        actors: MultiMatcher::new(&[
            &[
                Actor::Item_InsectGet_K,
                Actor::Item_InsectGet_O,
                Actor::Item_InsectGet_Z,
            ],
            &[Actor::Item_Material_02],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_D_09, 0),
        actors: MultiMatcher::new(&[
            &[
                Actor::Item_InsectGet_K,
                Actor::Item_InsectGet_O,
                Actor::Item_InsectGet_Z,
            ],
            &[Actor::Item_Ore_H],
        ]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_D_03, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Ore_H]]),
        tags: MultiMatcher::new(&[&[Tag::CookFish]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_M_01, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Material_07], &[Actor::Item_Ore_H]]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_D_02, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Ore_H]]),
        tags: MultiMatcher::new(&[&[Tag::CookVegetable, Tag::CookPlant]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_D_01, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Ore_H]]),
        tags: MultiMatcher::new(&[&[Tag::CookMushroom]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_16, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[
            &[Tag::CookMeat],
            &[Tag::CookMeat],
            &[Tag::CookMeat],
            &[Tag::CookMeat],
        ]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_11, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[
            &[Tag::CookVegetable, Tag::CookPlant],
            &[Tag::CookVegetable, Tag::CookPlant],
            &[Tag::CookVegetable, Tag::CookPlant],
            &[Tag::CookVegetable, Tag::CookPlant],
        ]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_12, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[
            &[Tag::CookFruit],
            &[Tag::CookFruit],
            &[Tag::CookFruit],
            &[Tag::CookFruit],
        ]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_13, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[
            &[Tag::CookMushroom],
            &[Tag::CookMushroom],
            &[Tag::CookMushroom],
            &[Tag::CookMushroom],
        ]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_15, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[
            &[Tag::CookFish],
            &[Tag::CookFish],
            &[Tag::CookFish],
            &[Tag::CookFish],
        ]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_06, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[&[Tag::CookMeat], &[Tag::CookMeat]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_02, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[&[Tag::CookFruit], &[Tag::CookFruit]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_A_01, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[&[Tag::CookMushroom], &[Tag::CookMushroom]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_01, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[
            &[Tag::CookVegetable, Tag::CookPlant],
            &[Tag::CookVegetable, Tag::CookPlant],
        ]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_19, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Meat_11, Actor::Item_Meat_12]]),
        tags: MultiMatcher::new(&[&[Tag::CookFish]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_18, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Meat_02, Actor::Item_Meat_07]]),
        tags: MultiMatcher::new(&[&[Tag::CookFish]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_17, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[&[Tag::CookMeat], &[Tag::CookFish]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_D_07, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Fruit_I]]),
        tags: MultiMatcher::new(&[&[Tag::CookMeat]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_D_08, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Fruit_I]]),
        tags: MultiMatcher::new(&[&[Tag::CookFish]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_23, 0),
        actors: MultiMatcher::new(&[&[
            Actor::Item_FishGet_K,
            Actor::Item_FishGet_M,
            Actor::Item_InsectGet_K,
            Actor::Item_InsectGet_O,
            Actor::Item_InsectGet_Z,
        ]]),
        tags: MultiMatcher::new(&[&[Tag::CookFish]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_05, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[&[Tag::CookFish], &[Tag::CookFish]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_A_05, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[&[Tag::CookPlant, Tag::CookVegetable], &[Tag::CookMeat]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_A_04, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[&[Tag::CookPlant, Tag::CookVegetable], &[Tag::CookFish]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_A_02, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[&[Tag::CookPlant, Tag::CookVegetable], &[Tag::CookMushroom]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_A_03, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[&[Tag::CookPlant, Tag::CookVegetable], &[Tag::CookFruit]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_A_08, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[&[Tag::CookMushroom], &[Tag::CookFish]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_A_09, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[&[Tag::CookMushroom], &[Tag::CookMeat]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_A_07, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[&[Tag::CookMushroom], &[Tag::CookFruit]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_06, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[&[Tag::CookMeat]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_23, 0),
        actors: MultiMatcher::new(&[&[
            Actor::Item_FishGet_K,
            Actor::Item_FishGet_M,
            Actor::Item_InsectGet_K,
            Actor::Item_InsectGet_O,
            Actor::Item_InsectGet_Z,
        ]]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_05, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[&[Tag::CookFish]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_A_10, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Material_04]]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_K_09, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Material_05]]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_A_01, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[&[Tag::CookMushroom]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_01, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[&[Tag::CookPlant]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_02, 0),
        actors: MultiMatcher::new(&[]),
        tags: MultiMatcher::new(&[&[Tag::CookFruit]]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_22, 0),
        actors: MultiMatcher::new(&[&[Actor::Item_Fruit_K], &[Actor::Item_Fruit_L]]),
        tags: MultiMatcher::new(&[]),
    },
    RecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_C_16, -12),
        actors: MultiMatcher::new(&[&[Actor::Animal_Insect_F]]),
        tags: MultiMatcher::new(&[]),
    },
];
pub(crate) static SINGLE_RECIPES: [SingleRecipeData; 13] = [
    SingleRecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_C_16, -12),
        actors: SingleMatcher::new(&[Actor::Animal_Insect_F]),
        tags: SingleMatcher::new(&[]),
    },
    SingleRecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_O_02, 0),
        actors: SingleMatcher::new(&[]),
        tags: SingleMatcher::new(&[Tag::CookOre]),
    },
    SingleRecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_21, 0),
        actors: SingleMatcher::new(&[Actor::Item_Fruit_I]),
        tags: SingleMatcher::new(&[]),
    },
    SingleRecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_A_10, 0),
        actors: SingleMatcher::new(&[Actor::Item_Material_04]),
        tags: SingleMatcher::new(&[]),
    },
    SingleRecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_06, 0),
        actors: SingleMatcher::new(&[]),
        tags: SingleMatcher::new(&[Tag::CookMeat]),
    },
    SingleRecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_23, 0),
        actors: SingleMatcher::new(&[
            Actor::Item_FishGet_K,
            Actor::Item_FishGet_M,
            Actor::Item_InsectGet_K,
            Actor::Item_InsectGet_O,
            Actor::Item_InsectGet_Z,
        ]),
        tags: SingleMatcher::new(&[]),
    },
    SingleRecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_05, 0),
        actors: SingleMatcher::new(&[]),
        tags: SingleMatcher::new(&[Tag::CookFish]),
    },
    SingleRecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_A_01, 0),
        actors: SingleMatcher::new(&[]),
        tags: SingleMatcher::new(&[Tag::CookMushroom]),
    },
    SingleRecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_I_16, -8),
        actors: SingleMatcher::new(&[Actor::BeeHome]),
        tags: SingleMatcher::new(&[]),
    },
    SingleRecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_22, -2),
        actors: SingleMatcher::new(&[Actor::Item_Fruit_K, Actor::Item_Fruit_L]),
        tags: SingleMatcher::new(&[]),
    },
    SingleRecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_K_09, 2),
        actors: SingleMatcher::new(&[Actor::Item_Material_05]),
        tags: SingleMatcher::new(&[]),
    },
    SingleRecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_01, 0),
        actors: SingleMatcher::new(&[]),
        tags: SingleMatcher::new(&[Tag::CookPlant]),
    },
    SingleRecipeData {
        recipe: Recipe::new(CookItem::Item_Cook_B_02, 0),
        actors: SingleMatcher::new(&[]),
        tags: SingleMatcher::new(&[Tag::CookFruit]),
    },
];
pub(crate) static DUBIOUS_RECIPE: &RecipeData = &RECIPES[5];
