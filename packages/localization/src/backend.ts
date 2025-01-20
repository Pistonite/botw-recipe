import type { BackendModule } from "i18next";
import i18next from "i18next";
import {
    initReactI18next,
    useTranslation as useI18nextTranslation,
} from "react-i18next";
import {
    addLocaleSubscriber,
    connectI18next,
    convertToSupportedLocale,
    detectLocale,
    getLocale,
    initLocale,
} from "@pistonite/pure/pref";
import { Host } from "botw-recipe-searcher-tauri";

import { initLocalizedItemSearch } from "./itemSearch.ts";

export const backend: BackendModule = {
    type: "backend",
    init: () => {
        // no init needed
    },
    read: async (language: string, namespace: string) => {
        if (language === "dev") {
            // don't load the default translation namespace
            return undefined;
        }
        const locale = convertToSupportedLocale(language);
        if (namespace === "actor") {
            try {
                const strings = await import(
                    `./actors/${locale}.yaml`
                );
                return strings.default;
            } catch (e) {
                console.error(e);
                return undefined;
            }
        }
        try {
            const strings = await import(`./strings/${locale}.yaml`);
            return strings.default;
        } catch (e) {
            console.error(e);
        }
        return undefined;
    },
};

export const SupportedLocales = [
    "de-DE",
    "en-US",
    "es-ES",
    "fr-FR",
    "it-IT",
    "ja-JP",
    "ko-KR",
    "nl-NL",
    "ru-RU",
    "zh-CN",
    "zh-TW",
] as const;

export const initI18n = async (host: Host) => {
    initLocale({
        supported: SupportedLocales,
        default: "en-US",
        persist: true,
    });

    await i18next.use(detectLocale).use(backend).use(initReactI18next)
    .use(connectI18next)
    .init();

    initLocalizedItemSearch(getLocale());
    // Sync title to localized title
    // however, don't set the title immediately, let the UI do it
    // when it's ready
    addLocaleSubscriber(() => {
        host.setTitle(translate("title"));
    });
};

export const translate = (key: string, options?: Record<string, unknown>) => {
    return i18next.t(key, options);
};
export const translateItem = (key: string) => {
    return i18next.t(`actor:${key}`);
};

export const useTranslation = () => {
    const { t } = useI18nextTranslation();
    return t;
};

export const useItemTranslation = () => {
    const { t } = useI18nextTranslation("actor");
    return t;
};
