import { useSelector } from "react-redux";
import { ErrorCircle20Filled, List24Regular } from "@fluentui/react-icons";
import {
    Body1,
    Button,
    Caption1,
    makeStyles,
    mergeClasses,
    MessageBar,
    Spinner,
} from "@fluentui/react-components";
import { useDark } from "@pistonite/pure-react";
import { useTranslation ,getErrorMessage } from "botw-recipe-searcher-localization";

import { StageTitle } from "components/StageTitle.tsx";
import { ResultList } from "components/ResultList.tsx";
import {
    getCookingResults,
    getResultCookingError,
    getResultFilter,
    getResultLimit,
    isResultCookingInProgress,
} from "store/result.ts";
import { getFilterStageDisabledMessage } from "store/selectors.ts";
import { getFavoriteActors, getFilterResultCount } from "store/filter.ts";

const useStyles = makeStyles({
    progressContainer: {
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
        position: "absolute",
        zIndex: 1,
        inset: 0,
        padding: "10px",
    },
    progressContainerLight: {
        backgroundColor: "rgba(255, 255, 255, 0.95)",
    },
    progressContainerDark: {
        backgroundColor: "rgba(0, 0, 0, 0.5)",
    },

    errorContainer: {
        backgroundColor: "#fde7e7",
        padding: "10px",
        borderRadius: "5px",
    },
    errorTextContainer: {
        display: "flex",
        gap: "10px",
    },
    errorIcon: {
        color: "#800",
    },
    errorRetryButton: {
        textAlign: "right",
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
    mainSection: {
        display: "flex",
        flexDirection: "column",
        flex: 1,
        position: "relative",
        overflow: "hidden",
    },
    countMessage: {
        paddingBottom: "4px",
    },
    noteMessage: {
        paddingBottom: "4px",
    },
});

export const ResultStage: React.FC = () => {
    const t = useTranslation();

    return (
        <>
            <StageTitle
                title={t("result.title")}
                icon={List24Regular}
                desc={t("result.desc")}
            />
            <ResultStageBody />
        </>
    );
};

const ResultStageBody: React.FC = () => {
    const t = useTranslation();
    const styles = useStyles();
    const isInProgress = useSelector(isResultCookingInProgress);
    const error = useSelector(getResultCookingError);
    const results = useSelector(getCookingResults);
    const filter = useSelector(getResultFilter);
    const rawResultCount = useSelector(getFilterResultCount);
    const limit = useSelector(getResultLimit);
    const { disabled: stageDisabled } = useSelector(
        getFilterStageDisabledMessage,
    );
    const favorited = useSelector(getFavoriteActors);
    const isDarkMode = useDark();

    if (!results.length && stageDisabled) {
        return (
            <div className={styles.mainMessage}>
                <Body1 block>{t("result.not_ready")}</Body1>
            </div>
        );
    }
    return (
        <div className={styles.mainSection}>
            {error !== undefined && (
                <div className={styles.errorContainer}>
                    <div className={styles.errorTextContainer}>
                        <ErrorCircle20Filled className={styles.errorIcon} />
                        <Body1>
                            {t("result.error", {
                                message: getErrorMessage(error),
                            })}
                        </Body1>
                    </div>
                    <div className={styles.errorRetryButton}>
                        <Button appearance="primary">
                            {t("result.button.retry")}
                        </Button>
                    </div>
                </div>
            )}
            <div className={styles.mainSection}>
                {isInProgress && (
                    <div
                        className={mergeClasses(
                            styles.progressContainer,
                            isDarkMode
                                ? styles.progressContainerDark
                                : styles.progressContainerLight,
                        )}
                    >
                        <Spinner
                            size="huge"
                            labelPosition="below"
                            label={t("result.progress")}
                        />
                    </div>
                )}
                {results.length === 0 ? (
                    <div className={styles.mainMessage}>
                        <Body1 block>{t("result.nothing")}</Body1>
                    </div>
                ) : (
                    <>
                        {rawResultCount > limit ? (
                            <MessageBar intent="warning" layout="multiline">
                                {t("result.limited", {
                                    limit,
                                    count: results.length,
                                })}
                            </MessageBar>
                        ) : (
                            <Body1 className={styles.countMessage} block>
                                {t("result.count", {
                                    limit,
                                    count: results.length,
                                })}
                            </Body1>
                        )}
                        <Caption1 className={styles.noteMessage} block>
                            {t("result.list.note")}
                        </Caption1>
                        <ResultList
                            results={results}
                            favorited={favorited}
                            filter={filter}
                        />
                    </>
                )}
            </div>
        </div>
    );
};
