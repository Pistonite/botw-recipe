import { PropsWithChildren } from "react";
import { makeStyles } from "@fluentui/react-components";

import { StageDivider } from "./StageDivider.tsx";

const useStyles = makeStyles({
    actions: {
        display: "flex",
        gap: "10px",
        justifyContent: "end",
        alignItems: "baseline",
    }
});

/** Component at the end of a stage for actions */
export const StageAction: React.FC<PropsWithChildren> = ({children}) => {
    const styles = useStyles();

    return (
        <div>
            <StageDivider />
            <div className={styles.actions}>
                {children}
            </div>
        </div>
    );
}
