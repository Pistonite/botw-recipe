import type { DatabaseError, HostError } from "botw-recipe-searcher-tauri";

import { translate } from "./backend.ts";

export const getErrorTitle = (): string => {
    return translate("error.title");
}

export const getErrorButtonText = (): string => {
    return translate("error.button");
}

/**
 * Get localized error message
 */
export const getErrorMessage = (error: HostError): string => {
    const type = error.type;
    switch (error.type) {
        case "IOError":
            return getIOErrorMessage(error.data);
        case "ExecutorUnavailable":
            return translate("error.message.executor.unavailable");
        case "Aborted":
            return translate("error.message.aborted");
        case "DatabaseError":
            return getDatabaseErrorMessage(error.data);
        case "MissingSearchResult":
        case "Unexpected":
            return getInternalErrorMessage(type);
        default:
            return getInternalErrorMessage(type);
    }
};

const getDatabaseErrorMessage = (error: DatabaseError): string => {
    const type = error.type;
    switch (error.type) {
        case "IO":
            return getIOErrorMessage(error.data);
        case "YAML":
            return getYAMLErrorMessage(error.data);
        case "Locked":
            return translate("error.message.database.locked");
        case "MissingIndex":
        case "InvalidIndexChunkCount":
        case "MissingChunk":
        case "InvalidChunkSize":
            return getInvalidDatabaseErrorMessage(type);
        default:
            return getInternalErrorMessage(type);
    }
};

const getInvalidDatabaseErrorMessage = (errorType: string): string => {
    return translate("error.message.database.invalid", { errorType });
};

const getIOErrorMessage = (message: string): string => {
    return translate("error.message.io", { message });
};

const getYAMLErrorMessage = (message: string): string => {
    return translate("error.internal", { message });
};

const getInternalErrorMessage = (message: string): string => {
    return translate("error.internal", { message });
};
