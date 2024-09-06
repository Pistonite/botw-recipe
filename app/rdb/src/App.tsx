import {
    Button,
    makeStaticStyles,
    makeStyles,
    mergeClasses,
    shorthands,
} from "@fluentui/react-components";
import {
    WeatherMoon20Regular,
    WeatherSunny20Regular,
} from "@fluentui/react-icons";

import { LocalePicker } from "components/LocalePicker.tsx";
import { useTheme } from "components/ThemeProvider.tsx";
import { SearchStage } from "stage/SearchStage.tsx";
import { FilterStage } from "stage/FilterStage.tsx";
import { ResultStage } from "stage/ResultStage.tsx";

const useStaticStyles = makeStaticStyles({
    "*": {
        minWidth: 0,
        boxSizing: "border-box",
    },
    ":root": {
        fontSynthesis: "none",
        textRendering: "optimizeLegibility",
        WebkitFontSmoothing: "antialiased",
        MozOsxFontSmoothing: "grayscale",
        WebkitTextSizeAdjust: "100%",
    },
    body: {
        ...shorthands.margin(0),
        ...shorthands.padding(0),
        overflow: "hidden",
    },
});

const useStyles = makeStyles({
    stageContainer: {
        display: "flex",
        flexDirection: "column",
        height: "100%",
        ...shorthands.padding("10px"),
        flex: 1,
    },
    stageContainerDark: {
        borderLeft: "1px solid #888",
    },
    stageContainerLight: {
        borderLeft: "1px solid #ccc",
    },
    mainScreen: {
        display: "flex",
        height: "100vh",
    },
    corner: {
        position: "fixed",
        right: "10px",
        top: "10px",
    },
});

export const App: React.FC = () => {
    useStaticStyles();
    const styles = useStyles();
    const { isDarkMode, setIsDarkMode } = useTheme();

    const stageContainerClass = mergeClasses(
        styles.stageContainer,
        isDarkMode ? styles.stageContainerDark : styles.stageContainerLight,
    );

    return (
        <>
            <div className={styles.mainScreen}>
                <div
                    className={styles.stageContainer}
                    style={{ minWidth: 300, maxWidth: 360 }}
                >
                    <SearchStage />
                </div>
                <div
                    className={stageContainerClass}
                    style={{
                        minWidth: 356,
                        maxWidth: 440, // 42*10 + 20 = display 10 items
                    }}
                >
                    <FilterStage />
                </div>
                <div className={stageContainerClass}>
                    <ResultStage />
                </div>
            </div>
            <div className={styles.corner}>
                <Button
                    appearance="subtle"
                    icon={
                        isDarkMode ? (
                            <WeatherSunny20Regular />
                        ) : (
                            <WeatherMoon20Regular />
                        )
                    }
                    onClick={() => setIsDarkMode(!isDarkMode)}
                />
                <Button
                    as="a"
                    appearance="subtle"
                    icon={
                        <img
                            src={
                                isDarkMode
                                    ? "/github-mark-white.svg"
                                    : "/github-mark.svg"
                            }
                        />
                    }
                    href="https://github.com/Pistonite/botw-recipe"
                    target="_blank"
                />
                <LocalePicker />
            </div>
        </>
    );
};
