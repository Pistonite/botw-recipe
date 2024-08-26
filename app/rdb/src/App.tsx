import "./App.css";
import { useTranslation } from "react-i18next";
import { switchLanguage } from "./i18n/initLocale";
import { Avatar, Body1, Button, Caption1, Card, Checkbox, Divider, Field, FluentProvider, Label, SpinButton, Subtitle1, TagPicker, TagPickerControl, TagPickerList, TagPickerOption, Title3, Tooltip, webLightTheme } from "@fluentui/react-components";
import { Info16Regular, Info20Regular } from "@fluentui/react-icons";


export const App: React.FC = () => {


  const {t} = useTranslation();


  return (
    <FluentProvider theme={webLightTheme}>
      <div style={{display: "flex", height: "100vh"}}>
        <div className="container" style={{minWidth: 250, maxWidth: 300, height: "100%", display: "flex", flexDirection: "column"}}>
          <div>
          <Title3 block>Search</Title3>
          <Body1 block>Scan the entire database based on modifier criteria</Body1>
          <Divider className="divider" />
          </div>
          
          <div style={{flex: 1, overflowY: "auto"}}>
          
          <Field label="Min Modifier Value (HP)">
            <SpinButton />
          </Field>
          <Field label="Max Modifier Value (HP)">
            <SpinButton />
          </Field>
          
            <Checkbox label={

                <Label>Allow RNG HP Crit
                <span style={{paddingLeft: 4}}></span>
                <Tooltip content={"Include recipes where a HP crit that is not guaranteed is required to reach the minimum HP. Guaranteed HP crit is not affected"} relationship={"label"}>
              <Info16Regular/>
          </Tooltip></Label>
            
              
            } />
                      <Checkbox label={

<Label>Allow Unusual Materials
<span style={{paddingLeft: 4}}></span>
<Tooltip content={"Include recipes where at least one material requires Prompt Entanglement, or is normally not obtainable or cookable"} relationship={"label"}>
<Info16Regular/>
</Tooltip></Label>


} />
         
          <Divider className="divider" />
          <Field label={

<Label>Include Modifiers
<span style={{paddingLeft: 4}}></span>
<Tooltip content={"Only include recipes that have ALL of the selected modifiers"} relationship={"label"}>
<Info16Regular/>
</Tooltip></Label>


}>
            <TagPicker>
              <TagPickerControl />
              <TagPickerList>
                <TagPickerOption
                  media = {<Avatar name="AttackUp" />}
                  value = "AttackUp"
                  key = "AttackUp"
                >
                  Attack Up
                </TagPickerOption>
              </TagPickerList>
            </TagPicker>
          </Field>
          <Field label={

<Label>Exclude Modifiers
<span style={{paddingLeft: 4}}></span>
<Tooltip content={"Only include recipes that don't have ANY of the selected modifiers"} relationship={"label"}>
<Info16Regular/>
</Tooltip></Label>


}>
            <TagPicker>
              <TagPickerControl />
              <TagPickerList>
                <TagPickerOption
                  media = {<Avatar name="AttackUp" />}
                  value = "AttackUp"
                  key = "AttackUp"
                >
                  Attack Up
                </TagPickerOption>
              </TagPickerList>
            </TagPicker>
          </Field>
          </div>
          <div >
          <Divider className="divider" />
          <div style={{display: "flex", gap: 10, justifyContent: "end"}}>
          <Button appearance="primary">Search</Button>
          <Button>Cancel</Button>
          </div>
          
          </div>
          

        </div>
        <div className="container" style={{minWidth: 400}}>
        <div>
          <Title3 block>Filter</Title3>
          <Body1 block>Narrow down the results based on items in the recipes</Body1>
          <Divider className="divider" />
          </div>
        </div>
        <div className="container" style={{flex: 1}}>
        <div>
          <Title3 block>Results</Title3>
          <Divider className="divider" />
          </div>
        </div>
      </div>
    </FluentProvider>
  );
}
