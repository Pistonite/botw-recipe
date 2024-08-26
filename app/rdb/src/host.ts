import { Result, Void } from "@pistonite/pure/result";

export interface Host {
    bind(): Promise<void>;
    initialize(arg: InitArg): Promise<Void<string>>;
    search(filter: SearchFilter): Promise<Result<SearchComplete, string>>;
    cancelSearch(): Promise<Void<string>>;
    filterActors(filter: ActorFilter): Promise<Result<FilterComplete, string>>;

}

export interface HostBinding {
    initialize(arg: InitArg): Promise<Void<string>>;
    abort(handle: number): Promise<Void<string>>;
    search(filter: SearchFilter): Promise<Result<number[], string>>;
    filterActors(filter: ActorFilter): Promise<Void<string>>;
    setInitializedHandler(handler: () => void): Promise<void>;
    setSearchCompleteHandler(handler: (result: Result<SearchComplete, string>) => void): Promise<void>;
    setFilterCompleteHandler(handler: (result: Result<FilterComplete, string>) => void): Promise<void>;
}

export type InitArg = {
    title: string;
};

export type SearchFilter = {
     minValue: number,
     maxValue: number,
     includesModifier: number,
     excludesModifier: number,
     includeCritRngHp: boolean,
     includePeOnly: boolean,
}

export type SearchComplete = {
    resultCount: number,
    actors: number[],
}

export type ActorFilter = number[];

export type RecipeInfo  ={
    groups: number[],
    value: number,
    price: number
}

export type FilterComplete = {
    results: RecipeInfo[]
}