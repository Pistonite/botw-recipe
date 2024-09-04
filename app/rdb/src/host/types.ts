// re-export types generated from rust
export * from "./types/CookError.ts";
export * from "./types/DatabaseError.ts";
import type { Error } from "./types/Error.ts";
export type HostError = Error;
export * from "./types/OptimizedRecipeData.ts";
export * from "./types/WeaponModifierSet.ts";
export * from "./types/SearchFilter.ts";
export * from "./types/Stats.ts";

export type HostProgressHandler = (percentage: number) => void;
