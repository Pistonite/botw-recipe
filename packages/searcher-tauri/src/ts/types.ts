// re-export types generated from rust
export * from "./gen/DatabaseError.ts";
import type { Error } from "./gen/Error.ts";
export type HostError = Error;
export * from "./gen/OptimizedRecipeData.ts";
export * from "./gen/WeaponModifierSet.ts";
export * from "./gen/SearchFilter.ts";
export * from "./gen/Stats.ts";

export type HostProgressHandler = (percentage: number) => void;

/** Create a generic error */
export const unexpected = (x: string): HostError => ({
    type: "Unexpected",
    data: x,
});
