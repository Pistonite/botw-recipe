import { Result, Void } from "@pistonite/pure/result";
import type { SearchComplete, SearchFilter } from "./types.ts";

/** Functions that binds to host functions */
export interface HostBinding {
    setTitle(title: string): Promise<void>;
    initialize(): Promise<Void<string>>;
    abort(handle: number): Promise<Void<string>>;
    search(filter: SearchFilter): Promise<Result<number[], string>>;
    // filterActors(filter: ActorFilter): Promise<Void<string>>;
    setInitializedHandler(handler: () => void): Promise<void>;
    setSearchCompleteHandler(handler: (result: Result<SearchComplete, string>) => void): Promise<void>;
    setSearchProgressHandler(handler: (percentage: number) => void): Promise<void>;
    // setFilterCompleteHandler(handler: (result: Result<FilterComplete, string>) => void): Promise<void>;
}
