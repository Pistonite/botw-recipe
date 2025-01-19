import Fuse from "fuse.js";
import i18next from "i18next";
import { cell } from "@pistonite/pure/sync";
import { useSyncExternalStore } from "react";

import { type Actor, ActorToName, getActors } from "botw-recipe-sys";

export type ItemSearchFn = (searchText: string) => Actor[] | undefined;

const searchFn = cell<ItemSearchFn>({
    initial: () => undefined,
});

export const useItemSearch = () => {
    return useSyncExternalStore(
        (x) => searchFn.subscribe(x),
        () => searchFn.get(),
    );
};

let currentLocale = "";

export const initLocalizedItemSearch = async (locale: string) => {
    if (currentLocale === locale) {
        return;
    }
    console.log("initializing localized item search for locale " + locale);
    currentLocale = locale;

    // Searching item by its English name is always supported
    const { default: englishTranslation } = await import(
        "botw-recipe-sys/i18n/en-US.yaml"
    );

    // Locale can have extra locale-specific keys.
    // like pinyin for Chinese
    let extraKeys: Record<string, string> = {};
    if (locale === "zh-CN") {
        const { default: keys } = await import(
            "botw-recipe-sys/i18n/zh-CN.pinyin.yaml"
        );
        extraKeys = keys;
    }

    // Process the items
    const entries = getActors().map((actor) => {
        const actorName = ActorToName[actor];
        const translationKey = `actor.${actorName}`;
        const keys: string[] = [];
        if (locale === "zh-CN") {
            const words = extraKeys[translationKey + ".full"];
            const initials = extraKeys[translationKey + ".initials"];
            if (words) {
                keys.push(words);
            }
            if (initials) {
                keys.push(initials);
            }
        }
        return {
            actor,
            actorName,
            localizedName: i18next.t(translationKey),
            englishName: englishTranslation[translationKey],
            keys,
        };
    });

    // Initialize search engine
    const fuse = new Fuse(entries, {
        threshold: 0.2,
        keys: ["actorName", "localizedName", "englishName", "keys"],
        shouldSort: true,
    });

    const itemSearch = (searchText: string) => {
        if (!searchText) {
            return undefined;
        }
        const results = fuse.search(searchText);
        return results.map((result) => result.item.actor);
    };
    console.log("localized item search initialized");
    searchFn.set(itemSearch);
};
