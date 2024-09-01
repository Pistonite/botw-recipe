import Fuse from "fuse.js";
import { useEffect, useState } from "react";

import { Actor, ActorToName, getActors } from "data/Actor.ts";

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
    const entries = getActors().map((actor) => {
        return {
            actor,
            actorName: ActorToName[actor],
            localizedName: translation[`actor.${actor}`],
            englishName: englishTranslation[`actor.${actor}`],
        };
    });
    const fuse = new Fuse(entries, {
        threshold: 0.3,
        keys: ["actorName", "localizedName", "englishName"],
        shouldSort: true,
    });
    itemSearch = (searchText: string) => {
        if (!searchText) {
            return undefined;
        }
        const results = fuse.search(searchText);
        return results.map((result) => result.item.actor);
    };
    console.log("localized item search initialized");
};

export const useItemSearch = () => {
    const [search, setSearch] = useState<ItemSearchFn>(itemSearch);
    useEffect(() => {
        const subscriber = (search: ItemSearchFn) => {
            setSearch(search);
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
