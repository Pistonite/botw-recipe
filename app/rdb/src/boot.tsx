import React from "react";
import { Provider as ReduxProvider } from "react-redux";
import ReactDOM from "react-dom/client";
import { FluentProvider, webLightTheme } from "@fluentui/react-components";

import { App } from "./App";

import { AlertProvider } from "components/AlertProvider.tsx";
import { Host } from "host/Host.ts";
import { HostContext } from "host/useHost.ts";
import { initLocale } from "i18n/locales.ts";
import { store } from "store/store.ts";
import { updateSearchProgress } from "store/search.ts";

/** Boot the app using the provided host */
export async function boot(host: Host) {
    await initLocale();

    ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
        <React.StrictMode>
            <HostContext.Provider value={host}>
                <FluentProvider theme={webLightTheme}>
                    <AlertProvider>
                        <ReduxProvider store={store}>
                            <App />
                        </ReduxProvider>
                    </AlertProvider>
                </FluentProvider>
            </HostContext.Provider>
        </React.StrictMode>,
    );

    const searchProgressHandler = (percentage: number) => {
        store.dispatch(updateSearchProgress(percentage));
    };

    await host.bind(searchProgressHandler);
    await host.initialize();
}
