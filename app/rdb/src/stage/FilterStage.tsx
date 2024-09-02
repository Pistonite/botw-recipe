import { useCallback, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import {
    Button,
    Caption1,
    Checkbox,
    makeStyles,
    SearchBox,
    Subtitle2,
    Tooltip,
} from "@fluentui/react-components";
import { Filter24Regular, Info20Regular } from "@fluentui/react-icons";

import { StageTitle } from "components/StageTitle.tsx";
import { ItemActorPool, ItemActorSelection } from "components/Actor.tsx";
import { StageAction } from "components/StageAction.tsx";
import { StageDivider } from "components/StageDivider.tsx";
import { Actor, getActors } from "data/Actor.ts";

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
    spaceBelow: {
        paddingBottom: "8px",
    },
});

export const FilterStage: React.FC = () => {
    const [favorited, setFavorited] = useState<Actor[]>([]);
    const [included, setIncluded] = useState<Actor[]>(getActors);

    const toggleFavorited = useCallback((actor: Actor) => {
        setFavorited((prev) => {
            if (prev.includes(actor)) {
                return prev.filter((a) => a !== actor);
            }
            return [...prev, actor];
        });
    }, []);
    const toggleIncluded = useCallback((actor: Actor) => {
        setIncluded((prev) => {
            if (prev.includes(actor)) {
                return prev.filter((a) => a !== actor);
            }
            return [...prev, actor];
        });
    }, []);

    const subtitles = useMemo(() => getActors().map(() => "Test"), []);

    const [searchText, setSearchText] = useState("");

    const styles = useStyles();
    const { t } = useTranslation();
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
                contentAfter={
                    <Tooltip
                        relationship="label"
                        content={t("filter.search.desc")}
                    >
                        <Info20Regular />
                    </Tooltip>
                }
            />
            <Checkbox label={t("filter.hide_excluded")} />
            <div className={styles.mainSection}>
                <ItemActorSelection
                    included={included}
                    favorited={favorited}
                    searchText={searchText}
                    showExcluded={true}
                    actorSubtitles={subtitles}
                    toggleFavorited={toggleFavorited}
                    toggleIncluded={toggleIncluded}
                />
                {favorited.length > 0 && (
                    <div>
                        <StageDivider />
                        <Subtitle2 block className={styles.spaceBelow}>
                            {t("filter.favorited")}
                        </Subtitle2>
                        <ItemActorPool actors={favorited} />
                    </div>
                )}
            </div>
            <StageAction>
                <Caption1>Press Apply </Caption1>
                <Button>Reset</Button>
                <Button appearance="primary">Update</Button>
            </StageAction>
        </>
    );
};
