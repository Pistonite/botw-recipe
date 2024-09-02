import {
    Button,
    createTableColumn,
    DataGrid,
    DataGridBody,
    DataGridCell,
    DataGridCellFocusMode,
    DataGridHeader,
    DataGridHeaderCell,
    DataGridRow,
    Divider,
    makeStaticStyles,
    makeStyles,
    shorthands,
    TableCellLayout,
    TableColumnDefinition,
    TableColumnId,
    Title3,
} from "@fluentui/react-components";
import {
    DeleteRegular,
    EditRegular,
    List24Regular,
} from "@fluentui/react-icons";

import { WeaponModifier, WeaponModifiers } from "data/WeaponModifier.ts";
import { Modifier, ModifierLabel } from "components/Modifier.tsx";
import { LocalePicker } from "components/LocalePicker.tsx";
import { SearchStage } from "stage/SearchStage.tsx";
import { FilterStage } from "stage/FilterStage.tsx";
import { StageTitle } from "components/StageTitle";

const useStaticStyles = makeStaticStyles({
    "*": {
        minWidth: 0,
        boxSizing: "border-box",
    },
    ":root": {
        fontSynthesis: "none",
        textRendering: "optimizeLegibility",
        WebkitFontSmoothing: "antialiased",
        MozOsxFontSmoothing: "grayscale",
        WebkitTextSizeAdjust: "100%",
    },
    body: {
        ...shorthands.margin(0),
        ...shorthands.padding(0),
        overflow: "hidden",
    },
});

const useStyles = makeStyles({
    stageContainer: {
        display: "flex",
        flexDirection: "column",
        height: "100%",
        ...shorthands.padding("10px"),
        backgroundColor: "#fcfcfc",
    },
});

export const App: React.FC = () => {
    useStaticStyles();
    const styles = useStyles();

    return (
        <>
            <div
                style={{
                    display: "flex",
                    height: "100vh",
                    backgroundColor: "#ccc",
                    gap: 1,
                }}
            >
                <div
                    className={styles.stageContainer}
                    style={{
                        minWidth: 300,
                        maxWidth: 360,
                    }}
                >
                    <SearchStage />
                </div>
                <div className={styles.stageContainer} style={{ 
                    minWidth: 440,
                    width: 440,
                    maxWidth: 440,
                }}>
                    <FilterStage />
                </div>
                <div className={styles.stageContainer} style={{ flex: 1 }}>
                    <StageTitle title="Results" icon={List24Regular} desc="Recipes here should both have the desired modifiers, and only include the desired items"/>
                </div>
            </div>
            <div style={{ position: "fixed", right: 10, top: 10 }}>
                <LocalePicker />
            </div>
        </>
    );
};
type Item = {
    modifier: WeaponModifier;
};
const items: Item[] = WeaponModifiers.map((modifier) => ({ modifier }));

const columns: TableColumnDefinition<Item>[] = [
    createTableColumn<Item>({
        columnId: "modifier",
        renderHeaderCell: () => {
            return "Modifier";
        },
        renderCell: ({ modifier }) => {
            return (
                <TableCellLayout media={<Modifier modifier={modifier} />}>
                    <ModifierLabel modifier={modifier} />
                </TableCellLayout>
            );
        },
    }),
    createTableColumn<Item>({
        columnId: "option",
        renderHeaderCell: () => {
            return "Option";
        },
        renderCell: () => {
            return (
                <>
                    <Button aria-label="Edit" icon={<EditRegular />} />
                    <Button aria-label="Delete" icon={<DeleteRegular />} />
                    <Button aria-label="2" icon={<DeleteRegular />} />
                </>
            );
        },
    }),
];

const getCellFocusMode = (columnId: TableColumnId): DataGridCellFocusMode => {
    return columnId === "modifier" ? "cell" : "group";
};

export const FocusableElementsInCells = () => {
    return (
        <DataGrid items={items} columns={columns}>
            <DataGridHeader>
                <DataGridRow>
                    {({ renderHeaderCell }) => (
                        <DataGridHeaderCell>
                            {renderHeaderCell()}
                        </DataGridHeaderCell>
                    )}
                </DataGridRow>
            </DataGridHeader>
            <DataGridBody<Item>>
                {({ item, rowId }) => (
                    <DataGridRow<Item> key={rowId}>
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

export const FocusableElementsInCells2 = () => {
    return (
        <DataGrid
            style={{ height: "100%", display: "flex", flexDirection: "column" }}
            items={[
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
                { modifier: { label: "Spicy Pepper", icon: "AddLife" } },
            ]}
            columns={columns}
            sortable
            getRowId={(item) => item.modifier.label}
        >
            <DataGridHeader>
                <DataGridRow>
                    {({ renderHeaderCell }) => (
                        <DataGridHeaderCell>
                            {renderHeaderCell()}
                        </DataGridHeaderCell>
                    )}
                </DataGridRow>
            </DataGridHeader>

            <DataGridBody<Item>
                style={{ minHeight: 0, flex: 1, overflowY: "auto" }}
            >
                {({ item, rowId }) => (
                    <DataGridRow<Item> key={rowId}>
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
