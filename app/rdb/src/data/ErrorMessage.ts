import i18n from "i18next";

import type { CookError, DatabaseError, HostError } from "host/types.ts";
import type { AlertPayload } from "components/AlertProvider.tsx";

export const getErrorAlertPayload = (error: HostError): AlertPayload => {
    return {
        title: i18n.t("error.title"),
        message: getErrorMessage(error),
        actions: [i18n.t("error.button")],
    };
};

export const createGenericError = (message: string): HostError => {
    return {
        type: "Generic",
        data: message,
    };
};

export const createUnknownError = (message: string): HostError => {
    return {
        type: "Generic",
        data: getUnknownErrorMessage(message),
    };
};

/**
 * Get localized error message
 */
export const getErrorMessage = (error: HostError): string => {
    const type = error.type;
    switch (error.type) {
        case "IOError":
            return getIOErrorMessage(error.data);
        case "PoisonError":
            return i18n.t("error.message.poison", { message: error.data });
        case "ExecutorUnavailable":
            return i18n.t("error.message.executor.unavailable");
        case "DatabaseError":
            return getDatabaseErrorMessage(error.data);
        case "Generic":
            return i18n.t("error.generic", { message: error.data });
        default:
            return getUnknownErrorMessage(type);
    }
};

const getDatabaseErrorMessage = (error: DatabaseError): string => {
    const type = error.type;
    switch (error.type) {
        case "IO":
            return getIOErrorMessage(error.data);
        case "YAML":
            return getYAMLErrorMessage(error.data);
        case "Cooking":
            return getCookingErrorMessage(error.data);
        case "Locked":
            return i18n.t("error.message.database.locked");
        case "MissingIndex":
        case "InvalidIndexChunkCount":
        case "MissingChunk":
        case "InvalidChunkSize":
            return getInvalidDatabaseErrorMessage(type);
        case "InvalidRecipeId":
            return getInvalidRecipeIdErrorMessage(error.data);
        default:
            return getInternalErrorMessage(type);
    }
};

const getCookingErrorMessage = (error: CookError): string => {
    switch (error.type) {
        case "Yaml":
            return getYAMLErrorMessage(error.data);
        case "InvalidRecipeId":
            return getInvalidRecipeIdErrorMessage(error.data);
        default:
            return getInternalErrorMessage(error.type);
    }
};

const getInvalidDatabaseErrorMessage = (errorType: string): string => {
    return i18n.t("error.message.database.invalid", { errorType });
};

const getInvalidRecipeIdErrorMessage = (id: number): string => {
    return i18n.t("error.message.database.invalid_recipe", { id });
};

const getIOErrorMessage = (message: string): string => {
    return i18n.t("error.message.io", { message });
};

const getYAMLErrorMessage = (message: string): string => {
    return i18n.t("error.message.yaml", { message });
};

const getInternalErrorMessage = (message: string): string => {
    return i18n.t("error.internal", { message });
};

const getUnknownErrorMessage = (message: string): string => {
    return i18n.t("error.unknown", { message });
};
