/**
 * Group (item)-related components
 *
 * Things are called ItemGroup to distinguish from the Actor enum type
 */

import { memo, useState } from "react";
import { Body1, makeStyles, Tooltip } from "@fluentui/react-components";
import { useTranslation } from "botw-recipe-searcher-localization";
import type { Actor } from "botw-recipe-sys";

import { useItemAnimationFrame } from "util/useItemAnimationFrame.ts";

import { ItemActorDetail, ItemActorIcon, ItemActorPool } from "./ItemActor.tsx";

const useStyles = makeStyles({
    anyOfLabel: {
        paddingBottom: "4px",
    },
});

export type ItemGroupProps = {
    actors: Actor[];
};

/**
 * Showing a group of items as one ingredient.
 *
 * The items will be cycled through in the UI. The tooltip
 * will show all the items
 */
export const ItemGroup: React.FC<ItemGroupProps> = memo(({ actors }) => {
    const frame = useItemAnimationFrame(actors.length);
    const [ref, setRef] = useState<HTMLSpanElement | null>(null);

    return (
        <Tooltip
            positioning={{ target: ref }}
            appearance="inverted"
            withArrow
            content={<ItemGroupDetail actors={actors} />}
            relationship="label"
        >
            <span ref={setRef}>
                <ItemActorIcon actor={actors[frame]} />
            </span>
        </Tooltip>
    );
});

export const ItemGroupDetail: React.FC<ItemGroupProps> = memo(({ actors }) => {
    const styles = useStyles();
    const t = useTranslation();
    if (actors.length <= 1) {
        return <ItemActorDetail actor={actors[0]} />;
    }
    return (
        <div>
            <Body1 block className={styles.anyOfLabel}>
                {t("result.any_of")}
            </Body1>
            <ItemActorPool actors={actors} included={actors} />
        </div>
    );
});
