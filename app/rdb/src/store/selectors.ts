import { createSelector } from "@reduxjs/toolkit";

import type { State } from "./store.ts";

export type FilterStageDisabledMessage = {
    disabled: boolean,
    messageId: string | null,
}
/** 
 * If the filter stage can be enabled:
 *
 * - Search must be done (results available)
 * - Must have non-empty results
 * - Search must not be in progress
 * - Filter must not be in progress
 */
export const getFilterStageDisabledMessage = createSelector(
    [
        (state: State) => state.search.searchResultCount,
        (state: State) => state.search.searchProgress,
        (state: State) => state.filter.filterProgress
    ],
    (searchResultCount, searchProgress, filterProgress): FilterStageDisabledMessage => {
        if (searchProgress >= 0) {
            return {
                disabled: true,
                messageId: "filter.not_ready.searching"
            };
        }
        if (searchResultCount === -1) {
            return {
                disabled: true,
                messageId: "filter.not_ready.not_searched"
            };
        }
        if (searchResultCount === 0) {
            return {
                disabled: true,
                messageId: "filter.not_ready.no_result"
            };
        }
        if (filterProgress >= 0) {
            // disable controls instead of showing a message
            return {
                disabled: true,
                messageId: null
            };
        }
        return {
            disabled: false,
            messageId: null
        };
    }
);
