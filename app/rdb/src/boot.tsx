import React from "react";
import ReactDOM from "react-dom/client";

import type { Host } from "host/Host.ts";
import {
    initLocale,
    loadLocalePreference,
    switchLanguage,
} from "i18n/locales.ts";
import { setTranslationOverride } from "i18n/override.ts";
import { store } from "store/store.ts";
import { updateSearchProgress } from "store/search.ts";
import { updateFilterProgress } from "store/filter.ts";
import { setResultLimit } from "store/result.ts";

import { AppWrapper } from "./AppWrapper.tsx";

/** Boot the app using the provided host */
export async function boot(host: Host) {
    host.getBinding()
        .loadOverrideLocalizationJson()
        .then((json) => {
            if (json) {
                setTranslationOverride(() =>
                    host.getBinding().loadOverrideLocalizationJson(),
                );
                switchLanguage(loadLocalePreference(), host);
            }
        });
    await initLocale();

    ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
        <React.StrictMode>
            <AppWrapper host={host} />
        </React.StrictMode>,
    );

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
