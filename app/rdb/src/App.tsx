import "./App.css";
import { useTranslation } from "react-i18next";
import { Body1, Button, Caption1, Checkbox, CheckboxOnChangeData, createTableColumn, DataGrid, DataGridBody, DataGridCell, DataGridCellFocusMode, DataGridHeader, DataGridHeaderCell, DataGridRow, Divider, Field, FluentProvider, SearchBox, SpinButton, SpinButtonOnChangeData, SpinButtonProps, Subtitle2, TableCellLayout, TableColumnDefinition, TableColumnId, Title3, Tooltip, webLightTheme } from "@fluentui/react-components";
import { DeleteRegular, EditRegular, Filter24Regular, Info16Regular, List24Regular, Search24Regular } from "@fluentui/react-icons";
import { WeaponModifier, WeaponModifiers, WeaponModifierSet } from "data/WeaponModifier";
import { Modifier, ModifierLabel, ModifierSelection } from "components/Modifier";
import { LocalePicker } from "components/LocalePicker";
import { useCallback, useState } from "react";
import { SearchFilter } from "host";
import { useHost } from "HostProvider";

function parseHp(value: number | undefined | null, displayValue: string | undefined): number {
    if (value !== undefined) {
        return Math.min(120, Math.max(0, value || 0));
    }
    if (displayValue !== undefined) {
        const parsedValue = parseInt(displayValue);
        if (isNaN(parsedValue)) {
            return 0;
        }
        return Math.min(120, Math.max(0, parsedValue || 0));
    }
    return 0;
}


export const App: React.FC = () => {
    const [searchMinHp, setSearchMinHp] = useState<number>(0);
    const onSearchMinHpChange: SpinButtonProps["onChange"] = useCallback((_: unknown, data: SpinButtonOnChangeData) => {
        setSearchMinHp(parseHp(data.value, data.displayValue));
    }, []);
    const [searchMaxHp, setSearchMaxHp] = useState<number>(120);
    const onSearchMaxHpChange: SpinButtonProps["onChange"] = useCallback((_: unknown, data: SpinButtonOnChangeData) => {
        setSearchMaxHp(parseHp(data.value, data.displayValue));
    }, []);
    const [searchCritRngHp, setSearchCritRngHp] = useState<boolean>(false);
    const onSearchCritRngHpChange = useCallback((_: unknown, data: CheckboxOnChangeData) => {
        setSearchCritRngHp(data.checked === true);
    }, []);
    const [searchAllowPe, setSearchAllowPe] = useState<boolean>(true);
    const onSearchAllowPeChange = useCallback((_: unknown, data: CheckboxOnChangeData) => {
        setSearchAllowPe(data.checked === true);
    }, []);
    const [searchIncludeModifiers, setSearchIncludeModifiers] = useState<WeaponModifierSet>(0);
    const [searchExcludeModifiers, setSearchExcludeModifiers] = useState<WeaponModifierSet>(0);

    const onSelectSearchModifiers = useCallback((includeModifiers: WeaponModifierSet, excludeModifiers: WeaponModifierSet) => {
        setSearchIncludeModifiers(includeModifiers);
        setSearchExcludeModifiers(excludeModifiers);
    }, []);


const {t} = useTranslation();

    const host = useHost();

  return (
    <FluentProvider theme={webLightTheme}>
      <div style={{display: "flex", height: "100vh"}}>
        <div className="container" style={{minWidth: 300, maxWidth: 360, height: "100%", display: "flex", flexDirection: "column"}}>
          <div>
          <Title3
                            style={{display: "inline-flex", alignItems: "center", gap: 8}}
                            block> <Search24Regular /> {t("search.title")}</Title3>
          <Body1 block>
                            {t("search.desc")}
                        </Body1>
          <Divider className="divider" />

          </div>
          
          <div style={{flex: 1, overflowY: "auto"}}>
                        <Subtitle2 block>
                            {t("search.value.title")}
                        </Subtitle2>
          
          <Field label={t("search.value.min")}>
            <SpinButton
                value={searchMinHp}
                                min={0}
                                max={120}
                onChange={onSearchMinHpChange}
                            />
          </Field>
          <Field label={t("search.value.max")}>
            <SpinButton 
                value={searchMaxHp}
                                min={0}
                                max={120}
                onChange={onSearchMaxHpChange}
                            />
          </Field>
          
            <Checkbox 
            checked={searchCritRngHp}
            onChange={onSearchCritRngHpChange}
                            label={

                <>{t("search.value.rng")}
                <span style={{paddingLeft: 4}}></span>
                <Tooltip appearance="inverted" withArrow content={t("search.value.rng.desc")} relationship={"label"}>
              <Info16Regular/>
          </Tooltip></>
            
              
            } />
                      <Checkbox 
            checked={searchAllowPe}
            onChange={onSearchAllowPeChange}
                            label={

<>{t("search.value.pe")}
<span style={{paddingLeft: 4}}></span>
<Tooltip 
                                    withArrow
                                    appearance="inverted"
                                    content={t("search.value.pe.desc")} relationship={"label"}>
<Info16Regular/>
</Tooltip></>


} />
         
          <Divider className="divider" />
                        <Subtitle2 block>{t("search.modifier.title")}</Subtitle2>
                        <Caption1 block>{t("search.modifier.desc")}</Caption1>
                        <ModifierSelection 
                            selectedInclude={searchIncludeModifiers}
                            selectedExclude={searchExcludeModifiers}
                            onSelect={onSelectSearchModifiers} 
                        />
          </div>
          <div >
          <Divider className="divider" />
          <div style={{display: "flex", gap: 10, justifyContent: "end", alignItems: "baseline"}}>
            <Caption1>12817924 Recipes Found.</Caption1>
          <Button appearance="primary"
                                onClick={async () => {

                                    const filter: SearchFilter = {
                                        minValue: searchMinHp,
                                        maxValue: searchMaxHp,
                                        includesModifier: searchIncludeModifiers,
                                        excludesModifier: searchExcludeModifiers,
                                        includeCritRngHp: searchCritRngHp,
                                        includePeOnly: searchAllowPe,
                                    };

                                    const start = performance.now();
                                    const result = await host.search(filter);
                                    console.log(result);
                                    const end = performance.now();
                                    console.log("Search took", end - start, "ms");




                                }}


                            >{t("search.button")}</Button>
          </div>
          
          </div>
          

        </div>
        <div className="container" style={{minWidth: 400}}>
        <div style={{display: "flex", flexDirection: "column", height: "100%"}}>
          <div>
          <Title3 block>
            <Filter24Regular />
            Filter</Title3>
          <Body1 block>Narrow down the results based on items in the recipes</Body1>
          <Divider className="divider" />

          <Body1 block>
            Search for items
            </Body1>
            <SearchBox placeholder="Search for items" 
            style={{width: "100%"}}/>
            <div>

            <Checkbox label="Show items not included in the recipes found" />
          </div>
         
            </div>
            <div style={{display: "flex", flexDirection: "column", flex: 1, minHeight: 0}}>
              <div style={{flex:1, display: "flex", flexDirection: "column", minHeight: 0}}>
                <Subtitle2 block>Included Items</Subtitle2>
                <Caption1 block>77 Items. 23 more not in recipes</Caption1>
                <div style={{flex: 1, minHeight: 0, border: "1px solid #ddd"}}>



                </div>
                
              </div>
              <div style={{flex:1, display: "flex", flexDirection: "column", minHeight: 0}}>
              <Subtitle2 block>Excluded Items</Subtitle2>
              <Caption1 block>77 Items. 23 more not in recipes</Caption1>
              <div style={{flex: 1, overflowY: "auto", border: "1px solid #ddd"}}>

                </div>
              </div>
            </div>
            
            
          </div>
        </div>
        <div className="container" style={{flex: 1}}>
        <div>
          <Title3 block>
            <List24Regular />
            Results</Title3>
          <Divider className="divider" />
          </div>
        </div>
      </div>
      <div style={{position: "fixed", right: 10, top: 10}}>
        <LocalePicker />
      </div>
    </FluentProvider>
  );
}
type Item = {
  modifier: WeaponModifier,
}
const items: Item[] = WeaponModifiers.map(modifier => ({modifier}));

const columns: TableColumnDefinition<Item>[] = [
  createTableColumn<Item>({
    columnId: "modifier",
    renderHeaderCell: () => {
      return "Modifier";
    },
    renderCell: ({modifier}) => {
      return (
        <TableCellLayout media={<Modifier modifier={modifier}/>} >
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
          <Button  aria-label="Edit" icon={<EditRegular />} />
          <Button  aria-label="Delete" icon={<DeleteRegular />} />
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
    <DataGrid
      items={items}
      columns={columns}
    >
      <DataGridHeader>
        <DataGridRow

        >
          {({ renderHeaderCell }) => (
            <DataGridHeaderCell>{renderHeaderCell()}</DataGridHeaderCell>
          )}
        </DataGridRow>
      </DataGridHeader>
      <DataGridBody<Item>>
        {({ item, rowId }) => (
          <DataGridRow<Item>
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

export const FocusableElementsInCells2 = () => {
  return (
    <DataGrid style={{height: "100%", display: "flex", flexDirection: "column"}}
      items={[
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
        {modifier: {label: "Spicy Pepper", icon: "AddLife"}},
      ]}
      columns={columns}
      sortable
      getRowId={(item) => item.modifier.label}
      
    >
      <DataGridHeader>
        <DataGridRow

        >
          {({ renderHeaderCell }) => (
            <DataGridHeaderCell>{renderHeaderCell()}</DataGridHeaderCell>
          )}
        </DataGridRow>
      </DataGridHeader>
     

      <DataGridBody<Item> style={{minHeight: 0, flex: 1, overflowY: "auto"}}>
        {({ item, rowId }) => (
          <DataGridRow<Item>
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
