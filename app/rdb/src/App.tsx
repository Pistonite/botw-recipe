import {
    Button,
    makeStaticStyles,
    makeStyles,
    mergeClasses,
} from "@fluentui/react-components";
import {
    WeatherMoon20Regular,
    WeatherSunny20Regular,
} from "@fluentui/react-icons";

import { LocalePicker } from "components/LocalePicker.tsx";
import { SearchStage } from "stage/SearchStage.tsx";
import { FilterStage } from "stage/FilterStage.tsx";
import { ResultStage } from "stage/ResultStage.tsx";
import { useDark } from "@pistonite/pure-react";
import { setDark } from "@pistonite/pure/pref";

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
        margin: 0,
        padding: 0,
        overflow: "hidden",
    },
});

const useStyles = makeStyles({
    stageContainer: {
        display: "flex",
        flexDirection: "column",
        height: "100%",
        padding: "10px",
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
    const isDarkMode = useDark();

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
                    onClick={() => setDark(!isDarkMode)}
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
