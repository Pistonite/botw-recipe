/**
 * System for side-loading localization to make it easier
 * for translators to test their translations.
 */

let overrideGetter: (() => Promise<string>) = async () => "";

export const setTranslationOverride = (getter: () => Promise<string>) => {
    overrideGetter = getter;
};

export const getTranslationOverrideResource = async (): Promise<{
    translation: Record<string, string>;
} | undefined > => {
    const override = await getTranslationOverride();
    if (!override) {
        return undefined;
    }
    let translation: Record<string, string> = {};
    try {
        translation = JSON.parse(override);
    } catch (e) {
        console.error("failed to parse override translation", e);
    }
    return { translation };
}

export const getTranslationOverride = (): Promise<string> => {
    return overrideGetter();
};
