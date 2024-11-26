import type { PropsWithChildren } from "react";
import {
    FluentProvider,
    webDarkTheme,
    webLightTheme,
} from "@fluentui/react-components";
import {useDark} from "@pistonite/pure-react";

export const ThemeProvider: React.FC<PropsWithChildren> = ({ children }) => {
    const isDarkMode = useDark();
    return (
        <FluentProvider theme={isDarkMode ? webDarkTheme : webLightTheme}>
            {children}
        </FluentProvider>
    );
};
