import i18n from "i18next";
import { initReactI18next } from "react-i18next";

const RESOURCES: Record<string, {translation: Record<string, string>}> = {};

export async function initLocale() {
    const locale = getLocale();
    await loadLocale(locale);

        i18n.use(initReactI18next)
        .init({
            resources: RESOURCES,
            lng: locale,
            
        });
        return locale;
}

export async function switchLanguage(locale: string) {
    console.log("switching to "+ locale);
    if (!RESOURCES[locale]) {
        await loadLocale(locale);
    }
    i18n.changeLanguage(locale);
}

async function loadLocale(locale: string) {
    const module = await import(`./locales/${locale}.json`);
    RESOURCES[locale] = {translation: module};
}



const SUPPORTED_LOCALES = ["en-US", "zh-CN"];

function getLocale(): string {
    const locale = getBrowserLocale();
    if (SUPPORTED_LOCALES.includes(locale)) {
        return locale;
    }
    return "en-US";
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
    return "en-US";
}

