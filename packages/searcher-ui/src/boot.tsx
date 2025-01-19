import { initDark } from "@pistonite/pure/pref";
import React from "react";
import ReactDOM from "react-dom/client";
import { Host } from "botw-recipe-searcher-tauri";
import { initI18n, translate } from "botw-recipe-searcher-localization";

import { store } from "./store/store.ts";
import { updateFilterProgress } from "./store/filter.ts";
import { updateSearchProgress } from "./store/search.ts";
import { setResultLimit } from "./store/result.ts";

import { AppWrapper } from "./AppWrapper.tsx";

/** Boot the UI using the provided host and mount to the provided DOM node */
export async function boot(host: Host, domRoot: HTMLElement) {
    initDark({ persist: true });

    await initI18n(host);

    ReactDOM.createRoot(domRoot).render(
        <React.StrictMode>
            <AppWrapper host={host} />
        </React.StrictMode>,
    );
    // set title will show the window after initial render
    host.setTitle(translate("title"));

    const searchProgressHandler = (percentage: number) => {
        store.dispatch(updateSearchProgress(percentage));
    };

    const filterProgressHandler = (percentage: number) => {
        store.dispatch(updateFilterProgress(percentage));
    };

    host.initialize();
    host.bind(searchProgressHandler, filterProgressHandler);
    host.getResultLimit()
        .then((limit) => {
            store.dispatch(setResultLimit(limit));
        });
}
