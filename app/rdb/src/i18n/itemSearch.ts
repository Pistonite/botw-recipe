import Fuse from "fuse.js";
import { useEffect, useState } from "react";

import { type Actor, ActorToName, getActors } from "data/Actor.ts";

import { loadLocale } from "./locales.ts";

export type ItemSearchFn = (searchText: string) => Actor[] | undefined;

let currentLocale = "";
let itemSearch: ItemSearchFn = getActors;
const subscribers: ((search: ItemSearchFn) => void)[] = [];

export const initLocalizedItemSearch = async (
    locale: string,
    translation: Record<string, string>,
) => {
    if (currentLocale === locale) {
        return;
    }
    console.log("initializing localized item search for locale " + locale);
    currentLocale = locale;
    const englishTranslation = await loadLocale("en-US");
    let extraKeys: Record<string, string> = {};
    if (locale === "zh-CN") {
        // load pinyin keys
        const { default: keys } = await import("./locales/zh-CN.pinyin.yaml");
        extraKeys = keys;
    }
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
            localizedName: translation[translationKey],
            englishName: englishTranslation[translationKey],
            keys,
        };
    });
    const fuse = new Fuse(entries, {
        threshold: 0.2,
        keys: ["actorName", "localizedName", "englishName", "keys"],
        shouldSort: true,
    });
    itemSearch = (searchText: string) => {
        if (!searchText) {
            return undefined;
        }
        const results = fuse.search(searchText);
        return results.map((result) => result.item.actor);
    };
    subscribers.forEach((subscriber) => subscriber(itemSearch));
    console.log("localized item search initialized");
};

export const useItemSearch = () => {
    const [search, setSearch] = useState<ItemSearchFn>(() => itemSearch);
    useEffect(() => {
        setSearch(() => itemSearch);
        const subscriber = (search: ItemSearchFn) => {
            setSearch(() => search);
        };
        subscribers.push(subscriber);
        return () => {
            const index = subscribers.indexOf(subscriber);
            if (index !== -1) {
                subscribers.splice(index, 1);
            }
        };
    }, []);
    return search;
};
