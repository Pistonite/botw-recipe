import { useTranslation } from "react-i18next";
import {
    Body1,
    Caption1,
    Checkbox,
    SearchBox,
    Subtitle2,
} from "@fluentui/react-components";
import { Filter24Regular } from "@fluentui/react-icons";
import { StageTitle } from "components/StageTitle";
import { ItemActorImg, ItemActorLabel } from "components/Actor";
import { Actor } from "data/Actor";

export const FilterStage: React.FC = () => {
    const { t } = useTranslation();
    return (
        <>
            <div
                style={{
                    display: "flex",
                    flexDirection: "column",
                    height: "100%",
                }}
            >
                <StageTitle
                    title={t("filter.title")}
                    icon={Filter24Regular}
                    desc={t("filter.desc")}
                />
                <div>
                    <Body1 block>Search for items</Body1>
                    <SearchBox
                        placeholder="Search for items"
                        style={{ width: "100%" }}
                    />
                    <div>
                        <Checkbox label="Show excluded items" />
                    </div>
                </div>
                <div
                    style={{
                        display: "flex",
                        flexDirection: "column",
                        flex: 1,
                        minHeight: 0,
                    }}
                >
                    <div
                        style={{
                            flex: 1,
                            display: "flex",
                            flexDirection: "column",
                            minHeight: 0,
                        }}
                    >
                        <Subtitle2 block>Included Items</Subtitle2>
                        <Caption1 block>
                            77 Items. 23 more not in recipes
                        </Caption1>
                        <div
                            style={{
                                flex: 1,
                                minHeight: 0,
                                border: "1px solid #ddd",
                            }}
                        >
                            <div style={{ display: "flex" }}>
                                <ItemActorImg actor={Actor.Item_Fruit_A} />
                                <ItemActorLabel actor={Actor.Item_Fruit_A} />
                                <ItemActorImg actor={Actor.Item_Ore_A} />
                            </div>
                        </div>
                    </div>
                    <div
                        style={{
                            flex: 1,
                            display: "flex",
                            flexDirection: "column",
                            minHeight: 0,
                        }}
                    >
                        <Subtitle2 block>Excluded Items</Subtitle2>
                        <Caption1 block>
                            77 Items. 23 more not in recipes
                        </Caption1>
                        <div
                            style={{
                                flex: 1,
                                overflowY: "auto",
                                border: "1px solid #ddd",
                            }}
                        ></div>
                    </div>
                </div>
            </div>
        </>
    );
};
