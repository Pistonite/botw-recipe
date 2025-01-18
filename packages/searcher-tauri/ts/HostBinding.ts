import type { Result, Void } from "@pistonite/pure/result";

import type { Group } from "data/Group.ts";
import type {
    HostError,
    Stats,
    SearchFilter,
    OptimizedRecipeData,
} from "./types.ts";

/** Functions that binds to host functions */
export interface HostBinding {
    setTitle(title: string): Promise<void>;
    initialize(): Promise<Void<HostError>>;
    setInitializedHandler(handler: () => void): Promise<void>;
    search(filter: SearchFilter): Promise<Void<HostError>>;
    abortSearch(): Promise<Void<HostError>>;
    setSearchCompleteHandler(
        handler: (result: Result<Stats, HostError>) => void,
    ): Promise<void>;
    setSearchProgressHandler(
        handler: (percentage: number) => void,
    ): Promise<void>;
    filter(filter: Group[]): Promise<Void<HostError>>;
    abortFilter(): Promise<Void<HostError>>;
    setFilterCompleteHandler(
        handler: (result: Result<Stats, HostError>) => void,
    ): Promise<void>;
    setFilterProgressHandler(
        handler: (percentage: number) => void,
    ): Promise<void>;
    cook(): Promise<Void<HostError>>;
    setCookCompleteHandler(
        handler: (result: Result<OptimizedRecipeData[], HostError>) => void,
    ): Promise<void>;
    loadOverrideLocalizationJson(): Promise<string>;
    getResultLimit(): Promise<number>;
}
