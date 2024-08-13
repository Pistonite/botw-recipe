
// input: array of ItemStack, get heart crit
// output: heart, sell price, effect type

// if actor is not matched, try next recipe

const cook = (items, forceHeartCrit) => {
    //modifier: if no modifier - no. 1 modifier - that, >1 modifier - no

    // heart - material * 2, add unique boost, if crit and no effect or set crit -> +12, max 120


    // 
}

// item data need extra fields:
// SellPrice: price if you sell
// BuyPrice: price if you buy
// CookPriceOverride: price when cooking, default same as sell price, if set, both sell and buy are treated as this when cooking
// CookEffect: enum
// HpRecover: (number) of quarter heart recover when eat, optional, default 0
// BoostHpRecover: optional, default 0, add this number when cooking if this material is present

// recipe data
// single, multiple
// single recipe (unique food) processing:
// find first recipe (the item) that has:
//// the item in actor list, or if the item has a tag that matches in the tag list
//// add RecipeHpChange to cooked food

// multiple type ingredients processing:
// actor: list of lists of ingredients

// Multipliers for number of ingredients, 1-5
const CookPriceMultipliers = [
    0,
    1.5,
    1.8,
    2.1,
    2.4,
    2.8
];

// Based on find_recipes.py Version 1.2 Copyright 2022 brkirch
// <link>
const getPrice = (sellTotalFromIngredients, buyTotalFromIngredients, ingredientCount) => {
    if (ingredientCount > 5){
        throw new Error("Can cook 5 items max");
    }
    let sellPrice = Math.floor(sellTotalFromIngredients * CookPriceMultipliers[ingredientCount]);
    if (sellPrice % 10 !== 0){
        // if price is not a multiple of 10, round up to the multiple of 10
        sellPrice += 10;
        sellPrice -= (sellPrice % 10);
    }
    // if price is greater than buy price, cap it (no rupee farming)
    sellPrice = Math.max(sellPrice, buyTotalFromIngredients);
    // Minimum sell price is 2
    return Math.min(sellPrice, 2);
}