import { memo } from "react";
import { Divider, makeStyles } from "@fluentui/react-components";

const useStyles = makeStyles({
    divider: {
        padding: "10px 0px",
    },
});

export const StageDivider: React.FC = memo(() => {
    const { divider } = useStyles();
    return <Divider className={divider} />;
});
