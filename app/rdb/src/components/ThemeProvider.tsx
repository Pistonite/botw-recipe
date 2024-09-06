import {
    createContext,
    useContext,
    useEffect,
    useMemo,
    useState,
    type PropsWithChildren,
} from "react";
import {
    FluentProvider,
    webDarkTheme,
    webLightTheme,
} from "@fluentui/react-components";

type ThemeContextType = {
    isDarkMode: boolean;
    setIsDarkMode: (isDarkMode: boolean) => void;
};

const ThemeContext = createContext<ThemeContextType>({
    isDarkMode: false,
    setIsDarkMode: () => {
        /*stub*/
    },
});

function isDarkModePreferred(): boolean {
    return !!window.matchMedia?.("(prefers-color-scheme: dark)").matches;
}

export const ThemeProvider: React.FC<PropsWithChildren> = ({ children }) => {
    const [isDarkMode, setIsDarkMode] = useState(isDarkModePreferred);
    // need this to override the browser's dark mode preference
    useEffect(() => {
        let styles = document.getElementById("root-theme");
        if (!styles) {
            styles = document.createElement("style");
            styles.id = "root-theme";
            document.head.appendChild(styles);
        }
        styles.innerText = `:root { color-scheme: ${isDarkMode ? "dark" : "light"}`;
    }, [isDarkMode]);
    const memoState = useMemo(
        () => ({ isDarkMode, setIsDarkMode }),
        [isDarkMode, setIsDarkMode],
    );
    return (
        <ThemeContext.Provider value={memoState}>
            <FluentProvider theme={isDarkMode ? webDarkTheme : webLightTheme}>
                {children}
            </FluentProvider>
        </ThemeContext.Provider>
    );
};

export const useTheme = () => useContext(ThemeContext);
