/**
 * Modifier related components
 */
import { useTranslation } from "react-i18next";
import { useMemo } from "react";
import { createTableColumn, DataGrid, DataGridBody, DataGridCell, DataGridCellFocusMode, DataGridHeader, DataGridHeaderCell, DataGridRow, Label, LabelProps, makeStyles, shorthands, TableCellLayout, TableColumnId, ToggleButton } from "@fluentui/react-components";
import { WeaponModifier, WeaponModifiers } from "data/WeaponModifier.ts";
import { Add20Filled, Add20Regular, Question20Filled, Question20Regular, Subtract20Filled, Subtract20Regular } from "@fluentui/react-icons";
import { WeaponModifierSet } from "host/types.ts";

const useStyles = makeStyles({
    iconContainer: {
        borderRadius: "4px",
        minWidth: "26px",
        width: "26px",
        minHeight: "26px",
        height: "26px",
       backgroundColor: "rgba(0, 0, 0, 0.8)",
       ...shorthands.padding("1px"),
    },
    icon: {
        minWidth: "24px",
        width: "24px",
        minHeight: "24px",
        height: "24px",
        borderRadius: "2px",
        ...shorthands.border("1px", "solid", "#888"),
    }
});


export type ModifierSelectionProps = {
    /** the currently selected included modifiers */
    selectedInclude: WeaponModifierSet;
    /** the currently selected excluded modifiers */
    selectedExclude: WeaponModifierSet;

    /** callback for when a modifier is selected */
    onSelect: (includeModifiers: WeaponModifierSet, excludeModifiers: WeaponModifierSet) => void;
}

export type ModifierProps = {
    /** which modifier to display */
    modifier: WeaponModifier;
}

type ModifierSelectionRowProps = {
    modifier: WeaponModifier;
    included: boolean;
    excluded: boolean;
    onSelectInclude: (modifier: WeaponModifier) => void;
    onSelectExclude: (modifier: WeaponModifier) => void;
    onSelectIgnore: (modifier: WeaponModifier) => void;
}

const ModifierSelectionColumns = [
    createTableColumn<ModifierSelectionRowProps>({
        columnId: "modifier",
        renderHeaderCell: (t) => (t as (key: string)=>string)("search.modifier.name"),
    renderCell: ({modifier}) => {
      return (
        <TableCellLayout media={<Modifier modifier={modifier}/>} >
          <ModifierLabel modifier={modifier} />
        </TableCellLayout>
      );
    },
    }),
    createTableColumn<ModifierSelectionRowProps>({
        columnId: "option",
        renderHeaderCell: (t) => (t as (key: string)=>string)("search.modifier.option"),
        renderCell: ({modifier, included, excluded, onSelectInclude, onSelectExclude, onSelectIgnore}) => {
            const ignore = !included && !excluded;
            return (<>
                <ToggleButton
                        shape="circular"
                        appearance={included ? "primary" : undefined}
                    checked={included}
                    onClick={() => onSelectInclude(modifier)}
                        icon={included ? <Add20Filled /> : <Add20Regular />}
                />
                <ToggleButton
                        shape="circular"
                        appearance={ignore ? "primary" :undefined}
                    checked={ignore}
                    onClick={() => onSelectIgnore(modifier)}
                        icon={ignore ? <Question20Filled /> : <Question20Regular />}

                />
                <ToggleButton
                        shape="circular"
                        appearance={excluded ? "primary" :undefined}
                    checked={excluded}
                    onClick={() => onSelectExclude(modifier)}
                        icon={excluded ? <Subtract20Filled /> : <Subtract20Regular />}
                />
            </>);
        },
    }),

];
const getCellFocusMode = (columnId: TableColumnId): DataGridCellFocusMode => {
  return columnId === "modifier" ? "cell" : "group";
};

export const ModifierSelection: React.FC<ModifierSelectionProps> = ({ 
    selectedInclude, selectedExclude, onSelect }) => {
    const {t} = useTranslation();
    const items = useMemo(() => {
        const onSelectIgnore = (modifier: WeaponModifier) => {
            const newInclude = selectedInclude & ~modifier;
            const newExclude = selectedExclude & ~modifier;
            onSelect(newInclude, newExclude);
        }
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
        }
        return WeaponModifiers.map(modifier => {
            const included = Boolean(selectedInclude & modifier);
            const excluded = Boolean(selectedExclude & modifier);
            return {
                modifier,
                included,
                excluded,
                onSelectInclude,
                onSelectExclude,
                onSelectIgnore,
            };
        });
    }, [selectedInclude, selectedExclude, onSelect]);
    return (
    <DataGrid
        items={items}
        columns={ModifierSelectionColumns}
        >
      <DataGridHeader>
        <DataGridRow

        >
          {({ renderHeaderCell }) => (
            <DataGridHeaderCell>{renderHeaderCell(t)}</DataGridHeaderCell>
          )}
        </DataGridRow>
      </DataGridHeader>
      <DataGridBody<ModifierSelectionRowProps>>
        {({ item, rowId }) => (
          <DataGridRow<ModifierSelectionRowProps>
            key={rowId}
         
          >
            {({ renderCell, columnId }) => (
              <DataGridCell  focusMode={getCellFocusMode(columnId)}>
                {renderCell(item)}
              </DataGridCell>
            )}
          </DataGridRow>
        )}
      </DataGridBody>
    </DataGrid>
    );
};

/** A localized modifier label */
export const ModifierLabel: React.FC<ModifierProps & LabelProps> = ({ modifier, ...rest }) => {
    const {t} = useTranslation();
    return (
        <Label {...rest}>{t(`modifier.${modifier}`)}</Label>
    );
};

/** Component that displays a weapon modifier block */
export const Modifier: React.FC<ModifierProps> = ({ modifier }) => {
    const styles = useStyles();

    return (
        <div className={styles.iconContainer} aria-hidden>
            <img className={styles.icon} src={`/modifiers/${modifier}.png`} />
        </div>
    );
};
