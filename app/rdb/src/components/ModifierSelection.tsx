/**
 * The modifier selection list used to select included, ignored and excluded modifiers.
 */

import { memo, useMemo } from "react";
import { useTranslation } from "react-i18next";
import {
    createTableColumn,
    DataGrid,
    DataGridBody,
    DataGridCell,
    DataGridHeader,
    DataGridHeaderCell,
    DataGridRow,
    TableCellLayout,
    ToggleButton,
} from "@fluentui/react-components";
import {
    Add20Filled,
    Add20Regular,
    Question20Filled,
    Question20Regular,
    Subtract20Filled,
    Subtract20Regular,
} from "@fluentui/react-icons";

import { WeaponModifiers, type WeaponModifier } from "data/WeaponModifier.ts";
import type { WeaponModifierSet } from "host/types.ts";

import { Modifier, ModifierLabel, type ModifierProps } from "./Modifier.tsx";

export type ModifierSelectionProps = {
    /** the currently selected included modifiers */
    selectedInclude: WeaponModifierSet;
    /** the currently selected excluded modifiers */
    selectedExclude: WeaponModifierSet;

    /** callback for when a modifier is selected */
    onSelect: (
        includeModifiers: WeaponModifierSet,
        excludeModifiers: WeaponModifierSet,
    ) => void;
};

export type ModifierSelectionRowProps = {
    modifier: WeaponModifier;
    included: boolean;
    excluded: boolean;
    onSelectInclude: (modifier: WeaponModifier) => void;
    onSelectExclude: (modifier: WeaponModifier) => void;
    onSelectIgnore: (modifier: WeaponModifier) => void;
    t: (key: string) => string;
};

const MODIFIER_COLUMN = "modifier" as const;
const OPTION_COLUMN = "option" as const;
const COLUMNS = [
    createTableColumn({ columnId: MODIFIER_COLUMN }),
    createTableColumn({ columnId: OPTION_COLUMN }),
];

export const ModifierSelection: React.FC<ModifierSelectionProps> = ({
    selectedInclude,
    selectedExclude,
    onSelect,
}) => {
    const { t } = useTranslation();
    const items = useMemo(() => {
        const onSelectIgnore = (modifier: WeaponModifier) => {
            const newInclude = selectedInclude & ~modifier;
            const newExclude = selectedExclude & ~modifier;
            onSelect(newInclude, newExclude);
        };
        const onSelectInclude = (modifier: WeaponModifier) => {
            if (selectedInclude & modifier) {
                onSelectIgnore(modifier);
                return;
            }
            const newExclude = selectedExclude & ~modifier;
            const newInclude = selectedInclude | modifier;
            onSelect(newInclude, newExclude);
        };
        const onSelectExclude = (modifier: WeaponModifier) => {
            if (selectedExclude & modifier) {
                onSelectIgnore(modifier);
                return;
            }
            const newInclude = selectedInclude & ~modifier;
            const newExclude = selectedExclude | modifier;
            onSelect(newInclude, newExclude);
        };
        return WeaponModifiers.map((modifier) => {
            const included = Boolean(selectedInclude & modifier);
            const excluded = Boolean(selectedExclude & modifier);
            return {
                modifier,
                included,
                excluded,
                onSelectInclude,
                onSelectExclude,
                onSelectIgnore,
                t,
            } satisfies ModifierSelectionRowProps;
        });
    }, [selectedInclude, selectedExclude, onSelect, t]);

    return (
        <DataGrid items={items} columns={COLUMNS}>
            <DataGridHeader>
                <DataGridRow>
                    {({ columnId }) => (
                        <DataGridHeaderCell>
                            {columnId === MODIFIER_COLUMN
                                ? t("search.modifier.name")
                                : t("search.modifier.option")}
                        </DataGridHeaderCell>
                    )}
                </DataGridRow>
            </DataGridHeader>
            <DataGridBody<ModifierSelectionRowProps>>
                {({ item, rowId }) => (
                    <DataGridRow<ModifierSelectionRowProps> key={rowId}>
                        {({ columnId }) => (
                            <DataGridCell
                                focusMode={
                                    columnId === MODIFIER_COLUMN
                                        ? "cell"
                                        : "group"
                                }
                            >
                                {columnId === MODIFIER_COLUMN ? (
                                    <ModifierCell modifier={item.modifier} />
                                ) : (
                                    <ModifierSelector {...item} />
                                )}
                            </DataGridCell>
                        )}
                    </DataGridRow>
                )}
            </DataGridBody>
        </DataGrid>
    );
};

const ModifierCell: React.FC<ModifierProps> = memo(({ modifier }) => {
    return (
        <TableCellLayout media={<Modifier modifier={modifier} />}>
            <ModifierLabel modifier={modifier} />
        </TableCellLayout>
    );
});

const ModifierSelector: React.FC<ModifierSelectionRowProps> = ({
    modifier,
    included,
    excluded,
    onSelectInclude,
    onSelectExclude,
    onSelectIgnore,
    t,
}) => {
    const ignore = !included && !excluded;
    return (
        <>
            <ModifierToggleButton
                label={t("search.modifier.option.include")}
                selected={included}
                onSelect={() => onSelectInclude(modifier)}
                selectedIcon={Add20Filled}
                unselectedIcon={Add20Regular}
            />
            <ModifierToggleButton
                label={t("search.modifier.option.ignore")}
                selected={ignore}
                onSelect={() => onSelectIgnore(modifier)}
                selectedIcon={Question20Filled}
                unselectedIcon={Question20Regular}
            />
            <ModifierToggleButton
                label={t("search.modifier.option.exclude")}
                selected={excluded}
                onSelect={() => onSelectExclude(modifier)}
                selectedIcon={Subtract20Filled}
                unselectedIcon={Subtract20Regular}
            />
        </>
    );
};

export type ModifierToggleButtonProps = {
    label: string;
    selected: boolean;
    onSelect: () => void;
    selectedIcon: React.ComponentType;
    unselectedIcon: React.ComponentType;
};

const ModifierToggleButton: React.FC<ModifierToggleButtonProps> = ({
    label,
    selected,
    onSelect,
    selectedIcon: Selected,
    unselectedIcon: Unselected,
}) => {
    return (
        <ToggleButton
            aria-label={label}
            shape="circular"
            appearance={selected ? "primary" : undefined}
            checked={selected}
            onClick={onSelect}
            icon={selected ? <Selected /> : <Unselected />}
        />
    );
};
