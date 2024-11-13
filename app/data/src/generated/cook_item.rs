//! Automatically generated.
//!
//! DO NOT EDIT
//!
//! Run `cd research && python main.py` (or `task research`) to regenerate.

/// Cooked Item (Output of cooking pot)
#[derive(
    enum_map::Enum,
    serde::Serialize,
    serde::Deserialize,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[allow(non_camel_case_types)]
pub enum CookItem {
    /// Mushroom Skewer
    Item_Cook_A_01,
    /// Steamed Mushrooms
    Item_Cook_A_02,
    /// Steamed Fruit
    Item_Cook_A_03,
    /// Steamed Fish
    Item_Cook_A_04,
    /// Steamed Meat
    Item_Cook_A_05,
    /// Fruit and Mushroom Mix
    Item_Cook_A_07,
    /// Fish and Mushroom Skewer
    Item_Cook_A_08,
    /// Meat and Mushroom Skewer
    Item_Cook_A_09,
    /// Omelet
    Item_Cook_A_10,
    /// Glazed Mushrooms
    Item_Cook_A_11,
    /// Glazed Meat
    Item_Cook_A_12,
    /// Glazed Seafood
    Item_Cook_A_13,
    /// Glazed Veggies
    Item_Cook_A_14,
    /// Fried Wild Greens
    Item_Cook_B_01,
    /// Simmered Fruit
    Item_Cook_B_02,
    /// Fish Skewer
    Item_Cook_B_05,
    /// Meat Skewer
    Item_Cook_B_06,
    /// Copious Fried Wild Greens
    Item_Cook_B_11,
    /// Copious Simmered Fruit
    Item_Cook_B_12,
    /// Copious Mushroom Skewers
    Item_Cook_B_13,
    /// Copious Seafood Skewers
    Item_Cook_B_15,
    /// Copious Meat Skewers
    Item_Cook_B_16,
    /// Meat and Seafood Fry
    Item_Cook_B_17,
    /// Prime Meat and Seafood Fry
    Item_Cook_B_18,
    /// Gourmet Meat and Seafood Fry
    Item_Cook_B_19,
    /// Meat-Stuffed Pumpkin
    Item_Cook_B_20,
    /// Sautéed Peppers
    Item_Cook_B_21,
    /// Sautéed Nuts
    Item_Cook_B_22,
    /// Seafood Skewer
    Item_Cook_B_23,
    /// Fairy Tonic
    Item_Cook_C_16,
    /// Elixir
    Item_Cook_C_17,
    /// Salt-Grilled Mushrooms
    Item_Cook_D_01,
    /// Salt-Grilled Greens
    Item_Cook_D_02,
    /// Salt-Grilled Fish
    Item_Cook_D_03,
    /// Salt-Grilled Meat
    Item_Cook_D_04,
    /// Salt-Grilled Prime Meat
    Item_Cook_D_05,
    /// Salt-Grilled Gourmet Meat
    Item_Cook_D_06,
    /// Pepper Steak
    Item_Cook_D_07,
    /// Pepper Seafood
    Item_Cook_D_08,
    /// Salt-Grilled Crab
    Item_Cook_D_09,
    /// Crab Stir-Fry
    Item_Cook_D_10,
    /// Poultry Pilaf
    Item_Cook_E_01,
    /// Prime Poultry Pilaf
    Item_Cook_E_02,
    /// Gourmet Poultry Pilaf
    Item_Cook_E_03,
    /// Fried Egg and Rice
    Item_Cook_E_04,
    /// Creamy Meat Soup
    Item_Cook_F_01,
    /// Creamy Seafood Soup
    Item_Cook_F_02,
    /// Veggie Cream Soup
    Item_Cook_F_03,
    /// Creamy Heart Soup
    Item_Cook_F_04,
    /// Seafood Rice Balls
    Item_Cook_G_02,
    /// Veggie Rice Balls
    Item_Cook_G_03,
    /// Mushroom Rice Balls
    Item_Cook_G_04,
    /// Meat and Rice Bowl
    Item_Cook_G_05,
    /// Prime Meat and Rice Bowl
    Item_Cook_G_06,
    /// Gourmet Meat and Rice Bowl
    Item_Cook_G_09,
    /// Seafood Fried Rice
    Item_Cook_G_10,
    /// Curry Pilaf
    Item_Cook_G_11,
    /// Mushroom Risotto
    Item_Cook_G_12,
    /// Vegetable Risotto
    Item_Cook_G_13,
    /// Salmon Risotto
    Item_Cook_G_14,
    /// Meaty Rice Balls
    Item_Cook_G_15,
    /// Crab Omelet with Rice
    Item_Cook_G_16,
    /// Crab Risotto
    Item_Cook_G_17,
    /// Seafood Meunière
    Item_Cook_H_01,
    /// Porgy Meunière
    Item_Cook_H_02,
    /// Salmon Meunière
    Item_Cook_H_03,
    /// Fruit Pie
    Item_Cook_I_01,
    /// Apple Pie
    Item_Cook_I_02,
    /// Egg Tart
    Item_Cook_I_03,
    /// Meat Pie
    Item_Cook_I_04,
    /// Carrot Cake
    Item_Cook_I_05,
    /// Pumpkin Pie
    Item_Cook_I_06,
    /// Hot Buttered Apple
    Item_Cook_I_07,
    /// Honeyed Apple
    Item_Cook_I_08,
    /// Honeyed Fruits
    Item_Cook_I_09,
    /// Plain Crepe
    Item_Cook_I_10,
    /// Wildberry Crepe
    Item_Cook_I_11,
    /// Nutcake
    Item_Cook_I_12,
    /// Fried Bananas
    Item_Cook_I_13,
    /// Egg Pudding
    Item_Cook_I_14,
    /// Fish Pie
    Item_Cook_I_15,
    /// Honey Candy
    Item_Cook_I_16,
    /// Honey Crepe
    Item_Cook_I_17,
    /// Curry Rice
    Item_Cook_J_01,
    /// Vegetable Curry
    Item_Cook_J_02,
    /// Seafood Curry
    Item_Cook_J_03,
    /// Poultry Curry
    Item_Cook_J_04,
    /// Prime Poultry Curry
    Item_Cook_J_05,
    /// Meat Curry
    Item_Cook_J_06,
    /// Prime Meat Curry
    Item_Cook_J_07,
    /// Gourmet Poultry Curry
    Item_Cook_J_08,
    /// Gourmet Meat Curry
    Item_Cook_J_09,
    /// Meat Stew
    Item_Cook_K_01,
    /// Prime Meat Stew
    Item_Cook_K_02,
    /// Pumpkin Stew
    Item_Cook_K_03,
    /// Clam Chowder
    Item_Cook_K_04,
    /// Gourmet Meat Stew
    Item_Cook_K_05,
    /// Cream of Mushroom Soup
    Item_Cook_K_06,
    /// Cream of Vegetable Soup
    Item_Cook_K_07,
    /// Carrot Stew
    Item_Cook_K_08,
    /// Milk
    Item_Cook_K_09,
    /// Wheat Bread
    Item_Cook_M_01,
    /// Seafood Paella
    Item_Cook_N_01,
    /// Fruitcake
    Item_Cook_N_02,
    /// Vegetable Omelet
    Item_Cook_N_03,
    /// Mushroom Omelet
    Item_Cook_N_04,
    /// Dubious Food
    Item_Cook_O_01,
    /// Rock-Hard Food
    Item_Cook_O_02,
    /// Fragrant Mushroom Sauté
    Item_Cook_P_01,
    /// Herb Sauté
    Item_Cook_P_02,
    /// Spiced Meat Skewer
    Item_Cook_P_03,
    /// Prime Spiced Meat Skewer
    Item_Cook_P_04,
    /// Gourmet Spiced Meat Skewer
    Item_Cook_P_05,
}
impl CookItem {
    /// Get the English name of the cook item
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Item_Cook_A_01 => "Mushroom Skewer",
            Self::Item_Cook_A_02 => "Steamed Mushrooms",
            Self::Item_Cook_A_03 => "Steamed Fruit",
            Self::Item_Cook_A_04 => "Steamed Fish",
            Self::Item_Cook_A_05 => "Steamed Meat",
            Self::Item_Cook_A_07 => "Fruit and Mushroom Mix",
            Self::Item_Cook_A_08 => "Fish and Mushroom Skewer",
            Self::Item_Cook_A_09 => "Meat and Mushroom Skewer",
            Self::Item_Cook_A_10 => "Omelet",
            Self::Item_Cook_A_11 => "Glazed Mushrooms",
            Self::Item_Cook_A_12 => "Glazed Meat",
            Self::Item_Cook_A_13 => "Glazed Seafood",
            Self::Item_Cook_A_14 => "Glazed Veggies",
            Self::Item_Cook_B_01 => "Fried Wild Greens",
            Self::Item_Cook_B_02 => "Simmered Fruit",
            Self::Item_Cook_B_05 => "Fish Skewer",
            Self::Item_Cook_B_06 => "Meat Skewer",
            Self::Item_Cook_B_11 => "Copious Fried Wild Greens",
            Self::Item_Cook_B_12 => "Copious Simmered Fruit",
            Self::Item_Cook_B_13 => "Copious Mushroom Skewers",
            Self::Item_Cook_B_15 => "Copious Seafood Skewers",
            Self::Item_Cook_B_16 => "Copious Meat Skewers",
            Self::Item_Cook_B_17 => "Meat and Seafood Fry",
            Self::Item_Cook_B_18 => "Prime Meat and Seafood Fry",
            Self::Item_Cook_B_19 => "Gourmet Meat and Seafood Fry",
            Self::Item_Cook_B_20 => "Meat-Stuffed Pumpkin",
            Self::Item_Cook_B_21 => "Sautéed Peppers",
            Self::Item_Cook_B_22 => "Sautéed Nuts",
            Self::Item_Cook_B_23 => "Seafood Skewer",
            Self::Item_Cook_C_16 => "Fairy Tonic",
            Self::Item_Cook_C_17 => "Elixir",
            Self::Item_Cook_D_01 => "Salt-Grilled Mushrooms",
            Self::Item_Cook_D_02 => "Salt-Grilled Greens",
            Self::Item_Cook_D_03 => "Salt-Grilled Fish",
            Self::Item_Cook_D_04 => "Salt-Grilled Meat",
            Self::Item_Cook_D_05 => "Salt-Grilled Prime Meat",
            Self::Item_Cook_D_06 => "Salt-Grilled Gourmet Meat",
            Self::Item_Cook_D_07 => "Pepper Steak",
            Self::Item_Cook_D_08 => "Pepper Seafood",
            Self::Item_Cook_D_09 => "Salt-Grilled Crab",
            Self::Item_Cook_D_10 => "Crab Stir-Fry",
            Self::Item_Cook_E_01 => "Poultry Pilaf",
            Self::Item_Cook_E_02 => "Prime Poultry Pilaf",
            Self::Item_Cook_E_03 => "Gourmet Poultry Pilaf",
            Self::Item_Cook_E_04 => "Fried Egg and Rice",
            Self::Item_Cook_F_01 => "Creamy Meat Soup",
            Self::Item_Cook_F_02 => "Creamy Seafood Soup",
            Self::Item_Cook_F_03 => "Veggie Cream Soup",
            Self::Item_Cook_F_04 => "Creamy Heart Soup",
            Self::Item_Cook_G_02 => "Seafood Rice Balls",
            Self::Item_Cook_G_03 => "Veggie Rice Balls",
            Self::Item_Cook_G_04 => "Mushroom Rice Balls",
            Self::Item_Cook_G_05 => "Meat and Rice Bowl",
            Self::Item_Cook_G_06 => "Prime Meat and Rice Bowl",
            Self::Item_Cook_G_09 => "Gourmet Meat and Rice Bowl",
            Self::Item_Cook_G_10 => "Seafood Fried Rice",
            Self::Item_Cook_G_11 => "Curry Pilaf",
            Self::Item_Cook_G_12 => "Mushroom Risotto",
            Self::Item_Cook_G_13 => "Vegetable Risotto",
            Self::Item_Cook_G_14 => "Salmon Risotto",
            Self::Item_Cook_G_15 => "Meaty Rice Balls",
            Self::Item_Cook_G_16 => "Crab Omelet with Rice",
            Self::Item_Cook_G_17 => "Crab Risotto",
            Self::Item_Cook_H_01 => "Seafood Meunière",
            Self::Item_Cook_H_02 => "Porgy Meunière",
            Self::Item_Cook_H_03 => "Salmon Meunière",
            Self::Item_Cook_I_01 => "Fruit Pie",
            Self::Item_Cook_I_02 => "Apple Pie",
            Self::Item_Cook_I_03 => "Egg Tart",
            Self::Item_Cook_I_04 => "Meat Pie",
            Self::Item_Cook_I_05 => "Carrot Cake",
            Self::Item_Cook_I_06 => "Pumpkin Pie",
            Self::Item_Cook_I_07 => "Hot Buttered Apple",
            Self::Item_Cook_I_08 => "Honeyed Apple",
            Self::Item_Cook_I_09 => "Honeyed Fruits",
            Self::Item_Cook_I_10 => "Plain Crepe",
            Self::Item_Cook_I_11 => "Wildberry Crepe",
            Self::Item_Cook_I_12 => "Nutcake",
            Self::Item_Cook_I_13 => "Fried Bananas",
            Self::Item_Cook_I_14 => "Egg Pudding",
            Self::Item_Cook_I_15 => "Fish Pie",
            Self::Item_Cook_I_16 => "Honey Candy",
            Self::Item_Cook_I_17 => "Honey Crepe",
            Self::Item_Cook_J_01 => "Curry Rice",
            Self::Item_Cook_J_02 => "Vegetable Curry",
            Self::Item_Cook_J_03 => "Seafood Curry",
            Self::Item_Cook_J_04 => "Poultry Curry",
            Self::Item_Cook_J_05 => "Prime Poultry Curry",
            Self::Item_Cook_J_06 => "Meat Curry",
            Self::Item_Cook_J_07 => "Prime Meat Curry",
            Self::Item_Cook_J_08 => "Gourmet Poultry Curry",
            Self::Item_Cook_J_09 => "Gourmet Meat Curry",
            Self::Item_Cook_K_01 => "Meat Stew",
            Self::Item_Cook_K_02 => "Prime Meat Stew",
            Self::Item_Cook_K_03 => "Pumpkin Stew",
            Self::Item_Cook_K_04 => "Clam Chowder",
            Self::Item_Cook_K_05 => "Gourmet Meat Stew",
            Self::Item_Cook_K_06 => "Cream of Mushroom Soup",
            Self::Item_Cook_K_07 => "Cream of Vegetable Soup",
            Self::Item_Cook_K_08 => "Carrot Stew",
            Self::Item_Cook_K_09 => "Milk",
            Self::Item_Cook_M_01 => "Wheat Bread",
            Self::Item_Cook_N_01 => "Seafood Paella",
            Self::Item_Cook_N_02 => "Fruitcake",
            Self::Item_Cook_N_03 => "Vegetable Omelet",
            Self::Item_Cook_N_04 => "Mushroom Omelet",
            Self::Item_Cook_O_01 => "Dubious Food",
            Self::Item_Cook_O_02 => "Rock-Hard Food",
            Self::Item_Cook_P_01 => "Fragrant Mushroom Sauté",
            Self::Item_Cook_P_02 => "Herb Sauté",
            Self::Item_Cook_P_03 => "Spiced Meat Skewer",
            Self::Item_Cook_P_04 => "Prime Spiced Meat Skewer",
            Self::Item_Cook_P_05 => "Gourmet Spiced Meat Skewer",
        }
    }
    /// Get the actor name of the cook item
    pub const fn actor_name(&self) -> &'static str {
        match self {
            Self::Item_Cook_A_01 => "Item_Cook_A_01",
            Self::Item_Cook_A_02 => "Item_Cook_A_02",
            Self::Item_Cook_A_03 => "Item_Cook_A_03",
            Self::Item_Cook_A_04 => "Item_Cook_A_04",
            Self::Item_Cook_A_05 => "Item_Cook_A_05",
            Self::Item_Cook_A_07 => "Item_Cook_A_07",
            Self::Item_Cook_A_08 => "Item_Cook_A_08",
            Self::Item_Cook_A_09 => "Item_Cook_A_09",
            Self::Item_Cook_A_10 => "Item_Cook_A_10",
            Self::Item_Cook_A_11 => "Item_Cook_A_11",
            Self::Item_Cook_A_12 => "Item_Cook_A_12",
            Self::Item_Cook_A_13 => "Item_Cook_A_13",
            Self::Item_Cook_A_14 => "Item_Cook_A_14",
            Self::Item_Cook_B_01 => "Item_Cook_B_01",
            Self::Item_Cook_B_02 => "Item_Cook_B_02",
            Self::Item_Cook_B_05 => "Item_Cook_B_05",
            Self::Item_Cook_B_06 => "Item_Cook_B_06",
            Self::Item_Cook_B_11 => "Item_Cook_B_11",
            Self::Item_Cook_B_12 => "Item_Cook_B_12",
            Self::Item_Cook_B_13 => "Item_Cook_B_13",
            Self::Item_Cook_B_15 => "Item_Cook_B_15",
            Self::Item_Cook_B_16 => "Item_Cook_B_16",
            Self::Item_Cook_B_17 => "Item_Cook_B_17",
            Self::Item_Cook_B_18 => "Item_Cook_B_18",
            Self::Item_Cook_B_19 => "Item_Cook_B_19",
            Self::Item_Cook_B_20 => "Item_Cook_B_20",
            Self::Item_Cook_B_21 => "Item_Cook_B_21",
            Self::Item_Cook_B_22 => "Item_Cook_B_22",
            Self::Item_Cook_B_23 => "Item_Cook_B_23",
            Self::Item_Cook_C_16 => "Item_Cook_C_16",
            Self::Item_Cook_C_17 => "Item_Cook_C_17",
            Self::Item_Cook_D_01 => "Item_Cook_D_01",
            Self::Item_Cook_D_02 => "Item_Cook_D_02",
            Self::Item_Cook_D_03 => "Item_Cook_D_03",
            Self::Item_Cook_D_04 => "Item_Cook_D_04",
            Self::Item_Cook_D_05 => "Item_Cook_D_05",
            Self::Item_Cook_D_06 => "Item_Cook_D_06",
            Self::Item_Cook_D_07 => "Item_Cook_D_07",
            Self::Item_Cook_D_08 => "Item_Cook_D_08",
            Self::Item_Cook_D_09 => "Item_Cook_D_09",
            Self::Item_Cook_D_10 => "Item_Cook_D_10",
            Self::Item_Cook_E_01 => "Item_Cook_E_01",
            Self::Item_Cook_E_02 => "Item_Cook_E_02",
            Self::Item_Cook_E_03 => "Item_Cook_E_03",
            Self::Item_Cook_E_04 => "Item_Cook_E_04",
            Self::Item_Cook_F_01 => "Item_Cook_F_01",
            Self::Item_Cook_F_02 => "Item_Cook_F_02",
            Self::Item_Cook_F_03 => "Item_Cook_F_03",
            Self::Item_Cook_F_04 => "Item_Cook_F_04",
            Self::Item_Cook_G_02 => "Item_Cook_G_02",
            Self::Item_Cook_G_03 => "Item_Cook_G_03",
            Self::Item_Cook_G_04 => "Item_Cook_G_04",
            Self::Item_Cook_G_05 => "Item_Cook_G_05",
            Self::Item_Cook_G_06 => "Item_Cook_G_06",
            Self::Item_Cook_G_09 => "Item_Cook_G_09",
            Self::Item_Cook_G_10 => "Item_Cook_G_10",
            Self::Item_Cook_G_11 => "Item_Cook_G_11",
            Self::Item_Cook_G_12 => "Item_Cook_G_12",
            Self::Item_Cook_G_13 => "Item_Cook_G_13",
            Self::Item_Cook_G_14 => "Item_Cook_G_14",
            Self::Item_Cook_G_15 => "Item_Cook_G_15",
            Self::Item_Cook_G_16 => "Item_Cook_G_16",
            Self::Item_Cook_G_17 => "Item_Cook_G_17",
            Self::Item_Cook_H_01 => "Item_Cook_H_01",
            Self::Item_Cook_H_02 => "Item_Cook_H_02",
            Self::Item_Cook_H_03 => "Item_Cook_H_03",
            Self::Item_Cook_I_01 => "Item_Cook_I_01",
            Self::Item_Cook_I_02 => "Item_Cook_I_02",
            Self::Item_Cook_I_03 => "Item_Cook_I_03",
            Self::Item_Cook_I_04 => "Item_Cook_I_04",
            Self::Item_Cook_I_05 => "Item_Cook_I_05",
            Self::Item_Cook_I_06 => "Item_Cook_I_06",
            Self::Item_Cook_I_07 => "Item_Cook_I_07",
            Self::Item_Cook_I_08 => "Item_Cook_I_08",
            Self::Item_Cook_I_09 => "Item_Cook_I_09",
            Self::Item_Cook_I_10 => "Item_Cook_I_10",
            Self::Item_Cook_I_11 => "Item_Cook_I_11",
            Self::Item_Cook_I_12 => "Item_Cook_I_12",
            Self::Item_Cook_I_13 => "Item_Cook_I_13",
            Self::Item_Cook_I_14 => "Item_Cook_I_14",
            Self::Item_Cook_I_15 => "Item_Cook_I_15",
            Self::Item_Cook_I_16 => "Item_Cook_I_16",
            Self::Item_Cook_I_17 => "Item_Cook_I_17",
            Self::Item_Cook_J_01 => "Item_Cook_J_01",
            Self::Item_Cook_J_02 => "Item_Cook_J_02",
            Self::Item_Cook_J_03 => "Item_Cook_J_03",
            Self::Item_Cook_J_04 => "Item_Cook_J_04",
            Self::Item_Cook_J_05 => "Item_Cook_J_05",
            Self::Item_Cook_J_06 => "Item_Cook_J_06",
            Self::Item_Cook_J_07 => "Item_Cook_J_07",
            Self::Item_Cook_J_08 => "Item_Cook_J_08",
            Self::Item_Cook_J_09 => "Item_Cook_J_09",
            Self::Item_Cook_K_01 => "Item_Cook_K_01",
            Self::Item_Cook_K_02 => "Item_Cook_K_02",
            Self::Item_Cook_K_03 => "Item_Cook_K_03",
            Self::Item_Cook_K_04 => "Item_Cook_K_04",
            Self::Item_Cook_K_05 => "Item_Cook_K_05",
            Self::Item_Cook_K_06 => "Item_Cook_K_06",
            Self::Item_Cook_K_07 => "Item_Cook_K_07",
            Self::Item_Cook_K_08 => "Item_Cook_K_08",
            Self::Item_Cook_K_09 => "Item_Cook_K_09",
            Self::Item_Cook_M_01 => "Item_Cook_M_01",
            Self::Item_Cook_N_01 => "Item_Cook_N_01",
            Self::Item_Cook_N_02 => "Item_Cook_N_02",
            Self::Item_Cook_N_03 => "Item_Cook_N_03",
            Self::Item_Cook_N_04 => "Item_Cook_N_04",
            Self::Item_Cook_O_01 => "Item_Cook_O_01",
            Self::Item_Cook_O_02 => "Item_Cook_O_02",
            Self::Item_Cook_P_01 => "Item_Cook_P_01",
            Self::Item_Cook_P_02 => "Item_Cook_P_02",
            Self::Item_Cook_P_03 => "Item_Cook_P_03",
            Self::Item_Cook_P_04 => "Item_Cook_P_04",
            Self::Item_Cook_P_05 => "Item_Cook_P_05",
        }
    }
}
impl std::fmt::Debug for CookItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(self.actor_name())
            .field(&self.name())
            .finish()
    }
}
impl std::fmt::Display for CookItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.actor_name())
    }
}
