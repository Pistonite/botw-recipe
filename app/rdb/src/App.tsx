import "./App.css";
import { Body1, Button, Caption1, Checkbox, createTableColumn, DataGrid, DataGridBody, DataGridCell, DataGridCellFocusMode, DataGridHeader, DataGridHeaderCell, DataGridRow, Divider, SearchBox, Subtitle2, TableCellLayout, TableColumnDefinition, TableColumnId, Title3, } from "@fluentui/react-components";
import { DeleteRegular, EditRegular, Filter24Regular, List24Regular } from "@fluentui/react-icons";
import { WeaponModifier, WeaponModifiers } from "data/WeaponModifier";
import { Modifier, ModifierLabel } from "components/Modifier";
import { LocalePicker } from "components/LocalePicker";
import { SearchStage } from "stage/SearchStage.tsx";

export const App: React.FC = () => {

  return <>
      <div style={{display: "flex", height: "100vh"}}>
        <div className="container" style={{minWidth: 300, maxWidth: 360, height: "100%", display: "flex", flexDirection: "column"}}>
                <SearchStage />
          

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
  </>;
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
