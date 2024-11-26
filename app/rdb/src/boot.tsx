import i18next from "i18next";
import React from "react";
import ReactDOM from "react-dom/client";
import { initDark } from "@pistonite/pure/pref";

import type { Host } from "host/Host.ts";
import { initLocale } from "i18n/locales.ts";
import { store } from "store/store.ts";
import { updateSearchProgress } from "store/search.ts";
import { updateFilterProgress } from "store/filter.ts";
import { setResultLimit } from "store/result.ts";

import { AppWrapper } from "./AppWrapper.tsx";

/** Boot the app using the provided host */
export async function boot(host: Host) {
    initDark({
        persist: true,
    });

    await initLocale(host);

    ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
        <React.StrictMode>
            <AppWrapper host={host} />
        </React.StrictMode>,
    );
    // set title will show the window after initial render
    host.setTitle(i18next.t("title"));

    const searchProgressHandler = (percentage: number) => {
        store.dispatch(updateSearchProgress(percentage));
    };

    const filterProgressHandler = (percentage: number) => {
        store.dispatch(updateFilterProgress(percentage));
    };

    host.initialize();
    host.bind(searchProgressHandler, filterProgressHandler);
    host.getBinding()
        .getResultLimit()
        .then((limit) => {
            store.dispatch(setResultLimit(limit));
        });
}
