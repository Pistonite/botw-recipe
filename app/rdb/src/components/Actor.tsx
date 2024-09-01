/**
 * Actor (item)-related components
 *
 * Things are called ItemActor to distinguish from the Actor enum type
 */

import { memo, useMemo } from "react";
import { useTranslation } from "react-i18next";
import {
    Button,
    Caption1,
    createTableColumn,
    DataGrid,
    DataGridBody,
    DataGridCell,
    DataGridCellFocusMode,
    DataGridHeader,
    DataGridHeaderCell,
    DataGridRow,
    Label,
    LabelProps,
    makeStyles,
    shorthands,
    TableCellLayout,
    TableColumnId,
    ToggleButton,
} from "@fluentui/react-components";
import {
    Add20Filled,
    Delete20Regular,
    Star20Filled,
    Star20Regular,
} from "@fluentui/react-icons";

import { Actor, ActorToName, getActors } from "data/Actor.ts";
import { useItemSearch } from "i18n/itemSearch";

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
    setIncluded: (included: Actor[]) => void;
    setFavorited: (favorited: Actor[]) => void;
};

export type ItemActorSelectionRowProps = {
    actor: Actor;
    included: boolean;
    subtitle: string | null;
    favorited: boolean;
    onSelectInclude: (actor: Actor, include: boolean) => void;
    onSelectFavorite: (actor: Actor, favorited: boolean) => void;
    t: (key: string) => string;
};

const ItemActorSelectionColumns = [
    createTableColumn<ItemActorSelectionRowProps>({
        columnId: "actor",
        renderHeaderCell: (t) =>
            (t as (key: string) => string)("filter.selection.actor"),
        renderCell: ({ actor, subtitle }) => {
            return (
                <TableCellLayout media={<ItemActorImg actor={actor} />}>
                    <ItemActorLabel actor={actor} />
                    <Caption1 block>{subtitle}&nbsp;</Caption1>
                </TableCellLayout>
            );
        },
    }),
    createTableColumn<ItemActorSelectionRowProps>({
        columnId: "option",
        renderHeaderCell: (t) =>
            (t as (key: string) => string)("filter.selection.option"),
        renderCell: ({
            actor,
            included,
            favorited,
            onSelectInclude,
            onSelectFavorite,
            t,
        }) => {
            const favoriteSelected = included && favorited;
            return (
                <>
                    <ToggleButton
                        aria-label={t("filter.selection.option.include")}
                        checked={favoriteSelected}
                        disabled={!included}
                        appearance={favoriteSelected ? "primary" : undefined}
                        onClick={() => onSelectFavorite(actor, !favorited)}
                        icon={
                            favoriteSelected ? (
                                <Star20Filled />
                            ) : (
                                <Star20Regular />
                            )
                        }
                    />
                    <Button
                        aria-label={
                            included
                                ? t("filter.selection.option.exclude")
                                : t("filter.selection.option.include")
                        }
                        onClick={() => onSelectInclude(actor, !included)}
                        icon={included ? <Delete20Regular /> : <Add20Filled />}
                    />
                </>
            );
        },
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
    setIncluded,
    setFavorited,
}) => {
    const search = useItemSearch();
    const searchResult = useMemo(
        () => search(searchText),
        [searchText, search],
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
        const onSelectInclude = (actor: Actor, include: boolean) => {
            setIncluded(
                include
                    ? [...included, actor]
                    : included.filter((a) => a !== actor),
            );
        };
        const onSelectFavorite = (actor: Actor, isFavorited: boolean) => {
            setFavorited(
                isFavorited
                    ? [...favorited, actor]
                    : favorited.filter((a) => a !== actor),
            );
        };
        return actors.map((actor) => {
            return {
                actor,
                included: incSet.has(actor),
                favorited: favSet.has(actor),
                subtitle: actorSubtitles[actor] || null,
                onSelectInclude,
                onSelectFavorite,
                t,
            } satisfies ItemActorSelectionRowProps;
        });
    }, [
        actors,
        included,
        favorited,
        actorSubtitles,
        setIncluded,
        setFavorited,
        t,
    ]);

    return (
        <DataGrid items={rowProps} columns={ItemActorSelectionColumns}>
            <DataGridHeader>
                <DataGridRow>
                    {({ renderHeaderCell }) => (
                        <DataGridHeaderCell>
                            {renderHeaderCell(t)}
                        </DataGridHeaderCell>
                    )}
                </DataGridRow>
            </DataGridHeader>
            <DataGridBody<ItemActorSelectionRowProps>>
                {({ item, rowId }) => (
                    <DataGridRow<ItemActorSelectionRowProps> key={rowId}>
                        {({ renderCell, columnId }) => (
                            <DataGridCell
                                focusMode={getCellFocusMode(columnId)}
                            >
                                {renderCell(item)}
                            </DataGridCell>
                        )}
                    </DataGridRow>
                )}
            </DataGridBody>
        </DataGrid>
    );
};

/** A localized item label */
export const ItemActorLabel: React.FC<ItemActorProps & LabelProps> = ({
    actor,
    ...rest
}) => {
    const { t } = useTranslation();
    return <Label {...rest}>{t(`actor.${ActorToName[actor]}`)}</Label>;
};

/** A component to display an actor image */
export const ItemActorImg: React.FC<ItemActorProps> = memo(({ actor }) => {
    const styles = useStyles();

    return (
        <div className={styles.iconContainer} aria-hidden>
            <img
                className={styles.icon}
                src={`/actors/${ActorToName[actor]}.png`}
            />
        </div>
    );
});
