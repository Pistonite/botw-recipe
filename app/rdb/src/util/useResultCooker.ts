import { useCallback } from "react";

import { useHost } from "host/useHost.ts";
import { useDispatch } from "store/hook.ts";
import { finishCooking, startCooking } from "store/result.ts";

export const useResultCooker = () => {
    const host = useHost();
    const dispatch = useDispatch();

    const cook = useCallback(async () => {
        dispatch(startCooking());
        const result = await host.cook();
        if (result.err) {
            if (result.err.type === "Aborted") {
                // ignore
                return;
            }
            if (result.err.type === "MissingSearchResult") {
                // treat as empty
                dispatch(
                    finishCooking({
                        val: [],
                    }),
                );
                return;
            }
        }
        dispatch(finishCooking(result));
    }, [host, dispatch]);

    return cook;
};
