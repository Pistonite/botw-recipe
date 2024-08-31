// re-export types generated from rust
export * from "./types/CookError.ts";
export * from "./types/DatabaseError.ts";
import { Error } from "./types/Error.ts";
export type HostError = Error;
export * from "./types/SearchFilter.ts";
export * from "./types/WeaponModifierSet.ts";
export * from "./types/SearchComplete.ts";

export type HostSearchProgressHandler = (percentage: number) => void;
