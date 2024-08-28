import i18n from "i18next";
import { initReactI18next } from "react-i18next";
export const DefaultLocale = "en-US" as const;

export const SupportedLocales = {
    "en-US": "English",
    "zh-CN": "\u7b80\u4f53\u4e2d\u6587",
} as const;

export function saveLocalePreference(locale: string) {
    localStorage.setItem("Locale", locale);
}

export function loadLocalePreference(): string {
    const locale = localStorage.getItem("Locale");
    if (locale && locale in SupportedLocales) {
        return locale;
    }
    return getLocale();
}

function getLocale(): string {
    const locale = getBrowserLocale();
    if (locale in SupportedLocales) {
        return locale;
    }
    return DefaultLocale;
}

function getBrowserLocale(): string {
    if (window.Intl) {
        try {
            return Intl.NumberFormat().resolvedOptions().locale;
        } catch (_) {
        }
    }
    if (navigator.languages) {
        return navigator.languages[0];
    }
    return DefaultLocale;
}

const RESOURCES: Record<string, {translation: Record<string, string>}> = {};

export async function initLocale() {
    const locale = loadLocalePreference();
    await loadLocale(locale);

        i18n.use(initReactI18next)
        .init({
            resources: RESOURCES,
            lng: locale,
            
        });
        return locale;
}

export async function switchLanguage(locale: string) {
    if (!RESOURCES[locale]) {
        await loadLocale(locale);
    }
    i18n.changeLanguage(locale);
    saveLocalePreference(locale);
}

async function loadLocale(locale: string) {
    const module = await import(`./locales/${locale}.yaml`);
    RESOURCES[locale] = {translation: module.default};
}

