import { Provider as ReduxProvider } from "react-redux";

import { AlertProvider } from "components/AlertProvider.tsx";
import type { Host } from "host/Host.ts";
import { HostContext } from "host/useHost.ts";
import { store } from "store/store.ts";

import { App } from "./App.tsx";
import { ThemeProvider } from "components/ThemeProvider.tsx";

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
