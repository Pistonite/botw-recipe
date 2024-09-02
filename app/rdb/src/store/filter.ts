import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import { Actor, ActorToGroup, getActors } from "data/Actor.ts";
import { Stats } from "host/types.ts";

type FilterSlice = {
    /** The included actors for what's currently displayed */
    currentIncluded: Actor[];
    /** The percentage usage of each actor. (num of recipes with actor/total recipe) */
    actorUsagePercentages: string[] | null;
    /** The actors that are included in the next search */
    nextIncluded: Actor[];
    favorited: Actor[];
    filterProgress: number;
    filterResultCount: number;
    filterDurationSeconds: string;
    /** Whether the filter was executed as part of search */
    isFromSearch: boolean;
};

const initialState: FilterSlice = {
    currentIncluded: [],
    actorUsagePercentages: null,
    nextIncluded: [],
    favorited: [],
    filterProgress: -1,
    filterResultCount: -1,
    filterDurationSeconds: "0",
    isFromSearch: false,
};

const filterSlice = createSlice({
    name: "filter",
    initialState,
    reducers: {
        resetFilter: (state) => {
            const nextCurrentIncluded = getActors().filter(
                (actor) => actor !== Actor.None,
            );
            state.currentIncluded = nextCurrentIncluded;
            state.nextIncluded = nextCurrentIncluded;
            state.actorUsagePercentages = null;
        },
        startFilter: (state) => {
            state.filterProgress = 0;
            state.filterResultCount = -1;
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
            if (groupStat) {
                nextCurrentIncluded = getActors().filter((actor) => {
                    if (actor === Actor.None) {
                        return false;
                    }
                    const group = ActorToGroup[actor];
                    return groupStat[group];
                });
                if (stats.foundCount === 0) {
                    state.actorUsagePercentages = null;
                } else {
                    state.actorUsagePercentages = getActors().map((actor) => {
                        if (actor === Actor.None) {
                            return "";
                        }
                        const group = ActorToGroup[actor];
                        if (!groupStat[group]) {
                            return "";
                        }
                        const percentage = (
                            (groupStat[group] / stats.foundCount) *
                            100
                        ).toFixed(2);
                        return percentage;
                    });
                }
            } else {
                nextCurrentIncluded = getActors().filter(
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

export const filterReducer = filterSlice.reducer;
