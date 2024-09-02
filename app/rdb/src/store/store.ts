import { configureStore } from "@reduxjs/toolkit";
import { searchReducer } from "./search.ts";
import { filterReducer, getFavoriteActors, saveFavorites } from "./filter.ts";

export const store = configureStore({
    reducer: {
        search: searchReducer,
        filter: filterReducer,
    },
});

let lastFavorites = getFavoriteActors(store.getState());
store.subscribe(() => {
    const currentFavorites = getFavoriteActors(store.getState());
    if (currentFavorites !== lastFavorites) {
        lastFavorites = currentFavorites;
        saveFavorites(currentFavorites);
    }
});

export type State = ReturnType<typeof store.getState>;
export type Dispatch = typeof store.dispatch;
export type Store = typeof store;
