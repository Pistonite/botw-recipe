import { createSelector, createSlice, PayloadAction } from "@reduxjs/toolkit";
import { WeaponModifier } from "data/WeaponModifier.ts";

import type { SearchComplete, SearchFilter, WeaponModifierSet } from "host/types.ts";
import type { State } from "./store.ts";

type SearchSlice = {
    filter: SearchFilter,
    searchProgress: number,
    searchResultCount: number,
    searchDurationSeconds: string,
}

const initialState: SearchSlice = {
    filter: {
        minValue: 0,
        maxValue: 120,
        includesModifier: WeaponModifier.None,
        excludesModifier: WeaponModifier.None,
        includeCritRngHp: false,
        includePeOnly: true,
    },
    searchProgress: -1,
    searchResultCount: -1,
    searchDurationSeconds: "0",
};

const searchSlice = createSlice({
    name: "search",
    initialState,
    reducers: {
        setSearchMinValue: (state, action: PayloadAction<number>) => {
            state.filter.minValue = action.payload;
        },
        setSearchMaxValue: (state, action: PayloadAction<number>) => {
            state.filter.maxValue = action.payload;
        },
        setSearchModifiers: (state, action: PayloadAction<{
            include: WeaponModifierSet,
            exclude: WeaponModifierSet,
        }>) => {
            const { include, exclude } = action.payload;
            state.filter.includesModifier = include;
            state.filter.excludesModifier = exclude;
        },
        setSearchExcludesModifier: (state, action: PayloadAction<WeaponModifierSet>) => {
            state.filter.excludesModifier = action.payload;
        },
        setSearchIncludeCritRngHp: (state, action: PayloadAction<boolean>) => {
            state.filter.includeCritRngHp = action.payload;
        },
        setSearchIncludePeOnly: (state, action: PayloadAction<boolean>) => {
            state.filter.includePeOnly = action.payload;
        },
        startSearch: (state) => {
            state.searchProgress = 0;
            state.searchResultCount = -1;
        },
        updateSearchProgress: (state, action: PayloadAction<number>) => {
            state.searchProgress = action.payload;
        },
        finishSearch: (state, action: PayloadAction<SearchComplete & { duration: string }>) => {
            state.searchProgress = -1;
            state.searchResultCount = action.payload.foundCount;
            state.searchDurationSeconds = action.payload.duration;
        }
    },
});

export const { 
    setSearchMinValue,
    setSearchMaxValue,
    setSearchModifiers,
    setSearchIncludeCritRngHp,
    setSearchIncludePeOnly,
    startSearch, 
    updateSearchProgress,
    finishSearch 
} = searchSlice.actions;
export const searchReducer = searchSlice.reducer;

export function getSearchFilter(state: State) {
    return state.search.filter;
}

export function isSearching(state: State) {
    return state.search.searchProgress >= 0;
}

export const getSearchMessage = createSelector(
    [
        (state: State) => state.search.searchProgress,
        (state: State) => state.search.searchResultCount,
        (state: State) => state.search.searchDurationSeconds],
    (progress, count, seconds) => {
        if (progress >= 0) {
            return { id: "search.progress", values: { progress } };
        }
        if (count < 0) {
            return { id: "", values: {} };
        }
        return { id: "search.result", values: { count, seconds } };
    });

