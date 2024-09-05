/**
 * System for side-loading localization to make it easier
 * for translators to test their translations.
 */

let overrideGetter: (() => Promise<string>) | undefined = undefined;

export const setTranslationOverride = (getter: () => Promise<string>) => {
    overrideGetter = getter;
};

export const getTranslationOverride = (): Promise<string> => {
    return overrideGetter ? overrideGetter() : Promise.resolve("");
};
