/** Filter for searching the database */
export type SearchFilter = {
     minValue: number,
     maxValue: number,
     includesModifier: number,
     excludesModifier: number,
     includeCritRngHp: boolean,
     includePeOnly: boolean,
}

/** Response from search-complete event */
export type SearchComplete = {
    foundCount: number,
    actors: number[],
}

export type HostErrorHandler = (error: unknown) => void;
export type HostSearchProgressHandler = (percentage: number) => void;
