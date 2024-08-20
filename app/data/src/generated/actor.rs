//! Automatically generated.
//!
//! DO NOT EDIT
//!
//! Run `cd research && python main.py` (or `task research`) to regenerate.

use super::Group;
/// Ingredients (actors)
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
pub enum Actor {
    #[default]
    None,
    /// Hearty Durian
    Item_Fruit_D,
    /// Palm Fruit
    Item_Fruit_G,
    /// Apple
    Item_Fruit_A,
    /// Wildberry
    Item_Fruit_B,
    /// Hydromelon
    Item_Fruit_F,
    /// Spicy Pepper
    Item_Fruit_I,
    /// Voltfruit
    Item_Fruit_C,
    /// Fleet-Lotus Seeds
    Item_Fruit_E,
    /// Mighty Bananas
    Item_Fruit_H,
    /// Big Hearty Truffle
    Item_Mushroom_N,
    /// Hearty Truffle
    Item_Mushroom_F,
    /// Endura Shroom
    Item_Mushroom_O,
    /// Hylian Shroom
    Item_Mushroom_E,
    /// Stamella Shroom
    Item_Mushroom_A,
    /// Chillshroom
    Item_Mushroom_B,
    /// Sunshroom
    Item_Mushroom_C,
    /// Zapshroom
    Item_Mushroom_H,
    /// Rushroom
    Item_MushroomGet_D,
    /// Razorshroom
    Item_Mushroom_L,
    /// Ironshroom
    Item_Mushroom_M,
    /// Silent Shroom
    Item_Mushroom_J,
    /// Big Hearty Radish
    Item_PlantGet_C,
    /// Hearty Radish
    Item_PlantGet_B,
    /// Endura Carrot
    Item_PlantGet_Q,
    /// Hyrule Herb
    Item_PlantGet_A,
    /// Swift Carrot
    Item_PlantGet_M,
    /// Fortified Pumpkin
    Item_Fruit_J,
    /// Cool Safflina
    Item_PlantGet_E,
    /// Warm Safflina
    Item_PlantGet_F,
    /// Electric Safflina
    Item_PlantGet_L,
    /// Swift Violet
    Item_PlantGet_O,
    /// Mighty Thistle
    Item_PlantGet_G,
    /// Armoranth
    Item_PlantGet_H,
    /// Blue Nightshade
    Item_PlantGet_I,
    /// Silent Princess
    Item_PlantGet_J,
    /// Raw Gourmet Meat
    Item_Meat_11,
    /// Raw Whole Bird
    Item_Meat_12,
    /// Raw Prime Meat
    Item_Meat_02,
    /// Raw Bird Thigh
    Item_Meat_07,
    /// Raw Meat
    Item_Meat_01,
    /// Raw Bird Drumstick
    Item_Meat_06,
    /// Courser Bee Honey
    BeeHome,
    /// Hylian Rice
    Item_Material_03,
    /// Bird Egg
    Item_Material_04,
    /// Tabantha Wheat
    Item_Material_07,
    /// Fresh Milk
    Item_Material_05,
    /// Acorn
    Item_Fruit_K,
    /// Chickaloo Tree Nut
    Item_Fruit_L,
    /// Cane Sugar
    Item_Material_01,
    /// Goat Butter
    Item_Material_06,
    /// Goron Spice
    Item_Material_02,
    /// Rock Salt
    Item_Ore_H,
    /// Star Fragment
    Item_Ore_J,
    /// Dinraal's Scale
    Item_Enemy_38,
    /// Dinraal's Claw
    Item_Enemy_39,
    /// Shard of Dinraal's Fang
    Item_Enemy_47,
    /// Shard of Dinraal's Horn
    Item_Enemy_48,
    /// Naydra's Scale
    Item_Enemy_49,
    /// Naydra's Claw
    Item_Enemy_50,
    /// Shard of Naydra's Fang
    Item_Enemy_51,
    /// Shard of Naydra's Horn
    Item_Enemy_52,
    /// Farosh's Scale
    Item_Enemy_53,
    /// Farosh's Claw
    Item_Enemy_54,
    /// Shard of Farosh's Fang
    Item_Enemy_55,
    /// Shard of Farosh's Horn
    Item_Enemy_56,
    /// Hearty Salmon
    Item_FishGet_I,
    /// Hearty Blueshell Snail
    Item_FishGet_K,
    /// Hearty Bass
    Item_FishGet_B,
    /// Hyrule Bass
    Item_FishGet_A,
    /// Staminoka Bass
    Item_FishGet_L,
    /// Chillfin Trout
    Item_FishGet_C,
    /// Sizzlefin Trout
    Item_FishGet_J,
    /// Voltfin Trout
    Item_FishGet_D,
    /// Stealthfin Trout
    Item_FishGet_X,
    /// Mighty Carp
    Item_FishGet_E,
    /// Armored Carp
    Item_FishGet_H,
    /// Sanke Carp
    Item_FishGet_Z,
    /// Mighty Porgy
    Item_FishGet_F,
    /// Armored Porgy
    Item_FishGet_G,
    /// Sneaky River Snail
    Item_FishGet_M,
    /// Razorclaw Crab
    Item_InsectGet_K,
    /// Ironshell Crab
    Item_InsectGet_O,
    /// Bright-Eyed Crab
    Item_InsectGet_Z,
    /// Fairy
    Animal_Insect_F,
    /// Winterwing Butterfly
    Animal_Insect_N,
    /// Summerwing Butterfly
    Animal_Insect_Q,
    /// Thunderwing Butterfly
    Animal_Insect_R,
    /// Smotherwing Butterfly
    Animal_Insect_AB,
    /// Cold Darner
    Animal_Insect_C,
    /// Warm Darner
    Animal_Insect_T,
    /// Electric Darner
    Animal_Insect_I,
    /// Restless Cricket
    Animal_Insect_H,
    /// Bladed Rhino Beetle
    Animal_Insect_G,
    /// Rugged Rhino Beetle
    Animal_Insect_P,
    /// Energetic Rhino Beetle
    Animal_Insect_AA,
    /// Sunset Firefly
    Animal_Insect_E,
    /// Hot-Footed Frog
    Animal_Insect_A,
    /// Tireless Frog
    Animal_Insect_B,
    /// Hightail Lizard
    Animal_Insect_S,
    /// Hearty Lizard
    Animal_Insect_M,
    /// Fireproof Lizard
    Animal_Insect_X,
    /// Flint
    Item_Ore_I,
    /// Amber
    Item_Ore_F,
    /// Opal
    Item_Ore_E,
    /// Luminous Stone
    Item_Ore_G,
    /// Topaz
    Item_Ore_D,
    /// Ruby
    Item_Ore_B,
    /// Sapphire
    Item_Ore_C,
    /// Diamond
    Item_Ore_A,
    /// Bokoblin Horn
    Item_Enemy_00,
    /// Bokoblin Fang
    Item_Enemy_01,
    /// Bokoblin Guts
    Item_Enemy_02,
    /// Moblin Horn
    Item_Enemy_06,
    /// Moblin Fang
    Item_Enemy_07,
    /// Moblin Guts
    Item_Enemy_08,
    /// Lizalfos Horn
    Item_Enemy_03,
    /// Lizalfos Talon
    Item_Enemy_04,
    /// Lizalfos Tail
    Item_Enemy_05,
    /// Icy Lizalfos Tail
    Item_Enemy_42,
    /// Red Lizalfos Tail
    Item_Enemy_41,
    /// Yellow Lizalfos Tail
    Item_Enemy_43,
    /// Lynel Horn
    Item_Enemy_12,
    /// Lynel Hoof
    Item_Enemy_13,
    /// Lynel Guts
    Item_Enemy_14,
    /// Chuchu Jelly
    Item_Enemy_40,
    /// White Chuchu Jelly
    Item_Enemy_17,
    /// Red Chuchu Jelly
    Item_Enemy_15,
    /// Yellow Chuchu Jelly
    Item_Enemy_16,
    /// Keese Wing
    Item_Enemy_18,
    /// Ice Keese Wing
    Item_Enemy_46,
    /// Fire Keese Wing
    Item_Enemy_44,
    /// Electric Keese Wing
    Item_Enemy_45,
    /// Keese Eyeball
    Item_Enemy_19,
    /// Octorok Tentacle
    Item_Enemy_20,
    /// Octorok Eyeball
    Item_Enemy_21,
    /// Octo Balloon
    Item_Enemy_57,
    /// Molduga Fin
    Item_Enemy_24,
    /// Molduga Guts
    Item_Enemy_25,
    /// Hinox Toenail
    Item_Enemy_32,
    /// Hinox Tooth
    Item_Enemy_33,
    /// Hinox Guts
    Item_Enemy_34,
    /// Ancient Screw
    Item_Enemy_27,
    /// Ancient Spring
    Item_Enemy_28,
    /// Ancient Gear
    Item_Enemy_26,
    /// Ancient Shaft
    Item_Enemy_29,
    /// Ancient Core
    Item_Enemy_30,
    /// Giant Ancient Core
    Item_Enemy_31,
    /// Wood
    Obj_FireWoodBundle,
    /// Baked Apple
    Item_Roast_03,
    /// Baked Palm Fruit
    Item_Roast_10,
    /// Roasted Wildberry
    Item_Roast_07,
    /// Roasted Acorn
    Item_Roast_48,
    /// Roasted Tree Nut
    Item_Roast_52,
    /// Roasted Hearty Durian
    Item_Roast_09,
    /// Roasted Hydromelon
    Item_Roast_12,
    /// Charred Pepper
    Item_Roast_13,
    /// Roasted Voltfruit
    Item_Roast_08,
    /// Roasted Lotus Seeds
    Item_Roast_16,
    /// Roasted Mighty Bananas
    Item_Roast_11,
    /// Toasty Hylian Shroom
    Item_Roast_06,
    /// Toasty Stamella Shroom
    Item_Roast_04,
    /// Toasty Endura Shroom
    Item_Roast_53,
    /// Toasted Hearty Truffle
    Item_Roast_05,
    /// Toasted Big Hearty Truffle
    Item_Roast_49,
    /// Toasty Chillshroom
    Item_Roast_31,
    /// Toasty Sunshroom
    Item_Roast_32,
    /// Toasty Zapshroom
    Item_Roast_33,
    /// Toasty Rushroom
    Item_Roast_36,
    /// Toasty Razorshroom
    Item_Roast_37,
    /// Toasty Ironshroom
    Item_Roast_38,
    /// Toasty Silent Shroom
    Item_Roast_39,
    /// Roasted Radish
    Item_Roast_18,
    /// Roasted Big Radish
    Item_Roast_19,
    /// Roasted Swift Carrot
    Item_Roast_24,
    /// Roasted Endura Carrot
    Item_Roast_50,
    /// Baked Fortified Pumpkin
    Item_Roast_15,
    /// Roasted Mighty Thistle
    Item_Roast_27,
    /// Roasted Armoranth
    Item_Roast_28,
    /// Campfire Egg
    Item_Roast_51,
    /// Hard-Boiled Egg
    Item_Boiled_01,
    /// Seared Steak
    Item_Roast_01,
    /// Seared Prime Steak
    Item_Roast_40,
    /// Seared Gourmet Steak
    Item_Roast_45,
    /// Roasted Bird Drumstick
    Item_Roast_02,
    /// Roasted Bird Thigh
    Item_Roast_41,
    /// Roasted Whole Bird
    Item_Roast_46,
    /// Roasted Bass
    Item_RoastFish_01,
    /// Roasted Hearty Bass
    Item_RoastFish_02,
    /// Roasted Hearty Salmon
    Item_RoastFish_04,
    /// Roasted Trout
    Item_RoastFish_03,
    /// Roasted Carp
    Item_RoastFish_07,
    /// Roasted Porgy
    Item_RoastFish_09,
    /// Sneaky River Escargot
    Item_RoastFish_13,
    /// Blueshell Escargot
    Item_RoastFish_11,
    /// Blackened Crab
    Item_RoastFish_15,
    /// Icy Meat
    Item_Chilled_01,
    /// Icy Prime Meat
    Item_Chilled_02,
    /// Icy Gourmet Meat
    Item_Chilled_03,
    /// Frozen Bird Drumstick
    Item_Chilled_04,
    /// Frozen Bird Thigh
    Item_Chilled_05,
    /// Frozen Whole Bird
    Item_Chilled_06,
    /// Frozen Bass
    Item_ChilledFish_01,
    /// Frozen Hearty Bass
    Item_ChilledFish_06,
    /// Frozen Hearty Salmon
    Item_ChilledFish_02,
    /// Frozen Trout
    Item_ChilledFish_03,
    /// Frozen Carp
    Item_ChilledFish_04,
    /// Frozen Porgy
    Item_ChilledFish_05,
    /// Frozen Crab
    Item_ChilledFish_07,
    /// Frozen River Snail
    Item_ChilledFish_08,
    /// Icy Hearty Blueshell Snail
    Item_ChilledFish_09,
    /// Sheikah Slate
    Obj_DRStone_Get,
    /// Dye
    dyecolor_00,
    /// Blue
    dyecolor_01,
    /// Red
    dyecolor_02,
    /// Yellow
    dyecolor_03,
    /// White
    dyecolor_04,
    /// Black
    dyecolor_05,
    /// Purple
    dyecolor_06,
    /// Green
    dyecolor_07,
    /// Light Blue
    dyecolor_08,
    /// Navy
    dyecolor_09,
    /// Orange
    dyecolor_10,
    /// Peach
    dyecolor_11,
    /// Crimson
    dyecolor_12,
    /// Light Yellow
    dyecolor_13,
    /// Brown
    dyecolor_14,
    /// Gray
    dyecolor_15,
    /// Fauna Picture
    Obj_Photo_Animal,
    /// Elite Enemy Picture
    Obj_Photo_BossEnemy,
    /// Enemy Picture
    Obj_Photo_Enemy,
    /// Material Picture
    Obj_Photo_Material,
    /// Other Picture
    Obj_Photo_Other,
    /// Weapon Picture
    Obj_Photo_Weapon,
}
impl Actor {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::None => "<none>",
            Self::Item_Fruit_D => "Hearty Durian",
            Self::Item_Fruit_G => "Palm Fruit",
            Self::Item_Fruit_A => "Apple",
            Self::Item_Fruit_B => "Wildberry",
            Self::Item_Fruit_F => "Hydromelon",
            Self::Item_Fruit_I => "Spicy Pepper",
            Self::Item_Fruit_C => "Voltfruit",
            Self::Item_Fruit_E => "Fleet-Lotus Seeds",
            Self::Item_Fruit_H => "Mighty Bananas",
            Self::Item_Mushroom_N => "Big Hearty Truffle",
            Self::Item_Mushroom_F => "Hearty Truffle",
            Self::Item_Mushroom_O => "Endura Shroom",
            Self::Item_Mushroom_E => "Hylian Shroom",
            Self::Item_Mushroom_A => "Stamella Shroom",
            Self::Item_Mushroom_B => "Chillshroom",
            Self::Item_Mushroom_C => "Sunshroom",
            Self::Item_Mushroom_H => "Zapshroom",
            Self::Item_MushroomGet_D => "Rushroom",
            Self::Item_Mushroom_L => "Razorshroom",
            Self::Item_Mushroom_M => "Ironshroom",
            Self::Item_Mushroom_J => "Silent Shroom",
            Self::Item_PlantGet_C => "Big Hearty Radish",
            Self::Item_PlantGet_B => "Hearty Radish",
            Self::Item_PlantGet_Q => "Endura Carrot",
            Self::Item_PlantGet_A => "Hyrule Herb",
            Self::Item_PlantGet_M => "Swift Carrot",
            Self::Item_Fruit_J => "Fortified Pumpkin",
            Self::Item_PlantGet_E => "Cool Safflina",
            Self::Item_PlantGet_F => "Warm Safflina",
            Self::Item_PlantGet_L => "Electric Safflina",
            Self::Item_PlantGet_O => "Swift Violet",
            Self::Item_PlantGet_G => "Mighty Thistle",
            Self::Item_PlantGet_H => "Armoranth",
            Self::Item_PlantGet_I => "Blue Nightshade",
            Self::Item_PlantGet_J => "Silent Princess",
            Self::Item_Meat_11 => "Raw Gourmet Meat",
            Self::Item_Meat_12 => "Raw Whole Bird",
            Self::Item_Meat_02 => "Raw Prime Meat",
            Self::Item_Meat_07 => "Raw Bird Thigh",
            Self::Item_Meat_01 => "Raw Meat",
            Self::Item_Meat_06 => "Raw Bird Drumstick",
            Self::BeeHome => "Courser Bee Honey",
            Self::Item_Material_03 => "Hylian Rice",
            Self::Item_Material_04 => "Bird Egg",
            Self::Item_Material_07 => "Tabantha Wheat",
            Self::Item_Material_05 => "Fresh Milk",
            Self::Item_Fruit_K => "Acorn",
            Self::Item_Fruit_L => "Chickaloo Tree Nut",
            Self::Item_Material_01 => "Cane Sugar",
            Self::Item_Material_06 => "Goat Butter",
            Self::Item_Material_02 => "Goron Spice",
            Self::Item_Ore_H => "Rock Salt",
            Self::Item_Ore_J => "Star Fragment",
            Self::Item_Enemy_38 => "Dinraal's Scale",
            Self::Item_Enemy_39 => "Dinraal's Claw",
            Self::Item_Enemy_47 => "Shard of Dinraal's Fang",
            Self::Item_Enemy_48 => "Shard of Dinraal's Horn",
            Self::Item_Enemy_49 => "Naydra's Scale",
            Self::Item_Enemy_50 => "Naydra's Claw",
            Self::Item_Enemy_51 => "Shard of Naydra's Fang",
            Self::Item_Enemy_52 => "Shard of Naydra's Horn",
            Self::Item_Enemy_53 => "Farosh's Scale",
            Self::Item_Enemy_54 => "Farosh's Claw",
            Self::Item_Enemy_55 => "Shard of Farosh's Fang",
            Self::Item_Enemy_56 => "Shard of Farosh's Horn",
            Self::Item_FishGet_I => "Hearty Salmon",
            Self::Item_FishGet_K => "Hearty Blueshell Snail",
            Self::Item_FishGet_B => "Hearty Bass",
            Self::Item_FishGet_A => "Hyrule Bass",
            Self::Item_FishGet_L => "Staminoka Bass",
            Self::Item_FishGet_C => "Chillfin Trout",
            Self::Item_FishGet_J => "Sizzlefin Trout",
            Self::Item_FishGet_D => "Voltfin Trout",
            Self::Item_FishGet_X => "Stealthfin Trout",
            Self::Item_FishGet_E => "Mighty Carp",
            Self::Item_FishGet_H => "Armored Carp",
            Self::Item_FishGet_Z => "Sanke Carp",
            Self::Item_FishGet_F => "Mighty Porgy",
            Self::Item_FishGet_G => "Armored Porgy",
            Self::Item_FishGet_M => "Sneaky River Snail",
            Self::Item_InsectGet_K => "Razorclaw Crab",
            Self::Item_InsectGet_O => "Ironshell Crab",
            Self::Item_InsectGet_Z => "Bright-Eyed Crab",
            Self::Animal_Insect_F => "Fairy",
            Self::Animal_Insect_N => "Winterwing Butterfly",
            Self::Animal_Insect_Q => "Summerwing Butterfly",
            Self::Animal_Insect_R => "Thunderwing Butterfly",
            Self::Animal_Insect_AB => "Smotherwing Butterfly",
            Self::Animal_Insect_C => "Cold Darner",
            Self::Animal_Insect_T => "Warm Darner",
            Self::Animal_Insect_I => "Electric Darner",
            Self::Animal_Insect_H => "Restless Cricket",
            Self::Animal_Insect_G => "Bladed Rhino Beetle",
            Self::Animal_Insect_P => "Rugged Rhino Beetle",
            Self::Animal_Insect_AA => "Energetic Rhino Beetle",
            Self::Animal_Insect_E => "Sunset Firefly",
            Self::Animal_Insect_A => "Hot-Footed Frog",
            Self::Animal_Insect_B => "Tireless Frog",
            Self::Animal_Insect_S => "Hightail Lizard",
            Self::Animal_Insect_M => "Hearty Lizard",
            Self::Animal_Insect_X => "Fireproof Lizard",
            Self::Item_Ore_I => "Flint",
            Self::Item_Ore_F => "Amber",
            Self::Item_Ore_E => "Opal",
            Self::Item_Ore_G => "Luminous Stone",
            Self::Item_Ore_D => "Topaz",
            Self::Item_Ore_B => "Ruby",
            Self::Item_Ore_C => "Sapphire",
            Self::Item_Ore_A => "Diamond",
            Self::Item_Enemy_00 => "Bokoblin Horn",
            Self::Item_Enemy_01 => "Bokoblin Fang",
            Self::Item_Enemy_02 => "Bokoblin Guts",
            Self::Item_Enemy_06 => "Moblin Horn",
            Self::Item_Enemy_07 => "Moblin Fang",
            Self::Item_Enemy_08 => "Moblin Guts",
            Self::Item_Enemy_03 => "Lizalfos Horn",
            Self::Item_Enemy_04 => "Lizalfos Talon",
            Self::Item_Enemy_05 => "Lizalfos Tail",
            Self::Item_Enemy_42 => "Icy Lizalfos Tail",
            Self::Item_Enemy_41 => "Red Lizalfos Tail",
            Self::Item_Enemy_43 => "Yellow Lizalfos Tail",
            Self::Item_Enemy_12 => "Lynel Horn",
            Self::Item_Enemy_13 => "Lynel Hoof",
            Self::Item_Enemy_14 => "Lynel Guts",
            Self::Item_Enemy_40 => "Chuchu Jelly",
            Self::Item_Enemy_17 => "White Chuchu Jelly",
            Self::Item_Enemy_15 => "Red Chuchu Jelly",
            Self::Item_Enemy_16 => "Yellow Chuchu Jelly",
            Self::Item_Enemy_18 => "Keese Wing",
            Self::Item_Enemy_46 => "Ice Keese Wing",
            Self::Item_Enemy_44 => "Fire Keese Wing",
            Self::Item_Enemy_45 => "Electric Keese Wing",
            Self::Item_Enemy_19 => "Keese Eyeball",
            Self::Item_Enemy_20 => "Octorok Tentacle",
            Self::Item_Enemy_21 => "Octorok Eyeball",
            Self::Item_Enemy_57 => "Octo Balloon",
            Self::Item_Enemy_24 => "Molduga Fin",
            Self::Item_Enemy_25 => "Molduga Guts",
            Self::Item_Enemy_32 => "Hinox Toenail",
            Self::Item_Enemy_33 => "Hinox Tooth",
            Self::Item_Enemy_34 => "Hinox Guts",
            Self::Item_Enemy_27 => "Ancient Screw",
            Self::Item_Enemy_28 => "Ancient Spring",
            Self::Item_Enemy_26 => "Ancient Gear",
            Self::Item_Enemy_29 => "Ancient Shaft",
            Self::Item_Enemy_30 => "Ancient Core",
            Self::Item_Enemy_31 => "Giant Ancient Core",
            Self::Obj_FireWoodBundle => "Wood",
            Self::Item_Roast_03 => "Baked Apple",
            Self::Item_Roast_10 => "Baked Palm Fruit",
            Self::Item_Roast_07 => "Roasted Wildberry",
            Self::Item_Roast_48 => "Roasted Acorn",
            Self::Item_Roast_52 => "Roasted Tree Nut",
            Self::Item_Roast_09 => "Roasted Hearty Durian",
            Self::Item_Roast_12 => "Roasted Hydromelon",
            Self::Item_Roast_13 => "Charred Pepper",
            Self::Item_Roast_08 => "Roasted Voltfruit",
            Self::Item_Roast_16 => "Roasted Lotus Seeds",
            Self::Item_Roast_11 => "Roasted Mighty Bananas",
            Self::Item_Roast_06 => "Toasty Hylian Shroom",
            Self::Item_Roast_04 => "Toasty Stamella Shroom",
            Self::Item_Roast_53 => "Toasty Endura Shroom",
            Self::Item_Roast_05 => "Toasted Hearty Truffle",
            Self::Item_Roast_49 => "Toasted Big Hearty Truffle",
            Self::Item_Roast_31 => "Toasty Chillshroom",
            Self::Item_Roast_32 => "Toasty Sunshroom",
            Self::Item_Roast_33 => "Toasty Zapshroom",
            Self::Item_Roast_36 => "Toasty Rushroom",
            Self::Item_Roast_37 => "Toasty Razorshroom",
            Self::Item_Roast_38 => "Toasty Ironshroom",
            Self::Item_Roast_39 => "Toasty Silent Shroom",
            Self::Item_Roast_18 => "Roasted Radish",
            Self::Item_Roast_19 => "Roasted Big Radish",
            Self::Item_Roast_24 => "Roasted Swift Carrot",
            Self::Item_Roast_50 => "Roasted Endura Carrot",
            Self::Item_Roast_15 => "Baked Fortified Pumpkin",
            Self::Item_Roast_27 => "Roasted Mighty Thistle",
            Self::Item_Roast_28 => "Roasted Armoranth",
            Self::Item_Roast_51 => "Campfire Egg",
            Self::Item_Boiled_01 => "Hard-Boiled Egg",
            Self::Item_Roast_01 => "Seared Steak",
            Self::Item_Roast_40 => "Seared Prime Steak",
            Self::Item_Roast_45 => "Seared Gourmet Steak",
            Self::Item_Roast_02 => "Roasted Bird Drumstick",
            Self::Item_Roast_41 => "Roasted Bird Thigh",
            Self::Item_Roast_46 => "Roasted Whole Bird",
            Self::Item_RoastFish_01 => "Roasted Bass",
            Self::Item_RoastFish_02 => "Roasted Hearty Bass",
            Self::Item_RoastFish_04 => "Roasted Hearty Salmon",
            Self::Item_RoastFish_03 => "Roasted Trout",
            Self::Item_RoastFish_07 => "Roasted Carp",
            Self::Item_RoastFish_09 => "Roasted Porgy",
            Self::Item_RoastFish_13 => "Sneaky River Escargot",
            Self::Item_RoastFish_11 => "Blueshell Escargot",
            Self::Item_RoastFish_15 => "Blackened Crab",
            Self::Item_Chilled_01 => "Icy Meat",
            Self::Item_Chilled_02 => "Icy Prime Meat",
            Self::Item_Chilled_03 => "Icy Gourmet Meat",
            Self::Item_Chilled_04 => "Frozen Bird Drumstick",
            Self::Item_Chilled_05 => "Frozen Bird Thigh",
            Self::Item_Chilled_06 => "Frozen Whole Bird",
            Self::Item_ChilledFish_01 => "Frozen Bass",
            Self::Item_ChilledFish_06 => "Frozen Hearty Bass",
            Self::Item_ChilledFish_02 => "Frozen Hearty Salmon",
            Self::Item_ChilledFish_03 => "Frozen Trout",
            Self::Item_ChilledFish_04 => "Frozen Carp",
            Self::Item_ChilledFish_05 => "Frozen Porgy",
            Self::Item_ChilledFish_07 => "Frozen Crab",
            Self::Item_ChilledFish_08 => "Frozen River Snail",
            Self::Item_ChilledFish_09 => "Icy Hearty Blueshell Snail",
            Self::Obj_DRStone_Get => "Sheikah Slate",
            Self::dyecolor_00 => "Dye",
            Self::dyecolor_01 => "Blue",
            Self::dyecolor_02 => "Red",
            Self::dyecolor_03 => "Yellow",
            Self::dyecolor_04 => "White",
            Self::dyecolor_05 => "Black",
            Self::dyecolor_06 => "Purple",
            Self::dyecolor_07 => "Green",
            Self::dyecolor_08 => "Light Blue",
            Self::dyecolor_09 => "Navy",
            Self::dyecolor_10 => "Orange",
            Self::dyecolor_11 => "Peach",
            Self::dyecolor_12 => "Crimson",
            Self::dyecolor_13 => "Light Yellow",
            Self::dyecolor_14 => "Brown",
            Self::dyecolor_15 => "Gray",
            Self::Obj_Photo_Animal => "Fauna Picture",
            Self::Obj_Photo_BossEnemy => "Elite Enemy Picture",
            Self::Obj_Photo_Enemy => "Enemy Picture",
            Self::Obj_Photo_Material => "Material Picture",
            Self::Obj_Photo_Other => "Other Picture",
            Self::Obj_Photo_Weapon => "Weapon Picture",
        }
    }
    pub const fn group(&self) -> Group {
        match self {
            Self::None => Group::None,
            Self::Item_Fruit_D => Group::Item_Fruit_D,
            Self::Item_Fruit_G => Group::Item_Fruit_G,
            Self::Item_Fruit_A => Group::Item_Fruit_A,
            Self::Item_Fruit_B => Group::Item_Fruit_B,
            Self::Item_Fruit_F => Group::Item_Fruit_F,
            Self::Item_Fruit_I => Group::Item_Fruit_I,
            Self::Item_Fruit_C => Group::Item_Fruit_C,
            Self::Item_Fruit_E => Group::Item_Fruit_E,
            Self::Item_Fruit_H => Group::Item_Fruit_H,
            Self::Item_Mushroom_N => Group::Item_Mushroom_N,
            Self::Item_Mushroom_F => Group::Item_Mushroom_F,
            Self::Item_Mushroom_O => Group::Item_Mushroom_O,
            Self::Item_Mushroom_E => Group::Item_Mushroom_E,
            Self::Item_Mushroom_A => Group::Item_Mushroom_A,
            Self::Item_Mushroom_B => Group::Item_Mushroom_B,
            Self::Item_Mushroom_C => Group::Item_Mushroom_C,
            Self::Item_Mushroom_H => Group::Item_Mushroom_H,
            Self::Item_MushroomGet_D => Group::Item_MushroomGet_D,
            Self::Item_Mushroom_L => Group::Item_Mushroom_L,
            Self::Item_Mushroom_M => Group::Item_Mushroom_M,
            Self::Item_Mushroom_J => Group::Item_Mushroom_J,
            Self::Item_PlantGet_C => Group::Item_PlantGet_C,
            Self::Item_PlantGet_B => Group::Item_PlantGet_B,
            Self::Item_PlantGet_Q => Group::Item_PlantGet_Q,
            Self::Item_PlantGet_A => Group::Item_PlantGet_A,
            Self::Item_PlantGet_M => Group::Item_PlantGet_M,
            Self::Item_Fruit_J => Group::Item_Fruit_J,
            Self::Item_PlantGet_E => Group::Item_PlantGet_E,
            Self::Item_PlantGet_F => Group::Item_PlantGet_F,
            Self::Item_PlantGet_L => Group::Item_PlantGet_L,
            Self::Item_PlantGet_O => Group::Item_PlantGet_O,
            Self::Item_PlantGet_G => Group::Item_PlantGet_G,
            Self::Item_PlantGet_H => Group::Item_PlantGet_H,
            Self::Item_PlantGet_I => Group::Item_PlantGet_I,
            Self::Item_PlantGet_J => Group::Item_PlantGet_J,
            Self::Item_Meat_11 => Group::Item_Meat_11,
            Self::Item_Meat_12 => Group::Item_Meat_12,
            Self::Item_Meat_02 => Group::Item_Meat_02,
            Self::Item_Meat_07 => Group::Item_Meat_07,
            Self::Item_Meat_01 => Group::Item_Meat_01,
            Self::Item_Meat_06 => Group::Item_Meat_06,
            Self::BeeHome => Group::BeeHome,
            Self::Item_Material_03 => Group::Item_Material_03,
            Self::Item_Material_04 => Group::Item_Material_04,
            Self::Item_Material_07 => Group::Item_Material_07,
            Self::Item_Material_05 => Group::Item_Material_05,
            Self::Item_Fruit_K => Group::Item_Fruit_K,
            Self::Item_Fruit_L => Group::Item_Fruit_L,
            Self::Item_Material_01 => Group::Item_Material_01,
            Self::Item_Material_06 => Group::Item_Material_06,
            Self::Item_Material_02 => Group::Item_Material_02,
            Self::Item_Ore_H => Group::Item_Ore_H,
            Self::Item_Ore_J => Group::Item_Ore_J,
            Self::Item_Enemy_38 => Group::Item_Enemy_Grp_54,
            Self::Item_Enemy_49 => Group::Item_Enemy_Grp_54,
            Self::Item_Enemy_53 => Group::Item_Enemy_Grp_54,
            Self::Item_Enemy_39 => Group::Item_Enemy_Grp_55,
            Self::Item_Enemy_50 => Group::Item_Enemy_Grp_55,
            Self::Item_Enemy_54 => Group::Item_Enemy_Grp_55,
            Self::Item_Enemy_47 => Group::Item_Enemy_Grp_56,
            Self::Item_Enemy_51 => Group::Item_Enemy_Grp_56,
            Self::Item_Enemy_55 => Group::Item_Enemy_Grp_56,
            Self::Item_Enemy_48 => Group::Item_Enemy_Grp_57,
            Self::Item_Enemy_52 => Group::Item_Enemy_Grp_57,
            Self::Item_Enemy_56 => Group::Item_Enemy_Grp_57,
            Self::Item_FishGet_I => Group::Item_FishGet_I,
            Self::Item_FishGet_K => Group::Item_FishGet_K,
            Self::Item_FishGet_B => Group::Item_FishGet_B,
            Self::Item_FishGet_A => Group::Item_FishGet_A,
            Self::Item_FishGet_L => Group::Item_FishGet_L,
            Self::Item_FishGet_C => Group::Item_FishGet_C,
            Self::Item_FishGet_J => Group::Item_FishGet_J,
            Self::Item_FishGet_D => Group::Item_FishGet_D,
            Self::Item_FishGet_X => Group::Item_FishGet_X,
            Self::Item_FishGet_E => Group::Item_FishGet_E,
            Self::Item_FishGet_H => Group::Item_FishGet_H,
            Self::Item_FishGet_Z => Group::Item_FishGet_Z,
            Self::Item_FishGet_F => Group::Item_FishGet_F,
            Self::Item_FishGet_G => Group::Item_FishGet_G,
            Self::Item_FishGet_M => Group::Item_FishGet_M,
            Self::Item_InsectGet_K => Group::Item_InsectGet_K,
            Self::Item_InsectGet_O => Group::Item_InsectGet_O,
            Self::Item_InsectGet_Z => Group::Item_InsectGet_Z,
            Self::Animal_Insect_F => Group::Animal_Insect_F,
            Self::Animal_Insect_N => Group::Animal_Insect_N,
            Self::Animal_Insect_Q => Group::Animal_Insect_Q,
            Self::Animal_Insect_R => Group::Animal_Insect_R,
            Self::Animal_Insect_AB => Group::Animal_Insect_AB,
            Self::Animal_Insect_C => Group::Animal_Insect_C,
            Self::Animal_Insect_T => Group::Animal_Insect_T,
            Self::Animal_Insect_I => Group::Animal_Insect_I,
            Self::Animal_Insect_H => Group::Animal_Insect_H,
            Self::Animal_Insect_G => Group::Animal_Insect_G,
            Self::Animal_Insect_P => Group::Animal_Insect_P,
            Self::Animal_Insect_AA => Group::Animal_Insect_AA,
            Self::Animal_Insect_E => Group::Animal_Insect_E,
            Self::Animal_Insect_A => Group::Animal_Insect_A,
            Self::Animal_Insect_B => Group::Animal_Insect_B,
            Self::Animal_Insect_S => Group::Animal_Insect_S,
            Self::Animal_Insect_M => Group::Animal_Insect_M,
            Self::Animal_Insect_X => Group::Animal_Insect_X,
            Self::Item_Ore_I => Group::Item_Ore_I,
            Self::Item_Ore_F => Group::Item_Ore_F,
            Self::Item_Ore_E => Group::Item_Ore_E,
            Self::Item_Ore_G => Group::Item_Ore_G,
            Self::Item_Ore_D => Group::Item_Ore_D,
            Self::Item_Ore_B => Group::Item_Ore_B,
            Self::Item_Ore_C => Group::Item_Ore_C,
            Self::Item_Ore_A => Group::Item_Ore_A,
            Self::Item_Enemy_00 => Group::Item_Enemy_00,
            Self::Item_Enemy_01 => Group::Item_Enemy_01,
            Self::Item_Enemy_02 => Group::Item_Enemy_Grp_104,
            Self::Item_Enemy_19 => Group::Item_Enemy_Grp_104,
            Self::Item_Enemy_06 => Group::Item_Enemy_Grp_105,
            Self::Item_Enemy_40 => Group::Item_Enemy_Grp_105,
            Self::Item_Enemy_57 => Group::Item_Enemy_Grp_105,
            Self::Item_Enemy_07 => Group::Item_Enemy_07,
            Self::Item_Enemy_08 => Group::Item_Enemy_08,
            Self::Item_Enemy_03 => Group::Item_Enemy_Grp_108,
            Self::Item_Enemy_20 => Group::Item_Enemy_Grp_108,
            Self::Item_Enemy_04 => Group::Item_Enemy_04,
            Self::Item_Enemy_05 => Group::Item_Enemy_05,
            Self::Item_Enemy_42 => Group::Item_Enemy_Grp_111,
            Self::Item_Enemy_41 => Group::Item_Enemy_Grp_111,
            Self::Item_Enemy_43 => Group::Item_Enemy_Grp_111,
            Self::Item_Enemy_12 => Group::Item_Enemy_12,
            Self::Item_Enemy_13 => Group::Item_Enemy_13,
            Self::Item_Enemy_14 => Group::Item_Enemy_14,
            Self::Item_Enemy_17 => Group::Item_Enemy_Grp_115,
            Self::Item_Enemy_15 => Group::Item_Enemy_Grp_115,
            Self::Item_Enemy_16 => Group::Item_Enemy_Grp_115,
            Self::Item_Enemy_18 => Group::Item_Enemy_18,
            Self::Item_Enemy_46 => Group::Item_Enemy_Grp_117,
            Self::Item_Enemy_44 => Group::Item_Enemy_Grp_117,
            Self::Item_Enemy_45 => Group::Item_Enemy_Grp_117,
            Self::Item_Enemy_21 => Group::Item_Enemy_21,
            Self::Item_Enemy_24 => Group::Item_Enemy_24,
            Self::Item_Enemy_25 => Group::Item_Enemy_25,
            Self::Item_Enemy_32 => Group::Item_Enemy_32,
            Self::Item_Enemy_33 => Group::Item_Enemy_33,
            Self::Item_Enemy_34 => Group::Item_Enemy_34,
            Self::Item_Enemy_27 => Group::Item_Enemy_27,
            Self::Item_Enemy_28 => Group::Item_Enemy_28,
            Self::Item_Enemy_26 => Group::Item_Enemy_26,
            Self::Item_Enemy_29 => Group::Item_Enemy_29,
            Self::Item_Enemy_30 => Group::Item_Enemy_30,
            Self::Item_Enemy_31 => Group::Item_Enemy_31,
            Self::Obj_FireWoodBundle => Group::Obj_FireWoodBundle,
            Self::Item_Roast_03 => Group::Item_Roast_03,
            Self::Item_Roast_10 => Group::Item_Roast_10,
            Self::Item_Roast_07 => Group::Item_Roast_07,
            Self::Item_Roast_48 => Group::Item_Roast_Grp_134,
            Self::Item_Roast_52 => Group::Item_Roast_Grp_134,
            Self::Item_Roast_09 => Group::Item_Roast_09,
            Self::Item_Roast_12 => Group::Item_Roast_Grp_136,
            Self::Item_Roast_08 => Group::Item_Roast_Grp_136,
            Self::Item_Roast_11 => Group::Item_Roast_Grp_136,
            Self::Item_Roast_13 => Group::Item_Roast_13,
            Self::Item_Roast_16 => Group::Item_Roast_16,
            Self::Item_Roast_06 => Group::Item_Roast_06,
            Self::Item_Roast_04 => Group::Item_Roast_Grp_140,
            Self::Item_Roast_36 => Group::Item_Roast_Grp_140,
            Self::Item_Roast_37 => Group::Item_Roast_Grp_140,
            Self::Item_Roast_38 => Group::Item_Roast_Grp_140,
            Self::Item_Roast_53 => Group::Item_Roast_53,
            Self::Item_Roast_05 => Group::Item_Roast_05,
            Self::Item_Roast_49 => Group::Item_Roast_49,
            Self::Item_Roast_31 => Group::Item_Roast_Grp_144,
            Self::Item_Roast_32 => Group::Item_Roast_Grp_144,
            Self::Item_Roast_33 => Group::Item_Roast_Grp_144,
            Self::Item_Roast_39 => Group::Item_Roast_39,
            Self::Item_Roast_18 => Group::Item_Roast_18,
            Self::Item_Roast_19 => Group::Item_Roast_19,
            Self::Item_Roast_24 => Group::Item_Roast_24,
            Self::Item_Roast_50 => Group::Item_Roast_50,
            Self::Item_Roast_15 => Group::Item_Roast_15,
            Self::Item_Roast_27 => Group::Item_Roast_Grp_151,
            Self::Item_Roast_28 => Group::Item_Roast_Grp_151,
            Self::Item_Roast_51 => Group::Item_Grp_152,
            Self::Item_Boiled_01 => Group::Item_Grp_152,
            Self::Item_Roast_01 => Group::Item_Roast_Grp_153,
            Self::Item_Roast_02 => Group::Item_Roast_Grp_153,
            Self::Item_Roast_40 => Group::Item_Roast_40,
            Self::Item_Roast_45 => Group::Item_Roast_Grp_155,
            Self::Item_Roast_46 => Group::Item_Roast_Grp_155,
            Self::Item_Roast_41 => Group::Item_Roast_41,
            Self::Item_RoastFish_01 => Group::Item_RoastFish_01,
            Self::Item_RoastFish_02 => Group::Item_RoastFish_02,
            Self::Item_RoastFish_04 => Group::Item_RoastFish_Grp_159,
            Self::Item_RoastFish_11 => Group::Item_RoastFish_Grp_159,
            Self::Item_RoastFish_03 => Group::Item_RoastFish_Grp_160,
            Self::Item_RoastFish_13 => Group::Item_RoastFish_Grp_160,
            Self::Item_RoastFish_07 => Group::Item_RoastFish_Grp_161,
            Self::Item_RoastFish_09 => Group::Item_RoastFish_Grp_161,
            Self::Item_RoastFish_15 => Group::Item_RoastFish_15,
            Self::Item_Chilled_01 => Group::Item_Chilled_Grp_163,
            Self::Item_Chilled_04 => Group::Item_Chilled_Grp_163,
            Self::Item_Chilled_02 => Group::Item_Chilled_Grp_164,
            Self::Item_Chilled_05 => Group::Item_Chilled_Grp_164,
            Self::Item_Chilled_03 => Group::Item_Chilled_Grp_165,
            Self::Item_Chilled_06 => Group::Item_Chilled_Grp_165,
            Self::Item_ChilledFish_01 => Group::Item_ChilledFish_Grp_166,
            Self::Item_ChilledFish_06 => Group::Item_ChilledFish_Grp_166,
            Self::Item_ChilledFish_07 => Group::Item_ChilledFish_Grp_166,
            Self::Item_ChilledFish_02 => Group::Item_ChilledFish_02,
            Self::Item_ChilledFish_03 => Group::Item_ChilledFish_Grp_168,
            Self::Item_ChilledFish_08 => Group::Item_ChilledFish_Grp_168,
            Self::Item_ChilledFish_04 => Group::Item_ChilledFish_Grp_169,
            Self::Item_ChilledFish_05 => Group::Item_ChilledFish_Grp_169,
            Self::Item_ChilledFish_09 => Group::Item_ChilledFish_09,
            Self::Obj_DRStone_Get => Group::Obj_DRStone_Get,
            Self::dyecolor_00 => Group::dyecolor_Grp_172,
            Self::dyecolor_01 => Group::dyecolor_Grp_172,
            Self::dyecolor_02 => Group::dyecolor_Grp_172,
            Self::dyecolor_03 => Group::dyecolor_Grp_172,
            Self::dyecolor_04 => Group::dyecolor_Grp_172,
            Self::dyecolor_05 => Group::dyecolor_Grp_172,
            Self::dyecolor_06 => Group::dyecolor_Grp_172,
            Self::dyecolor_07 => Group::dyecolor_Grp_172,
            Self::dyecolor_08 => Group::dyecolor_Grp_172,
            Self::dyecolor_09 => Group::dyecolor_Grp_172,
            Self::dyecolor_10 => Group::dyecolor_Grp_172,
            Self::dyecolor_11 => Group::dyecolor_Grp_172,
            Self::dyecolor_12 => Group::dyecolor_Grp_172,
            Self::dyecolor_13 => Group::dyecolor_Grp_172,
            Self::dyecolor_14 => Group::dyecolor_Grp_172,
            Self::dyecolor_15 => Group::dyecolor_Grp_172,
            Self::Obj_Photo_Animal => Group::Obj_Photo_Grp_173,
            Self::Obj_Photo_Enemy => Group::Obj_Photo_Grp_173,
            Self::Obj_Photo_Material => Group::Obj_Photo_Grp_173,
            Self::Obj_Photo_Other => Group::Obj_Photo_Grp_173,
            Self::Obj_Photo_Weapon => Group::Obj_Photo_Grp_173,
            Self::Obj_Photo_BossEnemy => Group::Obj_Photo_BossEnemy,
        }
    }
    /// Convert item name to actor with case-insenstive comparison
    ///
    /// `<none>` will return `Some(Actor::None)`, while invalid names will return `None`.
    ///
    pub fn try_from<S: AsRef<str>>(s: S) -> Option<Self> {
        match s.as_ref().to_ascii_lowercase().as_str() {
            "<none>" => Some(Actor::None),
            "hearty durian" => Some(Actor::Item_Fruit_D),
            "palm fruit" => Some(Actor::Item_Fruit_G),
            "apple" => Some(Actor::Item_Fruit_A),
            "wildberry" => Some(Actor::Item_Fruit_B),
            "hydromelon" => Some(Actor::Item_Fruit_F),
            "spicy pepper" => Some(Actor::Item_Fruit_I),
            "voltfruit" => Some(Actor::Item_Fruit_C),
            "fleet-lotus seeds" => Some(Actor::Item_Fruit_E),
            "mighty bananas" => Some(Actor::Item_Fruit_H),
            "big hearty truffle" => Some(Actor::Item_Mushroom_N),
            "hearty truffle" => Some(Actor::Item_Mushroom_F),
            "endura shroom" => Some(Actor::Item_Mushroom_O),
            "hylian shroom" => Some(Actor::Item_Mushroom_E),
            "stamella shroom" => Some(Actor::Item_Mushroom_A),
            "chillshroom" => Some(Actor::Item_Mushroom_B),
            "sunshroom" => Some(Actor::Item_Mushroom_C),
            "zapshroom" => Some(Actor::Item_Mushroom_H),
            "rushroom" => Some(Actor::Item_MushroomGet_D),
            "razorshroom" => Some(Actor::Item_Mushroom_L),
            "ironshroom" => Some(Actor::Item_Mushroom_M),
            "silent shroom" => Some(Actor::Item_Mushroom_J),
            "big hearty radish" => Some(Actor::Item_PlantGet_C),
            "hearty radish" => Some(Actor::Item_PlantGet_B),
            "endura carrot" => Some(Actor::Item_PlantGet_Q),
            "hyrule herb" => Some(Actor::Item_PlantGet_A),
            "swift carrot" => Some(Actor::Item_PlantGet_M),
            "fortified pumpkin" => Some(Actor::Item_Fruit_J),
            "cool safflina" => Some(Actor::Item_PlantGet_E),
            "warm safflina" => Some(Actor::Item_PlantGet_F),
            "electric safflina" => Some(Actor::Item_PlantGet_L),
            "swift violet" => Some(Actor::Item_PlantGet_O),
            "mighty thistle" => Some(Actor::Item_PlantGet_G),
            "armoranth" => Some(Actor::Item_PlantGet_H),
            "blue nightshade" => Some(Actor::Item_PlantGet_I),
            "silent princess" => Some(Actor::Item_PlantGet_J),
            "raw gourmet meat" => Some(Actor::Item_Meat_11),
            "raw whole bird" => Some(Actor::Item_Meat_12),
            "raw prime meat" => Some(Actor::Item_Meat_02),
            "raw bird thigh" => Some(Actor::Item_Meat_07),
            "raw meat" => Some(Actor::Item_Meat_01),
            "raw bird drumstick" => Some(Actor::Item_Meat_06),
            "courser bee honey" => Some(Actor::BeeHome),
            "hylian rice" => Some(Actor::Item_Material_03),
            "bird egg" => Some(Actor::Item_Material_04),
            "tabantha wheat" => Some(Actor::Item_Material_07),
            "fresh milk" => Some(Actor::Item_Material_05),
            "acorn" => Some(Actor::Item_Fruit_K),
            "chickaloo tree nut" => Some(Actor::Item_Fruit_L),
            "cane sugar" => Some(Actor::Item_Material_01),
            "goat butter" => Some(Actor::Item_Material_06),
            "goron spice" => Some(Actor::Item_Material_02),
            "rock salt" => Some(Actor::Item_Ore_H),
            "star fragment" => Some(Actor::Item_Ore_J),
            "dinraal's scale" => Some(Actor::Item_Enemy_38),
            "dinraal's claw" => Some(Actor::Item_Enemy_39),
            "shard of dinraal's fang" => Some(Actor::Item_Enemy_47),
            "shard of dinraal's horn" => Some(Actor::Item_Enemy_48),
            "naydra's scale" => Some(Actor::Item_Enemy_49),
            "naydra's claw" => Some(Actor::Item_Enemy_50),
            "shard of naydra's fang" => Some(Actor::Item_Enemy_51),
            "shard of naydra's horn" => Some(Actor::Item_Enemy_52),
            "farosh's scale" => Some(Actor::Item_Enemy_53),
            "farosh's claw" => Some(Actor::Item_Enemy_54),
            "shard of farosh's fang" => Some(Actor::Item_Enemy_55),
            "shard of farosh's horn" => Some(Actor::Item_Enemy_56),
            "hearty salmon" => Some(Actor::Item_FishGet_I),
            "hearty blueshell snail" => Some(Actor::Item_FishGet_K),
            "hearty bass" => Some(Actor::Item_FishGet_B),
            "hyrule bass" => Some(Actor::Item_FishGet_A),
            "staminoka bass" => Some(Actor::Item_FishGet_L),
            "chillfin trout" => Some(Actor::Item_FishGet_C),
            "sizzlefin trout" => Some(Actor::Item_FishGet_J),
            "voltfin trout" => Some(Actor::Item_FishGet_D),
            "stealthfin trout" => Some(Actor::Item_FishGet_X),
            "mighty carp" => Some(Actor::Item_FishGet_E),
            "armored carp" => Some(Actor::Item_FishGet_H),
            "sanke carp" => Some(Actor::Item_FishGet_Z),
            "mighty porgy" => Some(Actor::Item_FishGet_F),
            "armored porgy" => Some(Actor::Item_FishGet_G),
            "sneaky river snail" => Some(Actor::Item_FishGet_M),
            "razorclaw crab" => Some(Actor::Item_InsectGet_K),
            "ironshell crab" => Some(Actor::Item_InsectGet_O),
            "bright-eyed crab" => Some(Actor::Item_InsectGet_Z),
            "fairy" => Some(Actor::Animal_Insect_F),
            "winterwing butterfly" => Some(Actor::Animal_Insect_N),
            "summerwing butterfly" => Some(Actor::Animal_Insect_Q),
            "thunderwing butterfly" => Some(Actor::Animal_Insect_R),
            "smotherwing butterfly" => Some(Actor::Animal_Insect_AB),
            "cold darner" => Some(Actor::Animal_Insect_C),
            "warm darner" => Some(Actor::Animal_Insect_T),
            "electric darner" => Some(Actor::Animal_Insect_I),
            "restless cricket" => Some(Actor::Animal_Insect_H),
            "bladed rhino beetle" => Some(Actor::Animal_Insect_G),
            "rugged rhino beetle" => Some(Actor::Animal_Insect_P),
            "energetic rhino beetle" => Some(Actor::Animal_Insect_AA),
            "sunset firefly" => Some(Actor::Animal_Insect_E),
            "hot-footed frog" => Some(Actor::Animal_Insect_A),
            "tireless frog" => Some(Actor::Animal_Insect_B),
            "hightail lizard" => Some(Actor::Animal_Insect_S),
            "hearty lizard" => Some(Actor::Animal_Insect_M),
            "fireproof lizard" => Some(Actor::Animal_Insect_X),
            "flint" => Some(Actor::Item_Ore_I),
            "amber" => Some(Actor::Item_Ore_F),
            "opal" => Some(Actor::Item_Ore_E),
            "luminous stone" => Some(Actor::Item_Ore_G),
            "topaz" => Some(Actor::Item_Ore_D),
            "ruby" => Some(Actor::Item_Ore_B),
            "sapphire" => Some(Actor::Item_Ore_C),
            "diamond" => Some(Actor::Item_Ore_A),
            "bokoblin horn" => Some(Actor::Item_Enemy_00),
            "bokoblin fang" => Some(Actor::Item_Enemy_01),
            "bokoblin guts" => Some(Actor::Item_Enemy_02),
            "moblin horn" => Some(Actor::Item_Enemy_06),
            "moblin fang" => Some(Actor::Item_Enemy_07),
            "moblin guts" => Some(Actor::Item_Enemy_08),
            "lizalfos horn" => Some(Actor::Item_Enemy_03),
            "lizalfos talon" => Some(Actor::Item_Enemy_04),
            "lizalfos tail" => Some(Actor::Item_Enemy_05),
            "icy lizalfos tail" => Some(Actor::Item_Enemy_42),
            "red lizalfos tail" => Some(Actor::Item_Enemy_41),
            "yellow lizalfos tail" => Some(Actor::Item_Enemy_43),
            "lynel horn" => Some(Actor::Item_Enemy_12),
            "lynel hoof" => Some(Actor::Item_Enemy_13),
            "lynel guts" => Some(Actor::Item_Enemy_14),
            "chuchu jelly" => Some(Actor::Item_Enemy_40),
            "white chuchu jelly" => Some(Actor::Item_Enemy_17),
            "red chuchu jelly" => Some(Actor::Item_Enemy_15),
            "yellow chuchu jelly" => Some(Actor::Item_Enemy_16),
            "keese wing" => Some(Actor::Item_Enemy_18),
            "ice keese wing" => Some(Actor::Item_Enemy_46),
            "fire keese wing" => Some(Actor::Item_Enemy_44),
            "electric keese wing" => Some(Actor::Item_Enemy_45),
            "keese eyeball" => Some(Actor::Item_Enemy_19),
            "octorok tentacle" => Some(Actor::Item_Enemy_20),
            "octorok eyeball" => Some(Actor::Item_Enemy_21),
            "octo balloon" => Some(Actor::Item_Enemy_57),
            "molduga fin" => Some(Actor::Item_Enemy_24),
            "molduga guts" => Some(Actor::Item_Enemy_25),
            "hinox toenail" => Some(Actor::Item_Enemy_32),
            "hinox tooth" => Some(Actor::Item_Enemy_33),
            "hinox guts" => Some(Actor::Item_Enemy_34),
            "ancient screw" => Some(Actor::Item_Enemy_27),
            "ancient spring" => Some(Actor::Item_Enemy_28),
            "ancient gear" => Some(Actor::Item_Enemy_26),
            "ancient shaft" => Some(Actor::Item_Enemy_29),
            "ancient core" => Some(Actor::Item_Enemy_30),
            "giant ancient core" => Some(Actor::Item_Enemy_31),
            "wood" => Some(Actor::Obj_FireWoodBundle),
            "baked apple" => Some(Actor::Item_Roast_03),
            "baked palm fruit" => Some(Actor::Item_Roast_10),
            "roasted wildberry" => Some(Actor::Item_Roast_07),
            "roasted acorn" => Some(Actor::Item_Roast_48),
            "roasted tree nut" => Some(Actor::Item_Roast_52),
            "roasted hearty durian" => Some(Actor::Item_Roast_09),
            "roasted hydromelon" => Some(Actor::Item_Roast_12),
            "charred pepper" => Some(Actor::Item_Roast_13),
            "roasted voltfruit" => Some(Actor::Item_Roast_08),
            "roasted lotus seeds" => Some(Actor::Item_Roast_16),
            "roasted mighty bananas" => Some(Actor::Item_Roast_11),
            "toasty hylian shroom" => Some(Actor::Item_Roast_06),
            "toasty stamella shroom" => Some(Actor::Item_Roast_04),
            "toasty endura shroom" => Some(Actor::Item_Roast_53),
            "toasted hearty truffle" => Some(Actor::Item_Roast_05),
            "toasted big hearty truffle" => Some(Actor::Item_Roast_49),
            "toasty chillshroom" => Some(Actor::Item_Roast_31),
            "toasty sunshroom" => Some(Actor::Item_Roast_32),
            "toasty zapshroom" => Some(Actor::Item_Roast_33),
            "toasty rushroom" => Some(Actor::Item_Roast_36),
            "toasty razorshroom" => Some(Actor::Item_Roast_37),
            "toasty ironshroom" => Some(Actor::Item_Roast_38),
            "toasty silent shroom" => Some(Actor::Item_Roast_39),
            "roasted radish" => Some(Actor::Item_Roast_18),
            "roasted big radish" => Some(Actor::Item_Roast_19),
            "roasted swift carrot" => Some(Actor::Item_Roast_24),
            "roasted endura carrot" => Some(Actor::Item_Roast_50),
            "baked fortified pumpkin" => Some(Actor::Item_Roast_15),
            "roasted mighty thistle" => Some(Actor::Item_Roast_27),
            "roasted armoranth" => Some(Actor::Item_Roast_28),
            "campfire egg" => Some(Actor::Item_Roast_51),
            "hard-boiled egg" => Some(Actor::Item_Boiled_01),
            "seared steak" => Some(Actor::Item_Roast_01),
            "seared prime steak" => Some(Actor::Item_Roast_40),
            "seared gourmet steak" => Some(Actor::Item_Roast_45),
            "roasted bird drumstick" => Some(Actor::Item_Roast_02),
            "roasted bird thigh" => Some(Actor::Item_Roast_41),
            "roasted whole bird" => Some(Actor::Item_Roast_46),
            "roasted bass" => Some(Actor::Item_RoastFish_01),
            "roasted hearty bass" => Some(Actor::Item_RoastFish_02),
            "roasted hearty salmon" => Some(Actor::Item_RoastFish_04),
            "roasted trout" => Some(Actor::Item_RoastFish_03),
            "roasted carp" => Some(Actor::Item_RoastFish_07),
            "roasted porgy" => Some(Actor::Item_RoastFish_09),
            "sneaky river escargot" => Some(Actor::Item_RoastFish_13),
            "blueshell escargot" => Some(Actor::Item_RoastFish_11),
            "blackened crab" => Some(Actor::Item_RoastFish_15),
            "icy meat" => Some(Actor::Item_Chilled_01),
            "icy prime meat" => Some(Actor::Item_Chilled_02),
            "icy gourmet meat" => Some(Actor::Item_Chilled_03),
            "frozen bird drumstick" => Some(Actor::Item_Chilled_04),
            "frozen bird thigh" => Some(Actor::Item_Chilled_05),
            "frozen whole bird" => Some(Actor::Item_Chilled_06),
            "frozen bass" => Some(Actor::Item_ChilledFish_01),
            "frozen hearty bass" => Some(Actor::Item_ChilledFish_06),
            "frozen hearty salmon" => Some(Actor::Item_ChilledFish_02),
            "frozen trout" => Some(Actor::Item_ChilledFish_03),
            "frozen carp" => Some(Actor::Item_ChilledFish_04),
            "frozen porgy" => Some(Actor::Item_ChilledFish_05),
            "frozen crab" => Some(Actor::Item_ChilledFish_07),
            "frozen river snail" => Some(Actor::Item_ChilledFish_08),
            "icy hearty blueshell snail" => Some(Actor::Item_ChilledFish_09),
            "sheikah slate" => Some(Actor::Obj_DRStone_Get),
            "dye" => Some(Actor::dyecolor_00),
            "blue" => Some(Actor::dyecolor_01),
            "red" => Some(Actor::dyecolor_02),
            "yellow" => Some(Actor::dyecolor_03),
            "white" => Some(Actor::dyecolor_04),
            "black" => Some(Actor::dyecolor_05),
            "purple" => Some(Actor::dyecolor_06),
            "green" => Some(Actor::dyecolor_07),
            "light blue" => Some(Actor::dyecolor_08),
            "navy" => Some(Actor::dyecolor_09),
            "orange" => Some(Actor::dyecolor_10),
            "peach" => Some(Actor::dyecolor_11),
            "crimson" => Some(Actor::dyecolor_12),
            "light yellow" => Some(Actor::dyecolor_13),
            "brown" => Some(Actor::dyecolor_14),
            "gray" => Some(Actor::dyecolor_15),
            "fauna picture" => Some(Actor::Obj_Photo_Animal),
            "elite enemy picture" => Some(Actor::Obj_Photo_BossEnemy),
            "enemy picture" => Some(Actor::Obj_Photo_Enemy),
            "material picture" => Some(Actor::Obj_Photo_Material),
            "other picture" => Some(Actor::Obj_Photo_Other),
            "weapon picture" => Some(Actor::Obj_Photo_Weapon),
            _ => None,
        }
    }
}
impl std::fmt::Debug for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Actor").field(&self.name()).finish()
    }
}