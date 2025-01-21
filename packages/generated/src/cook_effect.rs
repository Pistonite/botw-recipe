//! Automatically generated.
//!
//! DO NOT EDIT. See packages/generated/README.md for more information.

/// Effect of cooked food
#[cfg_attr(feature = "cook-effect-enum-map", derive(enum_map::Enum))]
#[cfg_attr(
    feature = "cook-effect-enum-set",
    derive(enumset::EnumSetType, PartialOrd, Ord, Hash)
)]
#[cfg_attr(
    not(feature = "cook-effect-enum-set"),
    derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)
)]
#[derive(Default)]
#[repr(u8)]
pub enum CookEffect {
    /// No effect
    #[default]
    None = 0,
    /// Hasty
    MovingSpeed,
    /// Mighty
    AttackUp,
    /// Tough
    DefenseUp,
    /// Enduring
    ExGutsMaxUp,
    /// Fireproof
    Fireproof,
    /// Energizing
    GutsRecover,
    /// Hearty
    LifeMaxUp,
    LifeRecover,
    /// Sneaky
    Quietness,
    /// Spicy
    ResistCold,
    /// Electro
    ResistElectric,
    /// Chilly
    ResistHot,
}
impl CookEffect {
    /// Get the string representation of the effect
    #[cfg(feature = "cook-effect-to-str")]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::MovingSpeed => "MovingSpeed",
            Self::AttackUp => "AttackUp",
            Self::DefenseUp => "DefenseUp",
            Self::ExGutsMaxUp => "ExGutsMaxUp",
            Self::Fireproof => "Fireproof",
            Self::GutsRecover => "GutsRecover",
            Self::LifeMaxUp => "LifeMaxUp",
            Self::LifeRecover => "LifeRecover",
            Self::Quietness => "Quietness",
            Self::ResistCold => "ResistCold",
            Self::ResistElectric => "ResistElectric",
            Self::ResistHot => "ResistHot",
        }
    }
    /// Get the effect from string representation
    #[cfg(feature = "cook-effect-from-str")]
    pub fn from_str(name: &str) -> Option<Self> {
        COOK_EFFECT_STR_MAP.get(name).copied()
    }
    /// Get the English name of the effect
    #[cfg(feature = "cook-effect-english")]
    pub const fn name(self) -> &'static str {
        match self {
            Self::None => "",
            Self::MovingSpeed => "Hasty",
            Self::AttackUp => "Mighty",
            Self::DefenseUp => "Tough",
            Self::ExGutsMaxUp => "Enduring",
            Self::Fireproof => "Fireproof",
            Self::GutsRecover => "Energizing",
            Self::LifeMaxUp => "Hearty",
            Self::LifeRecover => "",
            Self::Quietness => "Sneaky",
            Self::ResistCold => "Spicy",
            Self::ResistElectric => "Electro",
            Self::ResistHot => "Chilly",
        }
    }
    /// Get the name of the SpecialStatus associated with this effect
    ///
    /// This is usually the same as the string representation, except for:
    /// - MovingSpeed -> AllSpeed
    /// - LifeRecover -> (doesn't have one)#[cfg(feature = "cook-effect-special-status")]
    pub const fn special_status(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::MovingSpeed => Some("AllSpeed"),
            Self::AttackUp => Some("AttackUp"),
            Self::DefenseUp => Some("DefenseUp"),
            Self::ExGutsMaxUp => Some("ExGutsMaxUp"),
            Self::Fireproof => Some("Fireproof"),
            Self::GutsRecover => Some("GutsRecover"),
            Self::LifeMaxUp => Some("LifeMaxUp"),
            Self::LifeRecover => None,
            Self::Quietness => Some("Quietness"),
            Self::ResistCold => Some("ResistCold"),
            Self::ResistElectric => Some("ResistElectric"),
            Self::ResistHot => Some("ResistHot"),
        }
    }
    /// Get the base time of the effect
    ///
    /// For effects that are not time based, this is 0
    #[cfg(feature = "cook-effect-data")]
    pub const fn base_time(self) -> u32 {
        match self {
            Self::None => 0,
            Self::MovingSpeed => 30,
            Self::AttackUp => 20,
            Self::DefenseUp => 20,
            Self::ExGutsMaxUp => 0,
            Self::Fireproof => 120,
            Self::GutsRecover => 0,
            Self::LifeMaxUp => 0,
            Self::LifeRecover => 0,
            Self::Quietness => 90,
            Self::ResistCold => 120,
            Self::ResistElectric => 120,
            Self::ResistHot => 120,
        }
    }
    /// Get the maximum level of the effect
    #[cfg(feature = "cook-effect-data")]
    pub const fn max_level(self) -> u32 {
        match self {
            Self::None => 0,
            Self::MovingSpeed => 3,
            Self::AttackUp => 3,
            Self::DefenseUp => 3,
            Self::ExGutsMaxUp => 20,
            Self::Fireproof => 2,
            Self::GutsRecover => 15,
            Self::LifeMaxUp => 108,
            Self::LifeRecover => 120,
            Self::Quietness => 3,
            Self::ResistCold => 2,
            Self::ResistElectric => 3,
            Self::ResistHot => 2,
        }
    }
    /// Get the minimum level of the effect
    #[cfg(feature = "cook-effect-data")]
    pub const fn min_level(self) -> u32 {
        match self {
            Self::None => 0,
            Self::MovingSpeed => 1,
            Self::AttackUp => 1,
            Self::DefenseUp => 1,
            Self::ExGutsMaxUp => 1,
            Self::Fireproof => 1,
            Self::GutsRecover => 1,
            Self::LifeMaxUp => 4,
            Self::LifeRecover => 1,
            Self::Quietness => 1,
            Self::ResistCold => 1,
            Self::ResistElectric => 1,
            Self::ResistHot => 1,
        }
    }
    /// Get the super success amount (SSA) of the effect
    #[cfg(feature = "cook-effect-data")]
    pub const fn super_success_amount(self) -> u32 {
        match self {
            Self::None => 0,
            Self::MovingSpeed => 1,
            Self::AttackUp => 1,
            Self::DefenseUp => 1,
            Self::ExGutsMaxUp => 2,
            Self::Fireproof => 1,
            Self::GutsRecover => 2,
            Self::LifeMaxUp => 4,
            Self::LifeRecover => 12,
            Self::Quietness => 1,
            Self::ResistCold => 1,
            Self::ResistElectric => 1,
            Self::ResistHot => 1,
        }
    }
    /// Convert the cook effect to the game enum value
    pub const fn game_repr(self) -> i32 {
        match self {
            Self::None => -1,
            Self::MovingSpeed => 13,
            Self::AttackUp => 10,
            Self::DefenseUp => 11,
            Self::ExGutsMaxUp => 15,
            Self::Fireproof => 16,
            Self::GutsRecover => 14,
            Self::LifeMaxUp => 2,
            Self::LifeRecover => 1,
            Self::Quietness => 12,
            Self::ResistCold => 5,
            Self::ResistElectric => 6,
            Self::ResistHot => 4,
        }
    }
    /// Convert game enum value to the cook effect
    pub fn from_game_repr(value: f32) -> Option<Self> {
        match value {
            -1. => Some(Self::None),
            13. => Some(Self::MovingSpeed),
            10. => Some(Self::AttackUp),
            11. => Some(Self::DefenseUp),
            15. => Some(Self::ExGutsMaxUp),
            16. => Some(Self::Fireproof),
            14. => Some(Self::GutsRecover),
            2. => Some(Self::LifeMaxUp),
            1. => Some(Self::LifeRecover),
            12. => Some(Self::Quietness),
            5. => Some(Self::ResistCold),
            6. => Some(Self::ResistElectric),
            4. => Some(Self::ResistHot),
            _ => None,
        }
    }
}
#[cfg(feature = "cook-effect-from-str")]
static COOK_EFFECT_STR_MAP: phf::Map<&'static str, CookEffect> = phf::phf_map! {
    "None" => CookEffect::None,
    "MovingSpeed" => CookEffect::MovingSpeed,
    "AttackUp" => CookEffect::AttackUp,
    "DefenseUp" => CookEffect::DefenseUp,
    "ExGutsMaxUp" => CookEffect::ExGutsMaxUp,
    "Fireproof" => CookEffect::Fireproof,
    "GutsRecover" => CookEffect::GutsRecover,
    "LifeMaxUp" => CookEffect::LifeMaxUp,
    "LifeRecover" => CookEffect::LifeRecover,
    "Quietness" => CookEffect::Quietness,
    "ResistCold" => CookEffect::ResistCold,
    "ResistElectric" => CookEffect::ResistElectric,
    "ResistHot" => CookEffect::ResistHot,
};
/// Get the count of the cook_effect enum
///
/// `count - 1` is the last valid enum variant
#[macro_export]
macro_rules! cook_effect_count {
    () => {
        13
    };
}
