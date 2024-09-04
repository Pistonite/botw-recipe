import type { OptimizedRecipeData, WeaponModifierSet } from "host/types.ts";

import type { Actor } from "./Actor.ts";

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
