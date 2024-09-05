import {
    Button,
    makeStaticStyles,
    makeStyles,
    shorthands,
} from "@fluentui/react-components";

import { LocalePicker } from "components/LocalePicker.tsx";
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
        backgroundColor: "#fcfcfc",
        flex: 1,
    },
    mainScreen: {
        display: "flex",
        height: "100vh",
        backgroundColor: "#ccc",
        gap: "1px", // for border
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
                    className={styles.stageContainer}
                    style={{
                        minWidth: 356,
                        maxWidth: 440, // 42*10 + 20 = display 10 items
                    }}
                >
                    <FilterStage />
                </div>
                <div className={styles.stageContainer}>
                    <ResultStage />
                </div>
            </div>
            <div className={styles.corner}>
                <Button
                    as="a"
                    appearance="subtle"
                    icon={<img src="/github-mark.svg" />}
                    href="https://github.com/Pistonite/botw-recipe"
                    target="_blank"
                />
                <LocalePicker />
            </div>
        </>
    );
};
