/**
 * Actor (item)-related components
 *
 * Things are called ItemActor to distinguish from the Actor enum type
 */

import {
    memo,
    useCallback,
    useDeferredValue,
    useLayoutEffect,
    useMemo,
    useState,
} from "react";
import { useTranslation } from "react-i18next";
import {
    Body1,
    Button,
    Caption1,
    createTableColumn,
    Label,
    LabelProps,
    makeStyles,
    shorthands,
    TableCellLayout,
    TableColumnId,
    ToggleButton,
    Tooltip,
    DataGridCellFocusMode,
    useFluent,
    useScrollbarWidth,
} from "@fluentui/react-components";
import {
    DataGrid,
    DataGridRow,
    DataGridHeader,
    DataGridHeaderCell,
    DataGridBody,
    DataGridCell,
    RowRenderer,
} from "@fluentui-contrib/react-data-grid-react-window";
import {
    Add20Filled,
    Delete20Regular,
    Star20Filled,
    Star20Regular,
} from "@fluentui/react-icons";

import { Actor, ActorToName, getActors } from "data/Actor.ts";
import { useItemSearch } from "i18n/itemSearch.ts";

const useStyles = makeStyles({
    iconContainer: {
        backgroundImage: 'url("/actors/bg.png")',
        minWidth: "42px",
        width: "42px",
        minHeight: "42px",
        height: "42px",
        ...shorthands.padding("2px"),
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
    searchText: string;
    showExcluded: boolean;
    actorSubtitles: string[];
    toggleIncluded: (actor: Actor) => void;
    toggleFavorited: (actor: Actor) => void;
};

export type ItemActorSelectionRowProps = {
    actor: Actor;
    included: boolean;
    subtitle: string | null;
    favorited: boolean;
    toggleIncluded: (actor: Actor) => void;
    toggleFavorited: (actor: Actor) => void;
    t: (key: string) => string;
};

const ActionRenderer: React.FC<ItemActorSelectionRowProps> = memo(
    ({ actor, included, favorited, toggleIncluded, toggleFavorited, t }) => {
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
                        disabled={!included}
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
                        onClick={() => toggleIncluded(actor)}
                        icon={included ? <Delete20Regular /> : <Add20Filled />}
                    />
                </Tooltip>
            </>
        );
    },
);

const LabelRenderer: React.FC<ItemActorSelectionRowProps> = memo(
    ({ actor, subtitle }) => {
        return (
            <TableCellLayout media={<ItemActorIcon actor={actor} />}>
                <ItemActorLabel actor={actor} />
                <Caption1 block>{subtitle}&nbsp;</Caption1>
            </TableCellLayout>
        );
    },
);

const ItemActorSelectionColumns = [
    createTableColumn<ItemActorSelectionRowProps>({
        columnId: "actor",
        renderHeaderCell: (t) =>
            (t as (key: string) => string)("filter.selection.actor"),
        // renderCell: ({ actor, subtitle }) => {
        //     return (
        //         <TableCellLayout media={<ItemActorIcon actor={actor} />}>
        //             <ItemActorLabel actor={actor} />
        //             <Caption1 block>{subtitle}&nbsp;</Caption1>
        //         </TableCellLayout>
        //     );
        // },
    }),
    createTableColumn<ItemActorSelectionRowProps>({
        columnId: "option",
        renderHeaderCell: (t) =>
            (t as (key: string) => string)("filter.selection.option"),
        // renderCell: ActionRenderer,
    }),
];

const getCellFocusMode = (columnId: TableColumnId): DataGridCellFocusMode => {
    return columnId === "actor" ? "cell" : "group";
};

export const ItemActorSelection: React.FC<ItemActorSelectionProps> = ({
    included,
    favorited,
    searchText,
    showExcluded,
    actorSubtitles,
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
    const { t } = useTranslation();

    const rowProps = useMemo(() => {
        const favSet = new Set(favorited);
        const incSet = new Set(included);
        return actors
            .map((actor) => {
                return {
                    actor,
                    included: incSet.has(actor),
                    favorited: favSet.has(actor),
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
        actorSubtitles,
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

    const renderRow = useCallback<RowRenderer<ItemActorSelectionRowProps>>(
        ({ item, rowId }, style) => {
            return (
                <DataGridRow<ItemActorSelectionRowProps>
                    key={rowId}
                    style={style}
                >
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
        },
        [],
    );

    return (
        <div ref={divRef} className={styles.actorSelectionContainer}>
            <DataGrid
                items={rowProps}
                columns={ItemActorSelectionColumns}
                columnSizingOptions={{
                    actor: {
                        defaultWidth: 300,
                        minWidth: 300,
                    },
                    option: {
                        defaultWidth: 50,
                        minWidth: 50,
                        idealWidth: 50,
                    },
                }}
                resizableColumns={true}
            >
                <DataGridHeader
                    ref={headerRef}
                    style={{ paddingRight: scrollbarWidth }}
                >
                    <DataGridRow>
                        {({ renderHeaderCell }) => (
                            <DataGridHeaderCell>
                                {renderHeaderCell(t)}
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

export type ItemActorPoolProps = {
    actors: Actor[];
};

export const ItemActorPool: React.FC<ItemActorPoolProps> = ({ actors }) => {
    const styles = useStyles();
    return (
        <div className={styles.poolContainer}>
            {actors.map((actor, i) => (
                <ItemActorIconWithTooltip key={i} actor={actor} />
            ))}
        </div>
    );
};

/** A localized item label */
export const ItemActorLabel: React.FC<ItemActorProps & LabelProps> = ({
    actor,
    ...rest
}) => {
    const { t } = useTranslation();
    return <Label {...rest}>{tActor(t, actor)}</Label>;
};

/** A component to display an actor image */
export const ItemActorIcon: React.FC<ItemActorProps> = memo(({ actor }) => {
    const styles = useStyles();

    return (
        <div className={styles.iconContainer} aria-hidden>
            <img className={styles.icon} src={getIconUrl(actor)} />
        </div>
    );
});

export const ItemActorIconWithTooltip: React.FC<ItemActorProps> = ({
    actor,
}) => {
    const [ref, setRef] = useState<HTMLSpanElement | null>(null);
    return (
        <Tooltip
            appearance="inverted"
            positioning={{ target: ref }}
            relationship="description"
            content={<ItemActorDetail actor={actor} />}
        >
            <span ref={setRef}>
                <ItemActorIcon actor={actor} />
            </span>
        </Tooltip>
    );
};

/** A component to display detail of the Actor*/
export const ItemActorDetail: React.FC<ItemActorProps> = ({ actor }) => {
    const styles = useStyles();
    const { t } = useTranslation();
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
