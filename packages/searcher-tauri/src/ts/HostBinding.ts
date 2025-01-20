import type { Result, Void } from "@pistonite/pure/result";

import type { Group } from "botw-recipe-sys";
import type {
    HostError,
    Stats,
    SearchFilter,
    OptimizedRecipeData,
} from "./types.ts";

/** Functions that binds to host functions */
export interface HostBinding {
    /** Set the localized title of the application */
    setTitle(title: string): Promise<void>;

    /** Start async initialization of the host */
    initialize(): Promise<Void<HostError>>;

    /** Set the callback for when the host is initialized */
    setInitializedHandler(handler: () => void): Promise<void>;

    /** Start recipe search stage */
    search(filter: SearchFilter): Promise<Void<HostError>>;
    /** Abort the current active search */
    abortSearch(): Promise<Void<HostError>>;
    /** Set the callback for when the search is complete */
    setSearchCompleteHandler(
        handler: (result: Result<Stats, HostError>) => void,
    ): Promise<void>;
    /** Set the callback to receive progress updates during the search */
    setSearchProgressHandler(
        handler: (percentage: number) => void,
    ): Promise<void>;

    /** Start recipe filter stage */
    filter(filter: Group[]): Promise<Void<HostError>>;
    /** Abort the current active filter */
    abortFilter(): Promise<Void<HostError>>;
    /** Set the callback for when the filter is complete */
    setFilterCompleteHandler(
        handler: (result: Result<Stats, HostError>) => void,
    ): Promise<void>;
    /** Set the callback to receive progress updates during the filter */
    setFilterProgressHandler(
        handler: (percentage: number) => void,
    ): Promise<void>;

    /** Start cooking stage */
    cook(): Promise<Void<HostError>>;
    /** Set the callback for when the cooking is complete */
    setCookCompleteHandler(
        handler: (result: Result<OptimizedRecipeData[], HostError>) => void,
    ): Promise<void>;

    /** Get the maximum number of recipes to show in the result */
    getResultLimit(): Promise<number>;
}
