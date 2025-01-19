/**
 * Actor (item)-related components
 *
 * Things are called ItemActor to distinguish from the Actor enum type
 */

import {
    memo,
    useDeferredValue,
    useLayoutEffect,
    useMemo,
    useState,
} from "react";
import {
    Body1,
    Button,
    Caption1,
    createTableColumn,
    Label,
    type LabelProps,
    makeStyles,
    TableCellLayout,
    type TableColumnId,
    ToggleButton,
    Tooltip,
    type DataGridCellFocusMode,
    useFluent,
    useScrollbarWidth,
    type DataGridProps,
} from "@fluentui/react-components";
import {
    DataGrid,
    DataGridRow,
    DataGridHeader,
    DataGridHeaderCell,
    DataGridBody,
    DataGridCell,
    type RowRenderer,
} from "@fluentui-contrib/react-data-grid-react-window";
import {
    Add20Filled,
    Delete20Regular,
    Star20Filled,
    Star20Regular,
} from "@fluentui/react-icons";
import { useTranslation, useItemSearch } from "botw-recipe-searcher-localization";
import { Actor, ActorToName, getActors } from "botw-recipe-sys";

const useStyles = makeStyles({
    iconContainer: {
        position: "relative", // for overlay to anchor
        backgroundImage: 'url("/actors/bg.png")',
        minWidth: "42px",
        width: "42px",
        minHeight: "42px",
        height: "42px",
        padding: "2px",
    },
    disabledOverlay: {
        backgroundColor: "rgba(0, 0, 0, 0.5)",
        position: "absolute",
        inset: 0,
        zIndex: 1,
    },
    icon: {
        minWidth: "38px",
        width: "38px",
        minHeight: "38px",
        height: "38px",
    },
    bigIcon: {
        minHeight: "64px",
        height: "64px",
    },
    detailContainer: {
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        color: "#fff",
    },
    poolContainer: {
        display: "flex",
        flexWrap: "wrap",
    },
    actorSelectionContainer: {
        flex: 1,
        // this is important to avoid update loops
        // when resizing the list
        overflow: "hidden",
    },
});

export type ItemActorProps = {
    actor: Actor;
};

export type ItemActorSelectionProps = {
    included: Actor[];
    favorited: Actor[];
    disabled: boolean;
    searchText: string;
    showExcluded: boolean;
    actorSubtitles: string[];
    actorPercentages: number[] | null;
    toggleIncluded: (actor: Actor) => void;
    toggleFavorited: (actor: Actor) => void;
};

export type ItemActorSelectionRowProps = {
    actor: Actor;
    percentage: number;
    included: boolean;
    subtitle: string | null;
    favorited: boolean;
    disabled: boolean;
    toggleIncluded: (actor: Actor) => void;
    toggleFavorited: (actor: Actor) => void;
    t: (key: string) => string;
};

const ActionRenderer: React.FC<ItemActorSelectionRowProps> = memo(
    ({
        actor,
        included,
        favorited,
        disabled,
        toggleIncluded,
        toggleFavorited,
        t,
    }) => {
        const favoriteSelected = included && favorited;
        return (
            <>
                <Tooltip
                    content={
                        favoriteSelected
                            ? t("filter.selection.option.unfavorite")
                            : t("filter.selection.option.favorite")
                    }
                    relationship="label"
                >
                    <ToggleButton
                        checked={favoriteSelected}
                        disabled={disabled || !included}
                        appearance={favoriteSelected ? "primary" : undefined}
                        onClick={() => toggleFavorited(actor)}
                        icon={
                            favoriteSelected ? (
                                <Star20Filled />
                            ) : (
                                <Star20Regular />
                            )
                        }
                    />
                </Tooltip>
                <span
                    aria-hidden
                    role="presentation"
                    style={{ minWidth: 4 }}
                ></span>
                <Tooltip
                    content={
                        included
                            ? t("filter.selection.option.exclude")
                            : t("filter.selection.option.include")
                    }
                    relationship="label"
                >
                    <Button
                        disabled={disabled}
                        onClick={() => toggleIncluded(actor)}
                        icon={included ? <Delete20Regular /> : <Add20Filled />}
                    />
                </Tooltip>
            </>
        );
    },
);

const LabelRenderer: React.FC<ItemActorSelectionRowProps> = memo(
    ({ actor, disabled, subtitle }) => {
        return (
            <TableCellLayout
                media={<ItemActorIcon actor={actor} disabled={disabled} />}
            >
                <ItemActorLabel actor={actor} />
                <Caption1 block>{subtitle}</Caption1>
            </TableCellLayout>
        );
    },
);

const ItemActorSelectionColumns = [
    createTableColumn<ItemActorSelectionRowProps>({
        columnId: "actor",
        compare: (a, b) => {
            if (!a.percentage && !b.percentage) {
                return a.actor - b.actor;
            }
            const aPercentage = a.percentage || 0;
            const bPercentage = b.percentage || 0;
            return aPercentage - bPercentage;
        },
    }),
    createTableColumn<ItemActorSelectionRowProps>({ columnId: "option" }),
];

const getCellFocusMode = (columnId: TableColumnId): DataGridCellFocusMode => {
    return columnId === "actor" ? "cell" : "group";
};

export const ItemActorSelection: React.FC<ItemActorSelectionProps> = ({
    included,
    favorited,
    disabled,
    searchText,
    showExcluded,
    actorSubtitles,
    actorPercentages,
    toggleIncluded,
    toggleFavorited,
}) => {
    const styles = useStyles();
    const search = useItemSearch();
    const deferredSearchText = useDeferredValue(searchText);
    const searchResult = useMemo(
        () => search(deferredSearchText),
        [deferredSearchText, search],
    );
    const actors = useMemo(() => {
        const baseActors = showExcluded ? getActors() : included;
        if (searchResult) {
            const set = new Set(searchResult);
            return baseActors.filter((actor) => set.has(actor));
        }
        return baseActors;
    }, [included, searchResult, showExcluded]);
    const t = useTranslation();

    const rowProps = useMemo(() => {
        const favSet = new Set(favorited);
        const incSet = new Set(included);
        return actors
            .map((actor) => {
                return {
                    actor,
                    percentage: actorPercentages?.[actor] || 0,
                    included: incSet.has(actor),
                    favorited: favSet.has(actor),
                    disabled,
                    subtitle: actorSubtitles[actor] || null,
                    toggleIncluded,
                    toggleFavorited,
                    t,
                } satisfies ItemActorSelectionRowProps;
            })
            .filter(({ actor }) => actor !== Actor.None);
    }, [
        actors,
        included,
        favorited,
        disabled,
        actorSubtitles,
        actorPercentages,
        toggleIncluded,
        toggleFavorited,
        t,
    ]);

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
        sortColumn: "actor",
        sortDirection: "descending",
    });

    return (
        <div ref={divRef} className={styles.actorSelectionContainer}>
            <DataGrid
                items={rowProps}
                columns={ItemActorSelectionColumns}
                columnSizingOptions={{
                    actor: {
                        defaultWidth: 300,
                        minWidth: 240,
                    },
                    option: {
                        defaultWidth: 50,
                        minWidth: 50,
                        idealWidth: 50,
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
                                {columnId === "actor"
                                    ? t("filter.selection.actor", {
                                          count: rowProps.length,
                                      })
                                    : t("filter.selection.option")}
                            </DataGridHeaderCell>
                        )}
                    </DataGridRow>
                </DataGridHeader>
                <DataGridBody<ItemActorSelectionRowProps>
                    itemSize={44}
                    height={height}
                >
                    {renderRow}
                </DataGridBody>
            </DataGrid>
        </div>
    );
};
const renderRow: RowRenderer<ItemActorSelectionRowProps> = (
    { item, rowId },
    style,
) => {
    return (
        <DataGridRow<ItemActorSelectionRowProps> key={rowId} style={style}>
            {({ columnId }) => (
                <DataGridCell focusMode={getCellFocusMode(columnId)}>
                    {columnId === "actor" ? (
                        <LabelRenderer {...item} />
                    ) : (
                        <ActionRenderer {...item} />
                    )}
                </DataGridCell>
            )}
        </DataGridRow>
    );
};

export type ItemActorPoolProps = {
    actors: Actor[];
    disabled?: boolean;
    /** Actors not included will display disabled */
    included: Actor[];
};

export const ItemActorPool: React.FC<ItemActorPoolProps> = ({
    actors,
    disabled,
    included,
}) => {
    const styles = useStyles();
    const props = useMemo(() => {
        const includedSet = new Set(included);
        return actors.map((actor) => ({
            actor,
            disabled: disabled || !includedSet.has(actor),
        }));
    }, [actors, disabled, included]);
    return (
        <div className={styles.poolContainer}>
            {props.map((props, i) => (
                <ItemActorIconWithTooltip key={i} {...props} />
            ))}
        </div>
    );
};

/** A localized item label */
export const ItemActorLabel: React.FC<ItemActorProps & LabelProps> = ({
    actor,
    ...rest
}) => {
    const t = useTranslation();
    return <Label {...rest}>{tActor(t, actor)}</Label>;
};

export type ItemActorIconProps = ItemActorProps & {
    disabled?: boolean;
};

/** A component to display an actor image */
export const ItemActorIcon: React.FC<ItemActorIconProps> = memo(
    ({ actor, disabled }) => {
        const styles = useStyles();

        return (
            <div className={styles.iconContainer} aria-hidden>
                {!!disabled && <div className={styles.disabledOverlay} />}
                <img className={styles.icon} src={getIconUrl(actor)} />
            </div>
        );
    },
);

/** A component to display an actor image with tooltip as a label */
export const ItemActorIconWithTooltip: React.FC<ItemActorIconProps> = ({
    actor,
    disabled,
}) => {
    const [ref, setRef] = useState<HTMLSpanElement | null>(null);
    return (
        <Tooltip
            appearance="inverted"
            positioning={{ target: ref }}
            relationship="label"
            content={<ItemActorDetail actor={actor} />}
        >
            <span ref={setRef}>
                <ItemActorIcon actor={actor} disabled={disabled} />
            </span>
        </Tooltip>
    );
};

/** A component to display detail of the Actor */
export const ItemActorDetail: React.FC<ItemActorProps> = ({ actor }) => {
    const styles = useStyles();
    const t = useTranslation();
    return (
        <div className={styles.detailContainer}>
            <img className={styles.bigIcon} src={getIconUrl(actor)} />
            <Body1>{tActor(t, actor)}</Body1>
            <Caption1>{ActorToName[actor]}</Caption1>
        </div>
    );
};

function tActor(t: (k: string) => string, actor: Actor): string {
    return t(`actor.${ActorToName[actor]}`);
}

function getIconUrl(actor: Actor): string {
    return `/actors/${ActorToName[actor]}.png`;
}
