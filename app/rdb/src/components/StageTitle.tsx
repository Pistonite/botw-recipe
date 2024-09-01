import { memo } from "react";
import { Body1, makeStyles, Title3 } from "@fluentui/react-components";

import { StageDivider } from "./StageDivider.tsx";

export type StageTitleProps = {
    /** Title to display */
    title: string;
    /** Icon to display next to title */
    icon: React.ComponentType;
    /** Description to display below the title */
    desc: string;
};

const useStyles = makeStyles({
    title: {
        display: "inline-flex",
        alignItems: "center",
        gap: "8px",
    }
});

export const StageTitle: React.FC<StageTitleProps> = memo(({title, icon: Icon, desc}) => {
    const styles = useStyles();
    return (
        <div>
            <Title3 className={styles.title} block>
                <Icon /> 
                {title}
            </Title3>
            <Body1 block> {desc} </Body1>
            <StageDivider />
        </div>
    );
});
