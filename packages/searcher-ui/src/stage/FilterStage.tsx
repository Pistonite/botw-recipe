import { useCallback, useMemo, useState } from "react";
import { useSelector } from "react-redux";
import {
    Body1,
    Button,
    Caption1,
    Checkbox,
    Link,
    makeStyles,
    SearchBox,
    Subtitle2,
    Tooltip,
} from "@fluentui/react-components";
import { Filter24Regular, Info20Regular } from "@fluentui/react-icons";
import type { Actor } from "botw-recipe-sys";
import { useTranslation } from "botw-recipe-searcher-localization";

import { StageTitle } from "components/StageTitle.tsx";
import { ItemActorPool, ItemActorSelection } from "components/ItemActor.tsx";
import { StageAction } from "components/StageAction.tsx";
import { StageDivider } from "components/StageDivider.tsx";
import { useAlert, useConfirm, getErrorAlertPayload } from "components/AlertProvider.tsx";
import { getFilterStageDisabledMessage } from "store/selectors.ts";
import {
    abortFilter,
    clearFavorites,
    finishFilter,
    getActorPercentages,
    getActorSubtitles,
    getFavoriteActors,
    getFilterMessage,
    getIncludedActors,
    isFilterInProgress,
    resetFilter,
    startFilter,
    toggleFavoriteActor,
    toggleIncludedActor,
} from "store/filter.ts";
import { useDispatch } from "store/hook.ts";
import { useHost } from "host/HostContext.ts";
import { useResultCooker } from "util/useResultCooker.ts";

const useStyles = makeStyles({
    search: {
        width: "100%",
    },
    mainSection: {
        display: "flex",
        flexDirection: "column",
        flex: 1,
        // This is needed to avoid feedback look when updating
        // size of item list
        minHeight: 0,
    },
    mainMessage: {
        display: "flex",
        flex: 1,
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        "& > *": {
            textAlign: "center",
        },
    },
    spaceBelow: {
        paddingBottom: "8px",
    },
});

const useLocalizedSubtitles = () => {
    const subtitles = useSelector(getActorSubtitles);
    const t = useTranslation();
    return useMemo(() => {
        return subtitles.map(({ id, values }) => t(id, values));
    }, [subtitles, t]);
};

export const FilterStage: React.FC = () => {
    const { disabled: stageDisabled, messageId: stageMessageId } = useSelector(
        getFilterStageDisabledMessage,
    );
    const included = useSelector(getIncludedActors);
    const favorited = useSelector(getFavoriteActors);
    const subtitles = useLocalizedSubtitles();
    const percentages = useSelector(getActorPercentages);
    const filterMessage = useSelector(getFilterMessage);
    const isFilteringInProgress = useSelector(isFilterInProgress);
    const dispatch = useDispatch();

    const toggleFavorited = useCallback(
        (actor: Actor) => {
            dispatch(toggleFavoriteActor(actor));
        },
        [dispatch],
    );
    const toggleIncluded = useCallback(
        (actor: Actor) => {
            dispatch(toggleIncludedActor(actor));
        },
        [dispatch],
    );

    const [searchText, setSearchText] = useState("");
    const [showOnlyIncluded, setShowOnlyIncluded] = useState(true);

    const styles = useStyles();
    const t = useTranslation();
    const alert = useAlert();
    const confirmClearFavorites = useConfirm(
        t("confirm.message.filter.clear_favorites"),
    );
    const confirmClearFilter = useConfirm(t("confirm.message.filter.reset"));
    const confirmAbortFilter = useConfirm(t("confirm.message.filter.abort"));

    const host = useHost();
    const cook = useResultCooker();
    const [abortInProgress, setAbortInProgress] = useState(false);
    const filterHandler = async () => {
        if (isFilteringInProgress) {
            if (abortInProgress) {
                return;
            }
            if (!(await confirmAbortFilter())) {
                return;
            }
            setAbortInProgress(true);
            const result = await host.cancelFilter();
            setAbortInProgress(false);
            if (result.err) {
                await alert(getErrorAlertPayload(result.err));
            }
            return;
        }
        if (stageDisabled) {
            return;
        }
        const startTime = performance.now();
        dispatch(startFilter());
        setAbortInProgress(false);
        const result = await host.filter(included);
        if (result.err) {
            if (result.err.type === "Aborted") {
                console.log("filter aborted");
                dispatch(abortFilter());
                return;
            }
            dispatch(
                finishFilter({
                    duration: "0",
                    foundCount: -1,
                    groupStat: null,
                    isFromSearch: false,
                }),
            );
            await alert(getErrorAlertPayload(result.err));
            return;
        }
        const endTime = performance.now();
        const elapsed = ((endTime - startTime) / 1000).toFixed(2);
        dispatch(
            finishFilter({
                duration: elapsed,
                isFromSearch: false,
                ...result.val,
            }),
        );
        cook();
    };

    return (
        <>
            <StageTitle
                title={t("filter.title")}
                icon={Filter24Regular}
                desc={t("filter.desc")}
            />
            <Subtitle2>{t("filter.list.title")}</Subtitle2>
            <Caption1 className={styles.spaceBelow}>
                {t("filter.list.desc")}
            </Caption1>
            <SearchBox
                value={searchText}
                className={styles.search}
                placeholder={t("filter.search.placeholder")}
                onChange={(_, data) => {
                    setSearchText(data.value);
                }}
                disabled={stageDisabled}
                contentAfter={
                    <Tooltip
                        relationship="label"
                        content={t("filter.search.desc")}
                    >
                        <Info20Regular />
                    </Tooltip>
                }
            />
            <Checkbox
                checked={showOnlyIncluded}
                label={t("filter.hide_excluded")}
                disabled={stageDisabled}
                onChange={(_, data) => setShowOnlyIncluded(!!data.checked)}
            />
            <div className={styles.mainSection}>
                {stageMessageId ? (
                    <div className={styles.mainMessage}>
                        <Body1 block>{t(stageMessageId)}</Body1>
                    </div>
                ) : (
                    <ItemActorSelection
                        included={included}
                        favorited={favorited}
                        disabled={stageDisabled}
                        searchText={searchText}
                        showExcluded={!showOnlyIncluded}
                        actorSubtitles={subtitles}
                        actorPercentages={percentages}
                        toggleFavorited={toggleFavorited}
                        toggleIncluded={toggleIncluded}
                    />
                )}
                {favorited.length > 0 && (
                    <div>
                        <StageDivider />
                        <Subtitle2 block className={styles.spaceBelow}>
                            {t("filter.favorited")}
                            <span
                                aria-hidden
                                role="presentation"
                                style={{ display: "inline-block", minWidth: 8 }}
                            />
                            <Link
                                onClick={async () => {
                                    if (await confirmClearFavorites()) {
                                        dispatch(clearFavorites());
                                    }
                                }}
                            >
                                {t("filter.favorited.clear")}
                            </Link>
                        </Subtitle2>
                        <ItemActorPool
                            actors={favorited}
                            disabled={stageDisabled}
                            included={included}
                        />
                    </div>
                )}
            </div>
            <StageAction>
                <Caption1>
                    {(!stageDisabled || isFilteringInProgress) &&
                        !!filterMessage.id &&
                        t(filterMessage.id, filterMessage.values)}
                </Caption1>
                <Button
                    disabled={stageDisabled}
                    onClick={async () => {
                        if (await confirmClearFilter()) {
                            dispatch(resetFilter());
                        }
                    }}
                >
                    {t("filter.button.reset")}
                </Button>
                <Button
                    disabled={stageDisabled && !isFilteringInProgress}
                    appearance="primary"
                    onClick={filterHandler}
                >
                    {isFilteringInProgress
                        ? t("filter.button.cancel")
                        : t("filter.button")}
                </Button>
            </StageAction>
        </>
    );
};
