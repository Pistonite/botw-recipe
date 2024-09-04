import {
    createSelector,
    createSlice,
    type PayloadAction,
} from "@reduxjs/toolkit";

import { Actor, ActorToGroup, getActors } from "data/Actor.ts";
import type { Stats } from "host/types.ts";

import type { State } from "./store.ts";

function loadFavorites() {
    const favorites = localStorage.getItem("favorites");
    if (favorites) {
        try {
            const data = JSON.parse(favorites);
            if (Array.isArray(data)) {
                const set = new Set(data);
                return getActors().filter((actor) => set.has(actor));
            }
        } catch {
            //ignore
        }
    }
    return [];
}

export function saveFavorites(favorites: Actor[]) {
    const data = JSON.stringify(favorites);
    localStorage.setItem("favorites", data);
}

type FilterSlice = {
    /** The included actors for what's currently displayed */
    currentIncluded: Actor[];
    /** The percentage usage of each actor. (num of recipes with actor/total recipe) */
    actorUsagePercentages: number[] | null;
    /** The actors that are included in the next search */
    nextIncluded: Actor[];
    favorited: Actor[];
    /** -1 = not started/done, 0-100 = in progress*/
    filterProgress: number;
    /** 1 = never started, >= 0 = number of results */
    filterResultCount: number;
    /** Duration of the last filter */
    filterDurationSeconds: string;
    /** Whether the filter was executed as part of search */
    isFromSearch: boolean;
};

const initialState: FilterSlice = {
    currentIncluded: [],
    actorUsagePercentages: null,
    nextIncluded: [],
    favorited: loadFavorites(),
    filterProgress: -1,
    filterResultCount: -1,
    filterDurationSeconds: "0",
    isFromSearch: false,
};

const filterSlice = createSlice({
    name: "filter",
    initialState,
    reducers: {
        clearFavorites: (state) => {
            state.favorited = [];
        },
        toggleFavoriteActor: (state, action: PayloadAction<Actor>) => {
            const actor = action.payload;
            if (!state.favorited.includes(actor)) {
                state.favorited.push(actor);
            } else {
                // perf: new array needs to be created anyway
                // so there's no point using constant time remove here
                state.favorited = state.favorited.filter((a) => a !== actor);
            }
        },
        toggleIncludedActor: (state, action: PayloadAction<Actor>) => {
            const actor = action.payload;
            if (!state.nextIncluded.includes(actor)) {
                state.nextIncluded.push(actor);
            } else {
                state.nextIncluded = state.nextIncluded.filter(
                    (a) => a !== actor,
                );
            }
        },
        resetFilter: (state) => {
            const nextCurrentIncluded = getActors().filter(
                (actor) => actor !== Actor.None,
            );
            state.currentIncluded = nextCurrentIncluded;
            state.nextIncluded = nextCurrentIncluded;
            state.actorUsagePercentages = null;
            state.filterResultCount = -1;
        },
        startFilter: (state) => {
            state.filterProgress = 0;
            // doesn't need to clear the result
            // if filter is aborted, previous result is still valid
        },
        abortFilter: (state) => {
            state.filterProgress = -1;
        },
        updateFilterProgress: (state, action: PayloadAction<number>) => {
            if (state.filterProgress >= 0) {
                state.filterProgress = action.payload;
            }
        },
        finishFilter: (
            state,
            action: PayloadAction<
                Stats & { duration: string; isFromSearch: boolean }
            >,
        ) => {
            const { duration, isFromSearch, ...stats } = action.payload;
            let nextCurrentIncluded;
            const groupStat = stats.groupStat;
            // using nextIncluded to filter
            // to avoid adding excluded actors from an included group
            if (groupStat) {
                nextCurrentIncluded = state.nextIncluded.filter((actor) => {
                    if (actor === Actor.None) {
                        return false;
                    }
                    const group = ActorToGroup[actor];
                    return groupStat[group];
                });

                if (stats.foundCount === 0) {
                    state.actorUsagePercentages = null;
                } else {
                    const set = new Set(nextCurrentIncluded);
                    state.actorUsagePercentages = getActors().map((actor) => {
                        if (actor === Actor.None || !set.has(actor)) {
                            return 0;
                        }
                        const group = ActorToGroup[actor];
                        if (!groupStat[group]) {
                            return 0;
                        }
                        const percentage =
                            (groupStat[group] / stats.foundCount) * 100;
                        return percentage;
                    });
                }
            } else {
                nextCurrentIncluded = state.nextIncluded.filter(
                    (actor) => actor !== Actor.None,
                );
                state.actorUsagePercentages = null;
            }
            state.currentIncluded = nextCurrentIncluded;
            state.nextIncluded = nextCurrentIncluded;
            state.filterProgress = -1;
            state.filterResultCount = stats.foundCount;
            state.filterDurationSeconds = duration;
            state.isFromSearch = isFromSearch;
        },
    },
});

export const {
    clearFavorites,
    abortFilter,
    toggleIncludedActor,
    toggleFavoriteActor,
    resetFilter,
    startFilter,
    updateFilterProgress,
    finishFilter,
} = filterSlice.actions;
export const filterReducer = filterSlice.reducer;

export const isFilterInProgress = (state: State) => {
    return state.filter.filterProgress >= 0;
};

export const getFavoriteActors = (state: State) => {
    return state.filter.favorited;
};

export const getIncludedActors = (state: State) => {
    return state.filter.nextIncluded;
};

export const getActorPercentages = (state: State) => {
    return state.filter.actorUsagePercentages;
};

export const getActorSubtitles = createSelector(
    [
        (state: State) => state.filter.currentIncluded,
        (state: State) => state.filter.nextIncluded,
        (state: State) => state.filter.actorUsagePercentages,
    ],
    (currentIncluded, nextIncluded, actorUsagePercentages) => {
        const currentIncludedSet = new Set(currentIncluded);
        const nextIncludedSet = new Set(nextIncluded);
        return getActors().map((actor) => {
            if (actor === Actor.None) {
                return { id: "", values: {} };
            }
            const prefix = "filter.selection.subtitle";
            let isCurrentExcluded = !currentIncludedSet.has(actor);
            if (actorUsagePercentages) {
                if (!actorUsagePercentages[actor]) {
                    isCurrentExcluded = true;
                }
            }
            const isNextIncluded = nextIncludedSet.has(actor);
            if (isCurrentExcluded) {
                if (isNextIncluded) {
                    return { id: `${prefix}.excluded.to_include`, values: {} };
                }
                return { id: `${prefix}.excluded`, values: {} };
            }
            if (actorUsagePercentages) {
                const percentage = getStringPercentage(
                    actorUsagePercentages[actor],
                );
                if (isNextIncluded) {
                    return {
                        id: `${prefix}.percentage`,
                        values: { percentage },
                    };
                }
                return {
                    id: `${prefix}.percentage.to_exclude`,
                    values: { percentage },
                };
            }
            if (isNextIncluded) {
                return { id: `${prefix}.no_percentage`, values: {} };
            }
            return { id: `${prefix}.no_percentage.to_exclude`, values: {} };
        });
    },
);

function getStringPercentage(percentage: number) {
    let digits = 2;
    let result = percentage.toFixed(digits);
    while (!parseFloat(result) && digits < 15) {
        digits++;
        result = percentage.toFixed(digits);
    }
    return result;
}

export const getFilterResultCount = (state: State) => {
    return state.filter.filterResultCount;
};

export const getFilterMessage = createSelector(
    [
        (state: State) => state.filter.filterProgress,
        (state: State) => state.filter.filterResultCount,
        (state: State) => state.filter.filterDurationSeconds,
        (state: State) => state.filter.isFromSearch,
    ],
    (progress, count, seconds, isFromSearch) => {
        if (progress == 0) {
            return { id: "filter.progress.initial", values: {} };
        }
        if (progress >= 0) {
            return { id: "filter.progress", values: { progress } };
        }
        if (count < 0 || isFromSearch) {
            return { id: "filter.prompt", values: {} };
        }
        return { id: "filter.result", values: { count, seconds } };
    },
);
