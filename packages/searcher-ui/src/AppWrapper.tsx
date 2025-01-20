import { Provider as ReduxProvider } from "react-redux";
import type { Host } from "botw-recipe-searcher-tauri";

import { ThemeProvider } from "./components/ThemeProvider.tsx";
import { AlertProvider } from "./components/AlertProvider.tsx";
import { HostContext } from "./host/HostContext.ts";
import { store } from "./store/store.ts";

import { App } from "./App.tsx";

type AppWrapperProps = {
    host: Host;
};

export const AppWrapper: React.FC<AppWrapperProps> = ({ host }) => {
    return (
        <HostContext.Provider value={host}>
            <ThemeProvider>
                <AlertProvider>
                    <ReduxProvider store={store}>
                        <App />
                    </ReduxProvider>
                </AlertProvider>
            </ThemeProvider>
        </HostContext.Provider>
    );
};
