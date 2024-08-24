//! Automatically generated.
//!
//! DO NOT EDIT
//!
//! Run `cd research && python main.py` (or `task research`) to regenerate.

use super::Actor;
/// Recipe input groups
#[derive(
    enum_map::Enum,
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[allow(non_camel_case_types)]
#[repr(usize)]
pub enum Group {
    #[default]
    None = 0,
    /// Hearty Durian
    Item_Fruit_D = 1,
    /// Palm Fruit
    Item_Fruit_G = 2,
    /// Apple
    Item_Fruit_A = 3,
    /// Wildberry
    Item_Fruit_B = 4,
    /// Hydromelon
    Item_Fruit_F = 5,
    /// Spicy Pepper
    Item_Fruit_I = 6,
    /// Voltfruit
    Item_Fruit_C = 7,
    /// Fleet-Lotus Seeds
    Item_Fruit_E = 8,
    /// Mighty Bananas
    Item_Fruit_H = 9,
    /// Big Hearty Truffle
    Item_Mushroom_N = 10,
    /// Hearty Truffle
    Item_Mushroom_F = 11,
    /// Endura Shroom
    Item_Mushroom_O = 12,
    /// Hylian Shroom
    Item_Mushroom_E = 13,
    /// Stamella Shroom
    Item_Mushroom_A = 14,
    /// Chillshroom
    Item_Mushroom_B = 15,
    /// Sunshroom
    Item_Mushroom_C = 16,
    /// Zapshroom
    Item_Mushroom_H = 17,
    /// Rushroom
    Item_MushroomGet_D = 18,
    /// Razorshroom
    Item_Mushroom_L = 19,
    /// Ironshroom
    Item_Mushroom_M = 20,
    /// Silent Shroom
    Item_Mushroom_J = 21,
    /// Big Hearty Radish
    Item_PlantGet_C = 22,
    /// Hearty Radish
    Item_PlantGet_B = 23,
    /// Endura Carrot
    Item_PlantGet_Q = 24,
    /// Hyrule Herb
    Item_PlantGet_A = 25,
    /// Swift Carrot
    Item_PlantGet_M = 26,
    /// Fortified Pumpkin
    Item_Fruit_J = 27,
    /// Cool Safflina
    Item_PlantGet_E = 28,
    /// Warm Safflina
    Item_PlantGet_F = 29,
    /// Electric Safflina
    Item_PlantGet_L = 30,
    /// Swift Violet
    Item_PlantGet_O = 31,
    /// Mighty Thistle
    Item_PlantGet_G = 32,
    /// Armoranth
    Item_PlantGet_H = 33,
    /// Blue Nightshade
    Item_PlantGet_I = 34,
    /// Silent Princess
    Item_PlantGet_J = 35,
    /// Raw Gourmet Meat
    Item_Meat_11 = 36,
    /// Raw Whole Bird
    Item_Meat_12 = 37,
    /// Raw Prime Meat
    Item_Meat_02 = 38,
    /// Raw Bird Thigh
    Item_Meat_07 = 39,
    /// Raw Meat
    Item_Meat_01 = 40,
    /// Raw Bird Drumstick
    Item_Meat_06 = 41,
    /// Courser Bee Honey
    BeeHome = 42,
    /// Hylian Rice
    Item_Material_03 = 43,
    /// Bird Egg
    Item_Material_04 = 44,
    /// Tabantha Wheat
    Item_Material_07 = 45,
    /// Fresh Milk
    Item_Material_05 = 46,
    /// Acorn
    Item_Fruit_K = 47,
    /// Chickaloo Tree Nut
    Item_Fruit_L = 48,
    /// Cane Sugar
    Item_Material_01 = 49,
    /// Goat Butter
    Item_Material_06 = 50,
    /// Goron Spice
    Item_Material_02 = 51,
    /// Rock Salt
    Item_Ore_H = 52,
    /// Star Fragment
    Item_Ore_J = 53,
    /// Dinraal's Scale, Naydra's Scale, Farosh's Scale
    Item_Enemy_Grp_54 = 54,
    /// Dinraal's Claw, Naydra's Claw, Farosh's Claw
    Item_Enemy_Grp_55 = 55,
    /// Shard of Dinraal's Fang, Shard of Naydra's Fang, Shard of Farosh's Fang
    Item_Enemy_Grp_56 = 56,
    /// Shard of Dinraal's Horn, Shard of Naydra's Horn, Shard of Farosh's Horn
    Item_Enemy_Grp_57 = 57,
    /// Hearty Salmon
    Item_FishGet_I = 58,
    /// Hearty Blueshell Snail
    Item_FishGet_K = 59,
    /// Hearty Bass
    Item_FishGet_B = 60,
    /// Hyrule Bass
    Item_FishGet_A = 61,
    /// Staminoka Bass
    Item_FishGet_L = 62,
    /// Chillfin Trout
    Item_FishGet_C = 63,
    /// Sizzlefin Trout
    Item_FishGet_J = 64,
    /// Voltfin Trout
    Item_FishGet_D = 65,
    /// Stealthfin Trout
    Item_FishGet_X = 66,
    /// Mighty Carp
    Item_FishGet_E = 67,
    /// Armored Carp
    Item_FishGet_H = 68,
    /// Sanke Carp
    Item_FishGet_Z = 69,
    /// Mighty Porgy
    Item_FishGet_F = 70,
    /// Armored Porgy
    Item_FishGet_G = 71,
    /// Sneaky River Snail
    Item_FishGet_M = 72,
    /// Razorclaw Crab
    Item_InsectGet_K = 73,
    /// Ironshell Crab
    Item_InsectGet_O = 74,
    /// Bright-Eyed Crab
    Item_InsectGet_Z = 75,
    /// Fairy
    Animal_Insect_F = 76,
    /// Winterwing Butterfly
    Animal_Insect_N = 77,
    /// Summerwing Butterfly
    Animal_Insect_Q = 78,
    /// Thunderwing Butterfly
    Animal_Insect_R = 79,
    /// Smotherwing Butterfly
    Animal_Insect_AB = 80,
    /// Cold Darner
    Animal_Insect_C = 81,
    /// Warm Darner
    Animal_Insect_T = 82,
    /// Electric Darner
    Animal_Insect_I = 83,
    /// Restless Cricket
    Animal_Insect_H = 84,
    /// Bladed Rhino Beetle
    Animal_Insect_G = 85,
    /// Rugged Rhino Beetle
    Animal_Insect_P = 86,
    /// Energetic Rhino Beetle
    Animal_Insect_AA = 87,
    /// Sunset Firefly
    Animal_Insect_E = 88,
    /// Hot-Footed Frog
    Animal_Insect_A = 89,
    /// Tireless Frog
    Animal_Insect_B = 90,
    /// Hightail Lizard
    Animal_Insect_S = 91,
    /// Hearty Lizard
    Animal_Insect_M = 92,
    /// Fireproof Lizard
    Animal_Insect_X = 93,
    /// Flint
    Item_Ore_I = 94,
    /// Amber
    Item_Ore_F = 95,
    /// Opal
    Item_Ore_E = 96,
    /// Luminous Stone
    Item_Ore_G = 97,
    /// Topaz
    Item_Ore_D = 98,
    /// Ruby
    Item_Ore_B = 99,
    /// Sapphire
    Item_Ore_C = 100,
    /// Diamond
    Item_Ore_A = 101,
    /// Bokoblin Horn
    Item_Enemy_00 = 102,
    /// Bokoblin Fang
    Item_Enemy_01 = 103,
    /// Bokoblin Guts, Keese Eyeball
    Item_Enemy_Grp_104 = 104,
    /// Moblin Horn, Chuchu Jelly, Octo Balloon
    Item_Enemy_Grp_105 = 105,
    /// Moblin Fang
    Item_Enemy_07 = 106,
    /// Moblin Guts
    Item_Enemy_08 = 107,
    /// Lizalfos Horn, Octorok Tentacle
    Item_Enemy_Grp_108 = 108,
    /// Lizalfos Talon
    Item_Enemy_04 = 109,
    /// Lizalfos Tail
    Item_Enemy_05 = 110,
    /// Icy Lizalfos Tail, Red Lizalfos Tail, Yellow Lizalfos Tail
    Item_Enemy_Grp_111 = 111,
    /// Lynel Horn
    Item_Enemy_12 = 112,
    /// Lynel Hoof
    Item_Enemy_13 = 113,
    /// Lynel Guts
    Item_Enemy_14 = 114,
    /// White Chuchu Jelly, Red Chuchu Jelly, Yellow Chuchu Jelly
    Item_Enemy_Grp_115 = 115,
    /// Keese Wing
    Item_Enemy_18 = 116,
    /// Ice Keese Wing, Fire Keese Wing, Electric Keese Wing
    Item_Enemy_Grp_117 = 117,
    /// Octorok Eyeball
    Item_Enemy_21 = 118,
    /// Molduga Fin
    Item_Enemy_24 = 119,
    /// Molduga Guts
    Item_Enemy_25 = 120,
    /// Hinox Toenail
    Item_Enemy_32 = 121,
    /// Hinox Tooth
    Item_Enemy_33 = 122,
    /// Hinox Guts
    Item_Enemy_34 = 123,
    /// Ancient Screw
    Item_Enemy_27 = 124,
    /// Ancient Spring
    Item_Enemy_28 = 125,
    /// Ancient Gear
    Item_Enemy_26 = 126,
    /// Ancient Shaft
    Item_Enemy_29 = 127,
    /// Ancient Core
    Item_Enemy_30 = 128,
    /// Giant Ancient Core
    Item_Enemy_31 = 129,
    /// Wood
    Obj_FireWoodBundle = 130,
    /// Baked Apple
    Item_Roast_03 = 131,
    /// Baked Palm Fruit
    Item_Roast_10 = 132,
    /// Roasted Wildberry
    Item_Roast_07 = 133,
    /// Roasted Acorn, Roasted Tree Nut
    Item_Roast_Grp_134 = 134,
    /// Roasted Hearty Durian
    Item_Roast_09 = 135,
    /// Roasted Hydromelon, Roasted Voltfruit, Roasted Mighty Bananas
    Item_Roast_Grp_136 = 136,
    /// Charred Pepper
    Item_Roast_13 = 137,
    /// Roasted Lotus Seeds
    Item_Roast_16 = 138,
    /// Toasty Hylian Shroom
    Item_Roast_06 = 139,
    /// Toasty Stamella Shroom, Toasty Rushroom, Toasty Razorshroom, Toasty Ironshroom
    Item_Roast_Grp_140 = 140,
    /// Toasty Endura Shroom
    Item_Roast_53 = 141,
    /// Toasted Hearty Truffle
    Item_Roast_05 = 142,
    /// Toasted Big Hearty Truffle
    Item_Roast_49 = 143,
    /// Toasty Chillshroom, Toasty Sunshroom, Toasty Zapshroom
    Item_Roast_Grp_144 = 144,
    /// Toasty Silent Shroom
    Item_Roast_39 = 145,
    /// Roasted Radish
    Item_Roast_18 = 146,
    /// Roasted Big Radish
    Item_Roast_19 = 147,
    /// Roasted Swift Carrot
    Item_Roast_24 = 148,
    /// Roasted Endura Carrot
    Item_Roast_50 = 149,
    /// Baked Fortified Pumpkin
    Item_Roast_15 = 150,
    /// Roasted Mighty Thistle, Roasted Armoranth
    Item_Roast_Grp_151 = 151,
    /// Campfire Egg, Hard-Boiled Egg
    Item_Grp_152 = 152,
    /// Seared Steak, Roasted Bird Drumstick
    Item_Roast_Grp_153 = 153,
    /// Seared Prime Steak
    Item_Roast_40 = 154,
    /// Seared Gourmet Steak, Roasted Whole Bird
    Item_Roast_Grp_155 = 155,
    /// Roasted Bird Thigh
    Item_Roast_41 = 156,
    /// Roasted Bass
    Item_RoastFish_01 = 157,
    /// Roasted Hearty Bass
    Item_RoastFish_02 = 158,
    /// Roasted Hearty Salmon, Blueshell Escargot
    Item_RoastFish_Grp_159 = 159,
    /// Roasted Trout, Sneaky River Escargot
    Item_RoastFish_Grp_160 = 160,
    /// Roasted Carp, Roasted Porgy
    Item_RoastFish_Grp_161 = 161,
    /// Blackened Crab
    Item_RoastFish_15 = 162,
    /// Icy Meat, Frozen Bird Drumstick
    Item_Chilled_Grp_163 = 163,
    /// Icy Prime Meat, Frozen Bird Thigh
    Item_Chilled_Grp_164 = 164,
    /// Icy Gourmet Meat, Frozen Whole Bird
    Item_Chilled_Grp_165 = 165,
    /// Frozen Bass, Frozen Hearty Bass, Frozen Crab
    Item_ChilledFish_Grp_166 = 166,
    /// Frozen Hearty Salmon
    Item_ChilledFish_02 = 167,
    /// Frozen Trout, Frozen River Snail
    Item_ChilledFish_Grp_168 = 168,
    /// Frozen Carp, Frozen Porgy
    Item_ChilledFish_Grp_169 = 169,
    /// Icy Hearty Blueshell Snail
    Item_ChilledFish_09 = 170,
    /// Sheikah Slate
    Obj_DRStone_Get = 171,
    /// Dye, Blue, Red, Yellow, White, Black, Purple, Green, Light Blue, Navy, Orange, Peach, Crimson, Light Yellow, Brown, Gray
    dyecolor_Grp_172 = 172,
    /// Fauna Picture, Enemy Picture, Material Picture, Other Picture, Weapon Picture
    Obj_Photo_Grp_173 = 173,
    /// Elite Enemy Picture
    Obj_Photo_BossEnemy = 174,
}
impl Group {
    #[inline]
    pub const fn id(&self) -> usize {
        *self as usize
    }
    #[inline]
    pub const fn from_id_unchecked(id: usize) -> Self {
        unsafe { std::mem::transmute(id) }
    }
    /// Get the first actor in the group
    pub const fn first_actor(&self) -> Actor {
        match self {
            Self::None => Actor::None,
            Self::Item_Fruit_D => Actor::Item_Fruit_D,
            Self::Item_Fruit_G => Actor::Item_Fruit_G,
            Self::Item_Fruit_A => Actor::Item_Fruit_A,
            Self::Item_Fruit_B => Actor::Item_Fruit_B,
            Self::Item_Fruit_F => Actor::Item_Fruit_F,
            Self::Item_Fruit_I => Actor::Item_Fruit_I,
            Self::Item_Fruit_C => Actor::Item_Fruit_C,
            Self::Item_Fruit_E => Actor::Item_Fruit_E,
            Self::Item_Fruit_H => Actor::Item_Fruit_H,
            Self::Item_Mushroom_N => Actor::Item_Mushroom_N,
            Self::Item_Mushroom_F => Actor::Item_Mushroom_F,
            Self::Item_Mushroom_O => Actor::Item_Mushroom_O,
            Self::Item_Mushroom_E => Actor::Item_Mushroom_E,
            Self::Item_Mushroom_A => Actor::Item_Mushroom_A,
            Self::Item_Mushroom_B => Actor::Item_Mushroom_B,
            Self::Item_Mushroom_C => Actor::Item_Mushroom_C,
            Self::Item_Mushroom_H => Actor::Item_Mushroom_H,
            Self::Item_MushroomGet_D => Actor::Item_MushroomGet_D,
            Self::Item_Mushroom_L => Actor::Item_Mushroom_L,
            Self::Item_Mushroom_M => Actor::Item_Mushroom_M,
            Self::Item_Mushroom_J => Actor::Item_Mushroom_J,
            Self::Item_PlantGet_C => Actor::Item_PlantGet_C,
            Self::Item_PlantGet_B => Actor::Item_PlantGet_B,
            Self::Item_PlantGet_Q => Actor::Item_PlantGet_Q,
            Self::Item_PlantGet_A => Actor::Item_PlantGet_A,
            Self::Item_PlantGet_M => Actor::Item_PlantGet_M,
            Self::Item_Fruit_J => Actor::Item_Fruit_J,
            Self::Item_PlantGet_E => Actor::Item_PlantGet_E,
            Self::Item_PlantGet_F => Actor::Item_PlantGet_F,
            Self::Item_PlantGet_L => Actor::Item_PlantGet_L,
            Self::Item_PlantGet_O => Actor::Item_PlantGet_O,
            Self::Item_PlantGet_G => Actor::Item_PlantGet_G,
            Self::Item_PlantGet_H => Actor::Item_PlantGet_H,
            Self::Item_PlantGet_I => Actor::Item_PlantGet_I,
            Self::Item_PlantGet_J => Actor::Item_PlantGet_J,
            Self::Item_Meat_11 => Actor::Item_Meat_11,
            Self::Item_Meat_12 => Actor::Item_Meat_12,
            Self::Item_Meat_02 => Actor::Item_Meat_02,
            Self::Item_Meat_07 => Actor::Item_Meat_07,
            Self::Item_Meat_01 => Actor::Item_Meat_01,
            Self::Item_Meat_06 => Actor::Item_Meat_06,
            Self::BeeHome => Actor::BeeHome,
            Self::Item_Material_03 => Actor::Item_Material_03,
            Self::Item_Material_04 => Actor::Item_Material_04,
            Self::Item_Material_07 => Actor::Item_Material_07,
            Self::Item_Material_05 => Actor::Item_Material_05,
            Self::Item_Fruit_K => Actor::Item_Fruit_K,
            Self::Item_Fruit_L => Actor::Item_Fruit_L,
            Self::Item_Material_01 => Actor::Item_Material_01,
            Self::Item_Material_06 => Actor::Item_Material_06,
            Self::Item_Material_02 => Actor::Item_Material_02,
            Self::Item_Ore_H => Actor::Item_Ore_H,
            Self::Item_Ore_J => Actor::Item_Ore_J,
            Self::Item_Enemy_Grp_54 => Actor::Item_Enemy_38,
            Self::Item_Enemy_Grp_55 => Actor::Item_Enemy_39,
            Self::Item_Enemy_Grp_56 => Actor::Item_Enemy_47,
            Self::Item_Enemy_Grp_57 => Actor::Item_Enemy_48,
            Self::Item_FishGet_I => Actor::Item_FishGet_I,
            Self::Item_FishGet_K => Actor::Item_FishGet_K,
            Self::Item_FishGet_B => Actor::Item_FishGet_B,
            Self::Item_FishGet_A => Actor::Item_FishGet_A,
            Self::Item_FishGet_L => Actor::Item_FishGet_L,
            Self::Item_FishGet_C => Actor::Item_FishGet_C,
            Self::Item_FishGet_J => Actor::Item_FishGet_J,
            Self::Item_FishGet_D => Actor::Item_FishGet_D,
            Self::Item_FishGet_X => Actor::Item_FishGet_X,
            Self::Item_FishGet_E => Actor::Item_FishGet_E,
            Self::Item_FishGet_H => Actor::Item_FishGet_H,
            Self::Item_FishGet_Z => Actor::Item_FishGet_Z,
            Self::Item_FishGet_F => Actor::Item_FishGet_F,
            Self::Item_FishGet_G => Actor::Item_FishGet_G,
            Self::Item_FishGet_M => Actor::Item_FishGet_M,
            Self::Item_InsectGet_K => Actor::Item_InsectGet_K,
            Self::Item_InsectGet_O => Actor::Item_InsectGet_O,
            Self::Item_InsectGet_Z => Actor::Item_InsectGet_Z,
            Self::Animal_Insect_F => Actor::Animal_Insect_F,
            Self::Animal_Insect_N => Actor::Animal_Insect_N,
            Self::Animal_Insect_Q => Actor::Animal_Insect_Q,
            Self::Animal_Insect_R => Actor::Animal_Insect_R,
            Self::Animal_Insect_AB => Actor::Animal_Insect_AB,
            Self::Animal_Insect_C => Actor::Animal_Insect_C,
            Self::Animal_Insect_T => Actor::Animal_Insect_T,
            Self::Animal_Insect_I => Actor::Animal_Insect_I,
            Self::Animal_Insect_H => Actor::Animal_Insect_H,
            Self::Animal_Insect_G => Actor::Animal_Insect_G,
            Self::Animal_Insect_P => Actor::Animal_Insect_P,
            Self::Animal_Insect_AA => Actor::Animal_Insect_AA,
            Self::Animal_Insect_E => Actor::Animal_Insect_E,
            Self::Animal_Insect_A => Actor::Animal_Insect_A,
            Self::Animal_Insect_B => Actor::Animal_Insect_B,
            Self::Animal_Insect_S => Actor::Animal_Insect_S,
            Self::Animal_Insect_M => Actor::Animal_Insect_M,
            Self::Animal_Insect_X => Actor::Animal_Insect_X,
            Self::Item_Ore_I => Actor::Item_Ore_I,
            Self::Item_Ore_F => Actor::Item_Ore_F,
            Self::Item_Ore_E => Actor::Item_Ore_E,
            Self::Item_Ore_G => Actor::Item_Ore_G,
            Self::Item_Ore_D => Actor::Item_Ore_D,
            Self::Item_Ore_B => Actor::Item_Ore_B,
            Self::Item_Ore_C => Actor::Item_Ore_C,
            Self::Item_Ore_A => Actor::Item_Ore_A,
            Self::Item_Enemy_00 => Actor::Item_Enemy_00,
            Self::Item_Enemy_01 => Actor::Item_Enemy_01,
            Self::Item_Enemy_Grp_104 => Actor::Item_Enemy_02,
            Self::Item_Enemy_Grp_105 => Actor::Item_Enemy_06,
            Self::Item_Enemy_07 => Actor::Item_Enemy_07,
            Self::Item_Enemy_08 => Actor::Item_Enemy_08,
            Self::Item_Enemy_Grp_108 => Actor::Item_Enemy_03,
            Self::Item_Enemy_04 => Actor::Item_Enemy_04,
            Self::Item_Enemy_05 => Actor::Item_Enemy_05,
            Self::Item_Enemy_Grp_111 => Actor::Item_Enemy_42,
            Self::Item_Enemy_12 => Actor::Item_Enemy_12,
            Self::Item_Enemy_13 => Actor::Item_Enemy_13,
            Self::Item_Enemy_14 => Actor::Item_Enemy_14,
            Self::Item_Enemy_Grp_115 => Actor::Item_Enemy_17,
            Self::Item_Enemy_18 => Actor::Item_Enemy_18,
            Self::Item_Enemy_Grp_117 => Actor::Item_Enemy_46,
            Self::Item_Enemy_21 => Actor::Item_Enemy_21,
            Self::Item_Enemy_24 => Actor::Item_Enemy_24,
            Self::Item_Enemy_25 => Actor::Item_Enemy_25,
            Self::Item_Enemy_32 => Actor::Item_Enemy_32,
            Self::Item_Enemy_33 => Actor::Item_Enemy_33,
            Self::Item_Enemy_34 => Actor::Item_Enemy_34,
            Self::Item_Enemy_27 => Actor::Item_Enemy_27,
            Self::Item_Enemy_28 => Actor::Item_Enemy_28,
            Self::Item_Enemy_26 => Actor::Item_Enemy_26,
            Self::Item_Enemy_29 => Actor::Item_Enemy_29,
            Self::Item_Enemy_30 => Actor::Item_Enemy_30,
            Self::Item_Enemy_31 => Actor::Item_Enemy_31,
            Self::Obj_FireWoodBundle => Actor::Obj_FireWoodBundle,
            Self::Item_Roast_03 => Actor::Item_Roast_03,
            Self::Item_Roast_10 => Actor::Item_Roast_10,
            Self::Item_Roast_07 => Actor::Item_Roast_07,
            Self::Item_Roast_Grp_134 => Actor::Item_Roast_48,
            Self::Item_Roast_09 => Actor::Item_Roast_09,
            Self::Item_Roast_Grp_136 => Actor::Item_Roast_12,
            Self::Item_Roast_13 => Actor::Item_Roast_13,
            Self::Item_Roast_16 => Actor::Item_Roast_16,
            Self::Item_Roast_06 => Actor::Item_Roast_06,
            Self::Item_Roast_Grp_140 => Actor::Item_Roast_04,
            Self::Item_Roast_53 => Actor::Item_Roast_53,
            Self::Item_Roast_05 => Actor::Item_Roast_05,
            Self::Item_Roast_49 => Actor::Item_Roast_49,
            Self::Item_Roast_Grp_144 => Actor::Item_Roast_31,
            Self::Item_Roast_39 => Actor::Item_Roast_39,
            Self::Item_Roast_18 => Actor::Item_Roast_18,
            Self::Item_Roast_19 => Actor::Item_Roast_19,
            Self::Item_Roast_24 => Actor::Item_Roast_24,
            Self::Item_Roast_50 => Actor::Item_Roast_50,
            Self::Item_Roast_15 => Actor::Item_Roast_15,
            Self::Item_Roast_Grp_151 => Actor::Item_Roast_27,
            Self::Item_Grp_152 => Actor::Item_Roast_51,
            Self::Item_Roast_Grp_153 => Actor::Item_Roast_01,
            Self::Item_Roast_40 => Actor::Item_Roast_40,
            Self::Item_Roast_Grp_155 => Actor::Item_Roast_45,
            Self::Item_Roast_41 => Actor::Item_Roast_41,
            Self::Item_RoastFish_01 => Actor::Item_RoastFish_01,
            Self::Item_RoastFish_02 => Actor::Item_RoastFish_02,
            Self::Item_RoastFish_Grp_159 => Actor::Item_RoastFish_04,
            Self::Item_RoastFish_Grp_160 => Actor::Item_RoastFish_03,
            Self::Item_RoastFish_Grp_161 => Actor::Item_RoastFish_07,
            Self::Item_RoastFish_15 => Actor::Item_RoastFish_15,
            Self::Item_Chilled_Grp_163 => Actor::Item_Chilled_01,
            Self::Item_Chilled_Grp_164 => Actor::Item_Chilled_02,
            Self::Item_Chilled_Grp_165 => Actor::Item_Chilled_03,
            Self::Item_ChilledFish_Grp_166 => Actor::Item_ChilledFish_01,
            Self::Item_ChilledFish_02 => Actor::Item_ChilledFish_02,
            Self::Item_ChilledFish_Grp_168 => Actor::Item_ChilledFish_03,
            Self::Item_ChilledFish_Grp_169 => Actor::Item_ChilledFish_04,
            Self::Item_ChilledFish_09 => Actor::Item_ChilledFish_09,
            Self::Obj_DRStone_Get => Actor::Obj_DRStone_Get,
            Self::dyecolor_Grp_172 => Actor::dyecolor_00,
            Self::Obj_Photo_Grp_173 => Actor::Obj_Photo_Animal,
            Self::Obj_Photo_BossEnemy => Actor::Obj_Photo_BossEnemy,
        }
    }
    /// Get if any actor in the group is only holdable with PE
    pub const fn contains_pe_only(&self) -> bool {
        match self {
            Self::None => false,
            Self::Item_Roast_03 => true,
            Self::Item_Roast_10 => true,
            Self::Item_Roast_07 => true,
            Self::Item_Roast_Grp_134 => true,
            Self::Item_Roast_09 => true,
            Self::Item_Roast_Grp_136 => true,
            Self::Item_Roast_13 => true,
            Self::Item_Roast_16 => true,
            Self::Item_Roast_06 => true,
            Self::Item_Roast_Grp_140 => true,
            Self::Item_Roast_53 => true,
            Self::Item_Roast_05 => true,
            Self::Item_Roast_49 => true,
            Self::Item_Roast_Grp_144 => true,
            Self::Item_Roast_39 => true,
            Self::Item_Roast_18 => true,
            Self::Item_Roast_19 => true,
            Self::Item_Roast_24 => true,
            Self::Item_Roast_50 => true,
            Self::Item_Roast_15 => true,
            Self::Item_Roast_Grp_151 => true,
            Self::Item_Grp_152 => true,
            Self::Item_Roast_Grp_153 => true,
            Self::Item_Roast_40 => true,
            Self::Item_Roast_Grp_155 => true,
            Self::Item_Roast_41 => true,
            Self::Item_RoastFish_01 => true,
            Self::Item_RoastFish_02 => true,
            Self::Item_RoastFish_Grp_159 => true,
            Self::Item_RoastFish_Grp_160 => true,
            Self::Item_RoastFish_Grp_161 => true,
            Self::Item_RoastFish_15 => true,
            Self::Item_Chilled_Grp_163 => true,
            Self::Item_Chilled_Grp_164 => true,
            Self::Item_Chilled_Grp_165 => true,
            Self::Item_ChilledFish_Grp_166 => true,
            Self::Item_ChilledFish_02 => true,
            Self::Item_ChilledFish_Grp_168 => true,
            Self::Item_ChilledFish_Grp_169 => true,
            Self::Item_ChilledFish_09 => true,
            Self::Obj_DRStone_Get => true,
            Self::dyecolor_Grp_172 => true,
            Self::Obj_Photo_Grp_173 => true,
            Self::Obj_Photo_BossEnemy => true,
            _ => false,
        }
    }
}
