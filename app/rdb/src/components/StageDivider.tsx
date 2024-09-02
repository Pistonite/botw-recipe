import { memo } from "react";
import { Divider, makeStyles, shorthands } from "@fluentui/react-components";

const useStyles = makeStyles({
    divider: {
        ...shorthands.padding("10px", "0px"),
    },
});

export const StageDivider: React.FC = memo(() => {
    const { divider } = useStyles();
    return <Divider className={divider} />;
});
