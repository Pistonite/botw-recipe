import type { Result } from "@pistonite/pure/result";
import { createSlice, type PayloadAction } from "@reduxjs/toolkit";
import type {
    HostError,
    OptimizedRecipeData,
    SearchFilter,
} from "botw-recipe-searcher-tauri";

import { type ResultData, toResultData } from "util/ResultData.ts";
import { WeaponModifier } from "util/WeaponModifier.ts";

import type { State } from "./store.ts";

type ResultSlice = {
    /** The filter used to produce the results */
    filter: SearchFilter;
    /** Error that occured when cooking the results */
    error: HostError | undefined;
    /** Cooked results */
    data: ResultData[];
    isInProgress: boolean;
    /** Limit on how many results are computed */
    limit: number;
};

const initialState: ResultSlice = {
    filter: {
        minValue: 0,
        maxValue: 120,
        includesModifier: WeaponModifier.None,
        excludesModifier: WeaponModifier.None,
        includeCritRngHp: false,
        includePeOnly: true,
    },
    error: undefined,
    data: [],
    isInProgress: false,
    limit: 0,
};

const resultSlice = createSlice({
    name: "result",
    initialState,
    reducers: {
        startCooking: (state) => {
            state.isInProgress = true;
            state.error = undefined;
            // doesn't need to clear previous result
        },
        finishCooking: (
            state,
            action: PayloadAction<Result<OptimizedRecipeData[], HostError>>,
        ) => {
            state.isInProgress = false;
            const result = action.payload;
            if (result.err) {
                state.error = result.err;
                state.data = [];
            } else {
                state.error = undefined;
                state.data = result.val.map(toResultData);
            }
        },
        setResultFilter: (state, action: PayloadAction<SearchFilter>) => {
            state.filter = action.payload;
        },
        setResultLimit: (state, action: PayloadAction<number>) => {
            state.limit = action.payload;
        },
    },
});

export const { startCooking, finishCooking, setResultFilter, setResultLimit } =
    resultSlice.actions;

export const resultReducer = resultSlice.reducer;

export const getCookingResults = (state: State) => {
    return state.result.data;
};

export const isResultCookingInProgress = (state: State) => {
    return state.result.isInProgress;
};

export const getResultCookingError = (state: State) => {
    if (state.result.isInProgress) {
        return undefined;
    }
    return state.result.error;
};

export const getResultFilter = (state: State) => {
    return state.result.filter;
};

export const getResultLimit = (state: State) => {
    return state.result.limit;
};
