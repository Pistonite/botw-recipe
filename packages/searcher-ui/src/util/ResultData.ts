import type { OptimizedRecipeData, WeaponModifierSet } from "botw-recipe-searcher-tauri";
import type { Actor } from "botw-recipe-sys";

export type ResultData = {
    actors: Actor[][];
    value: number;
    modifiers: WeaponModifierSet;
    isHearty: boolean;
};

export function toResultData(data: OptimizedRecipeData): ResultData {
    return {
        actors: data.actors as Actor[][],
        value: data.value,
        modifiers: data.price,
        isHearty: data.isHearty,
    };
}
