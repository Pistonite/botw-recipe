//! Automatically generated.
//!
//! DO NOT EDIT
//!
//! Run `cd research && python main.py` (or `task research`) to regenerate.

///
/// Number of "ingredients" in a recipe
///
/// Equivalent actors are grouped together. This also includes the "<none>" ingredient,
/// which indicates empty space (for example, a recipe with 4 items has 1 empty space).
///
pub const NUM_GROUPS: usize = 175;

///
/// Number of ingredients in a recipe record. Always 5
///
pub const NUM_INGR: usize = 5;

///
/// Number of total recipe records
///
/// This is choosing NUM_INGR from NUM_GROUPS, allowing for repetition.
/// In other words, binomial(NUM_GROUPS+NUM_INGR-1, NUM_INGR),
/// or equivalently, NUM_GROUPS multichoose NUM_INGR.
///
pub const NUM_TOTAL_RECORDS: usize = 1663834536;
