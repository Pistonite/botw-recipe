import { useDeferredValue, useLayoutEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import {
    createTableColumn,
    DataGridCell,
    DataGridHeader,
    DataGridHeaderCell,
    type DataGridProps,
    makeStyles,
    useFluent,
    useScrollbarWidth,
} from "@fluentui/react-components";
import {
    DataGrid,
    DataGridBody,
    DataGridRow,
    type RowRenderer,
} from "@fluentui-contrib/react-data-grid-react-window";

import type { ResultData } from "data/ResultData.ts";
import type { SearchFilter } from "host/types.ts";
import type { Actor } from "data/Actor.ts";

import { ModifierData } from "./Modifier.tsx";
import { ItemGroup } from "./ItemGroup.tsx";

export type ResultListProps = {
    filter: SearchFilter;
    favorited: Actor[];
    results: ResultData[];
};

const RECIPE_COLUMN = "recipe";
const VALUE_COLUMN = "value";
const MODIFIER_COLUMN = "modifier";

const useStyles = makeStyles({
    container: {
        flex: 1,
        // this is important to avoid update loops
        // when resizing the list
        overflow: "hidden",
    },
    recipeContainer: {
        display: "flex",
    },
});

export type ResultListRowProps = {
    data: ResultData;
    isCritRngRequired: boolean;
    favoritedScore: number;
};

const COLUMNS = [
    createTableColumn<ResultListRowProps>({
        columnId: RECIPE_COLUMN,
        compare: (a, b) => {
            return a.favoritedScore - b.favoritedScore;
        },
    }),
    createTableColumn<ResultListRowProps>({
        columnId: VALUE_COLUMN,
        compare: (a, b) => {
            const aValue = getDisplayValue(a);
            const bValue = getDisplayValue(b);
            if (aValue === bValue) {
                if (a.isCritRngRequired && !b.isCritRngRequired) {
                    return -1;
                }
                if (!a.isCritRngRequired && b.isCritRngRequired) {
                    return 1;
                }
                return 0;
            }
            return aValue - bValue;
        },
    }),
    createTableColumn({ columnId: MODIFIER_COLUMN }),
];

const RecipeCell: React.FC<ResultListRowProps> = ({ data }) => {
    const styles = useStyles();
    const sortedInputs = useMemo(() => {
        return [...data.actors].sort((a, b) => {
            if (a.length === 1 && b.length === 1) {
                return a[0] - b[0];
            }
            return a.length - b.length;
        });
    }, [data.actors]);
    return (
        <div className={styles.recipeContainer}>
            {sortedInputs.map((actors, i) => (
                <ItemGroup key={i} actors={actors} />
            ))}
        </div>
    );
};

const ValueCell: React.FC<ResultListRowProps> = (props) => {
    const value = getDisplayValue(props);
    return (
        <>
            {value}
            {props.isCritRngRequired && "*"}
        </>
    );
};

const ModifierCell: React.FC<ResultListRowProps> = ({ data }) => {
    return <ModifierData modifiers={data.modifiers} value={data.value} />;
};

function getDisplayValue({
    data,
    isCritRngRequired,
}: ResultListRowProps): number {
    if (isCritRngRequired) {
        const extra = data.isHearty ? 4 : 12;
        return Math.min(120, data.value + extra);
    }
    return data.value;
}

/** Component to display cook results */
export const ResultList: React.FC<ResultListProps> = ({
    filter,
    favorited,
    results,
}) => {
    const deferredResults = useDeferredValue(results);
    const deferredFavorited = useDeferredValue(favorited);
    const rowProps = useMemo(() => {
        const favoritedSet = new Set(favorited);
        const scoreOf = (data: ResultData) => {
            let score = 0;
            for (const actors of data.actors) {
                for (const actor of actors) {
                    if (favoritedSet.has(actor)) {
                        score++;
                    }
                }
            }
            return score;
        };
        return results.map((data) => {
            const isCritRngRequired =
                data.value < filter.minValue || data.value > filter.maxValue;
            return {
                data,
                isCritRngRequired,
                favoritedScore: scoreOf(data),
            };
        });
    }, [filter, deferredFavorited, deferredResults]);

    const { t } = useTranslation();
    const styles = useStyles();
    const { targetDocument } = useFluent();
    const scrollbarWidth = useScrollbarWidth({ targetDocument });
    const [div, divRef] = useState<HTMLDivElement | null>(null);
    const [header, headerRef] = useState<HTMLDivElement | null>(null);
    const [height, setHeight] = useState(0);
    useLayoutEffect(() => {
        if (!div || !header) {
            return;
        }
        const observer = new ResizeObserver(() => {
            setHeight(div.clientHeight - header.clientHeight);
        });
        observer.observe(div);
        observer.observe(header);
        return () => {
            observer.disconnect();
        };
    }, [div, header, setHeight]);

    const [sortState, setSortState] = useState<DataGridProps["sortState"]>({
        sortColumn: VALUE_COLUMN,
        sortDirection: "descending",
    });

    return (
        <div ref={divRef} className={styles.container}>
            <DataGrid
                items={rowProps}
                columns={COLUMNS}
                columnSizingOptions={{
                    [RECIPE_COLUMN]: {
                        defaultWidth: 220,
                        minWidth: 220,
                    },
                    [VALUE_COLUMN]: {
                        defaultWidth: 80,
                        minWidth: 80,
                        idealWidth: 80,
                    },
                }}
                resizableColumns={true}
                sortable
                sortState={sortState}
                onSortChange={(_, next) => setSortState(next)}
            >
                <DataGridHeader
                    ref={headerRef}
                    style={{ paddingRight: scrollbarWidth }}
                >
                    <DataGridRow>
                        {({ columnId }) => (
                            <DataGridHeaderCell>
                                {columnId === RECIPE_COLUMN &&
                                sortState?.sortColumn === columnId
                                    ? sortState.sortDirection === "ascending"
                                        ? t("result.header.recipe_asc")
                                        : t("result.header.recipe_desc")
                                    : t(`result.header.${columnId}`)}
                            </DataGridHeaderCell>
                        )}
                    </DataGridRow>
                </DataGridHeader>
                <DataGridBody<ResultListRowProps> itemSize={44} height={height}>
                    {renderRow}
                </DataGridBody>
            </DataGrid>
        </div>
    );
};

const renderRow: RowRenderer<ResultListRowProps> = ({ item, rowId }, style) => {
    return (
        <DataGridRow<ResultListRowProps> key={rowId} style={style}>
            {({ columnId }) => (
                <DataGridCell focusMode="cell">
                    {columnId === RECIPE_COLUMN && <RecipeCell {...item} />}
                    {columnId === VALUE_COLUMN && <ValueCell {...item} />}
                    {columnId === MODIFIER_COLUMN && <ModifierCell {...item} />}
                </DataGridCell>
            )}
        </DataGridRow>
    );
};
