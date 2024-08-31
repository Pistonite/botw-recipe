import { Result, Void } from "@pistonite/pure/result";
import type { HostError, SearchComplete, SearchFilter } from "./types.ts";

/** Functions that binds to host functions */
export interface HostBinding {
    setTitle(title: string): Promise<void>;
    initialize(): Promise<Void<HostError>>;
    abort(handle: number): Promise<Void<HostError>>;
    search(filter: SearchFilter): Promise<Result<number[], HostError>>;
    // filterActors(filter: ActorFilter): Promise<Void<string>>;
    setInitializedHandler(handler: () => void): Promise<void>;
    setSearchCompleteHandler(handler: (result: Result<SearchComplete, HostError>) => void): Promise<void>;
    setSearchProgressHandler(handler: (percentage: number) => void): Promise<void>;
    // setFilterCompleteHandler(handler: (result: Result<FilterComplete, string>) => void): Promise<void>;
}
