import i18next, { type BackendModule } from "i18next";
import { initReactI18next } from "react-i18next";
import {
    initLocale as initPureLocale,
    convertToSupportedLocaleOrDefault,
    detectLocale,
} from "@pistonite/pure/pref";

import type { Host } from "host/Host.ts";
import { initLocalizedItemSearch } from "./itemSearch.ts";
import {
    getTranslationOverrideResource,
    setTranslationOverride,
} from "./override.ts";

export const SupportedLocales = [
    "de-DE",
    "en-US",
    // "es-ES",
    "fr-FR",
    // "it-IT",
    // "ja-JP",
    // "ko-KR",
    // "nl-NL",
    // "ru-RU",
    "zh-CN",
    // "zh-TW"
] as const;

const OVERRIDE_RESOURCE: Record<
    string,
    { translation: Record<string, string> }
> = {};

export async function initLocale(host: Host) {
    if (await host.getBinding().loadOverrideLocalizationJson()) {
        console.log("using override translation");
        setTranslationOverride(() =>
            host.getBinding().loadOverrideLocalizationJson(),
        );
    }
    const defaultLocale = "en-US";
    initPureLocale({
        supported: SupportedLocales,
        default: defaultLocale,
        persist: true,
    });

    const overrideResource = await getTranslationOverrideResource();
    if (overrideResource) {
        OVERRIDE_RESOURCE[defaultLocale] = overrideResource;
        await i18next.use(initReactI18next).init({
            lng: defaultLocale,
            resources: OVERRIDE_RESOURCE,
        });
        return;
    }

    await i18next.use(detectLocale).use(backend).use(initReactI18next).init();
}

export async function switchLanguage(locale: string, host: Host) {
    const overrideResource = await getTranslationOverrideResource();
    if (overrideResource) {
        OVERRIDE_RESOURCE[locale] = overrideResource;
    }

    i18next.changeLanguage(locale);
    host.setTitle(i18next.t("title"));
    await initLocalizedItemSearch(locale);
}

const backend: BackendModule = {
    type: "backend",
    init: () => {},

    read: async (language: string, namespace: string) => {
        if (namespace !== "translation") {
            return undefined;
        }

        const locale = convertToSupportedLocaleOrDefault(language) || "en-US";
        const module = await import(`./locales/${locale}.yaml`);
        return module.default;
    },
};
