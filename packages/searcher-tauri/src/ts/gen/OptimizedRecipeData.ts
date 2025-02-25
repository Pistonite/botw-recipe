// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.

/**
 * Result of grouping/merging recipes with the same values
 */
export type OptimizedRecipeData = {
    /**
     * Actors for this recipe
     *
     * Each Vec is a group of actors
     */
    actors: Array<Array<number>>;
    value: number;
    isHearty: boolean;
    price: number;
};
