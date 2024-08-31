import { configureStore } from "@reduxjs/toolkit";
import { searchReducer } from "./search.ts";

export const store = configureStore({
    reducer: {
        search: searchReducer,
    }
});

export type State = ReturnType<typeof store.getState>;
export type Dispatch = typeof store.dispatch;
export type Store = typeof store;
