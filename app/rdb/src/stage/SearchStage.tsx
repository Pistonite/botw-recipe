import { useCallback, useState } from "react";
import { useTranslation } from "react-i18next";
import { Button, Caption1, Checkbox, Field, SpinButton, Subtitle2, Tooltip } from "@fluentui/react-components";
import { Info16Regular, Search24Regular } from "@fluentui/react-icons";

import { useAlert } from "components/AlertProvider.tsx";
import { ModifierSelection } from "components/Modifier.tsx";
import { StageDivider } from "components/StageDivider.tsx";
import { StageTitle } from "components/StageTitle.tsx";
import { useDispatch, useSelector } from "store/hook.ts";
import { finishSearch, getSearchFilter, getSearchMessage, isSearching, setSearchIncludeCritRngHp, setSearchIncludePeOnly, setSearchMaxValue, setSearchMinValue, setSearchModifiers, startSearch } from "store/search.ts";
import { WeaponModifierSet } from "host/types.ts";
import { useHost } from "host/useHost.ts";
import { getErrorAlertPayload } from "data/ErrorMessage.ts";

function parseHp(value: number | undefined | null, displayValue: string | undefined): number {
    if (value !== undefined) {
        return Math.min(120, Math.max(0, value || 0));
    }
    if (displayValue !== undefined) {
        const parsedValue = parseInt(displayValue);
        if (Number.isNaN(parsedValue)) {
            return 0;
        }
        return Math.min(120, Math.max(0, parsedValue || 0));
    }
    return 0;
}

export const SearchStage: React.FC = () => {
    const filter = useSelector(getSearchFilter);
    const searchMessage = useSelector(getSearchMessage);
    const isSearchInProgress = useSelector(isSearching);
    const [abortInProgress, setAbortInProgress] = useState(false);
    const dispatch = useDispatch();
    const onSelectSearchModifiers = useCallback((include: WeaponModifierSet, exclude: WeaponModifierSet) => {
        dispatch(setSearchModifiers({include, exclude}));
    }, [dispatch]);
    const host = useHost();
    const alert = useAlert();

    const {t} = useTranslation();

    const searchHandler = useCallback(async () => {
        if (isSearchInProgress) {
            if (abortInProgress) {
                return;
            }
            const confirmAction = await alert({
                title: t("confirm.title"),
                message: t("confirm.message.search.abort"),
                actions: [t("confirm.button.no"), t("confirm.button.yes")]
            });
            console.log("confirmAction", confirmAction);
            if (confirmAction === 0) {
                // no
                return;
            }
            console.log("sending abort signal");
            setAbortInProgress(true);
            const result = await host.cancelSearch();
            if (result.err) {
                await alert(getErrorAlertPayload(result.err));
            }
            return;
        }
        const startTime = performance.now();
        dispatch(startSearch());
        const result = await host.search(filter);
        setAbortInProgress(false);
        if (result.err) {
            dispatch(finishSearch({
                duration: "0",
                foundCount: -1,
                groupStat: null
            }));
            if (result.err.type === "Aborted") {
                console.log("search aborted");
                return;
            }
            await alert(getErrorAlertPayload(result.err));
            return;
        }
        const endTime = performance.now();
        const elapsed = ((endTime - startTime)/1000).toFixed(2);
        dispatch(finishSearch({
            duration: elapsed,
            ...result.val,
        }));

    }, [abortInProgress, isSearchInProgress, filter, host, alert, dispatch, t]);

    return <>
        <StageTitle title={t("search.title")} icon={Search24Regular} desc={t("search.desc")} />
        <div style={{flex: 1, overflowY: "auto"}}>
            <Subtitle2 block> {t("search.value.title")} </Subtitle2>
            <Field label={t("search.value.min")}>
                <SpinButton
                    value={filter.minValue}
                    min={0}
                    max={120}
                    onChange={(_, data) => {
                        dispatch(setSearchMinValue(parseHp(data.value, data.displayValue)));
                    }}
                />
            </Field>
            <Field label={t("search.value.max")}>
                <SpinButton 
                    value={filter.maxValue}
                    min={0}
                    max={120}
                    onChange={(_, data) => {
                        dispatch(setSearchMaxValue(parseHp(data.value, data.displayValue)));
                    }}
                />
            </Field>
            <Checkbox 
                checked={filter.includeCritRngHp}
                onChange={(_, data) => {
                    dispatch(setSearchIncludeCritRngHp(data.checked === true));
                }}
                label={
                    <span style={{display: "inline-flex", gap: 4, alignItems: "center"}}>
                        {t("search.value.rng")}
                        <Tooltip appearance="inverted" withArrow content={t("search.value.rng.desc")} relationship={"label"}>
                            <Info16Regular/>
                        </Tooltip>
                    </span>
                }
            />
            <Checkbox 
                checked={filter.includePeOnly}
                onChange={(_, data) => {
                    dispatch(setSearchIncludePeOnly(data.checked === true));
                }}
                label={
                    <>{t("search.value.pe")}
                        <span style={{paddingLeft: 4}}></span>
                        <Tooltip 
                            withArrow
                            appearance="inverted"
                            content={t("search.value.pe.desc")} relationship={"label"}>
                            <Info16Regular/>
                        </Tooltip>
                    </>
                } 
            />

            <StageDivider />
            <Subtitle2 block>{t("search.modifier.title")}</Subtitle2>
            <Caption1 block>{t("search.modifier.desc")}</Caption1>
            <ModifierSelection 
                selectedInclude={filter.includesModifier}
                selectedExclude={filter.excludesModifier}
                onSelect={onSelectSearchModifiers} 
            />
        </div>
        <div>
            <StageDivider />
            <div style={{display: "flex", gap: 10, justifyContent: "end", alignItems: "baseline"}}>
                <Caption1>{searchMessage.id && t(searchMessage.id, searchMessage.values)}</Caption1>
                <Button 
                    appearance="primary" 
                    onClick={searchHandler} 
                >
                    {isSearchInProgress ? t("search.button.cancel") : t("search.button")}
                </Button>
            </div>
        </div>
    </>;
}
