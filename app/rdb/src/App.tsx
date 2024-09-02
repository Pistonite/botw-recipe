import {
    makeStaticStyles,
    makeStyles,
    shorthands,
} from "@fluentui/react-components";
import { List24Regular } from "@fluentui/react-icons";

import { LocalePicker } from "components/LocalePicker.tsx";
import { SearchStage } from "stage/SearchStage.tsx";
import { FilterStage } from "stage/FilterStage.tsx";
import { StageTitle } from "components/StageTitle";

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
    },
    mainScreen: {
        display: "flex",
        height: "100vh",
        backgroundColor: "#ccc",
        gap: "1px", // for border
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
                        minWidth: 440,
                        width: 440,
                        maxWidth: 440, // 42*10 + 20 = display 10 items
                    }}
                >
                    <FilterStage />
                </div>
                <div className={styles.stageContainer} style={{ flex: 1 }}>
                    <StageTitle
                        title="Results"
                        icon={List24Regular}
                        desc="Recipes here should both have the desired modifiers, and only include the desired items"
                    />
                </div>
            </div>
            <div style={{ position: "fixed", right: 10, top: 10 }}>
                <LocalePicker />
            </div>
        </>
    );
};
