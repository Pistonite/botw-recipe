//! Automatically generated.
//!
//! DO NOT EDIT. See packages/generated/README.md for more information.

/// Tags used in the cooking code/recipes
#[cfg_attr(feature = "tag-enum-map", derive(enum_map::Enum))]
#[cfg_attr(
    feature = "tag-enum-set",
    derive(enumset::EnumSetType, PartialOrd, Ord, Hash)
)]
#[cfg_attr(
    not(feature = "tag-enum-set"),
    derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)
)]
#[allow(non_camel_case_types)]
#[derive(Default)]
#[repr(u8)]
pub enum Tag {
    /// No tag. This is used to make recipe matching implementation cleaner
    #[default]
    None = 0,
    CookEnemy,
    CookFish,
    CookFruit,
    CookInsect,
    CookLowPrice,
    CookMeat,
    CookMushroom,
    CookOre,
    CookPlant,
    CookSpice,
    CookVegetable,
}
impl Tag {
    /// Get the string representation of the tag
    #[cfg(feature = "tag-to-str")]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "<none>",
            Self::CookEnemy => "CookEnemy",
            Self::CookFish => "CookFish",
            Self::CookFruit => "CookFruit",
            Self::CookInsect => "CookInsect",
            Self::CookLowPrice => "CookLowPrice",
            Self::CookMeat => "CookMeat",
            Self::CookMushroom => "CookMushroom",
            Self::CookOre => "CookOre",
            Self::CookPlant => "CookPlant",
            Self::CookSpice => "CookSpice",
            Self::CookVegetable => "CookVegetable",
        }
    }
    /// Check if the tag is used in recipe matching
    ///
    /// Each actor should have at most 1 of these tags
    pub const fn is_used_in_recipe_matching(self) -> bool {
        match self {
            Self::CookEnemy => true,
            Self::CookFish => true,
            Self::CookFruit => true,
            Self::CookInsect => true,
            Self::CookMeat => true,
            Self::CookMushroom => true,
            Self::CookOre => true,
            Self::CookPlant => true,
            Self::CookVegetable => true,
            _ => false,
        }
    }
    /// Get the tag from string representation
    #[cfg(feature = "tag-from-str")]
    pub fn from_str(name: &str) -> Option<Self> {
        TAG_STR_MAP.get(name).copied()
    }
}
#[cfg(feature = "tag-from-str")]
static TAG_STR_MAP: phf::Map<&'static str, Tag> = phf::phf_map! {
    "CookEnemy" => Tag::CookEnemy,
    "CookFish" => Tag::CookFish,
    "CookFruit" => Tag::CookFruit,
    "CookInsect" => Tag::CookInsect,
    "CookLowPrice" => Tag::CookLowPrice,
    "CookMeat" => Tag::CookMeat,
    "CookMushroom" => Tag::CookMushroom,
    "CookOre" => Tag::CookOre,
    "CookPlant" => Tag::CookPlant,
    "CookSpice" => Tag::CookSpice,
    "CookVegetable" => Tag::CookVegetable,
};
/// Get the count of the tag enum
///
/// `count - 1` is the last valid enum variant
#[macro_export]
macro_rules! tag_count {
    () => {
        12
    };
}
