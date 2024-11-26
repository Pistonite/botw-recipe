import Fuse from "fuse.js";
import i18next from "i18next";
import { create } from "zustand";

import { type Actor, ActorToName, getActors } from "data/Actor.ts";

export type ItemSearchFn = (searchText: string) => Actor[] | undefined;

type Store = {
    fn: ItemSearchFn;
};
const store = create<Store>()(() => ({
    fn: () => undefined,
}));
export const useItemSearch = () => {
    return store((state) => state.fn);
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
        "./locales/en-US.yaml"
    );

    // Locale can have extra locale-specific keys.
    // like pinyin for Chinese
    let extraKeys: Record<string, string> = {};
    if (locale === "zh-CN") {
        const { default: keys } = await import("./locales/zh-CN.pinyin.yaml");
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
    store.setState({ fn: itemSearch });
};
