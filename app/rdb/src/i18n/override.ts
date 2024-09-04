/**
 * System for side-loading localization to make it easier
 * for translators to test their translations.
 */

let override: Record<string, string> | undefined = undefined;

export const setTranslationOverride = (jsonString: string) => {
    if (jsonString) {
        console.log("using side-loaded translation");
        override = JSON.parse(jsonString);
    }
};

export const getTranslationOverride = () => {
    return override;
};
