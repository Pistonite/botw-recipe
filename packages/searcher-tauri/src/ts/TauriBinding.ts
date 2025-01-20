import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { type Result, type Void, errstr } from "@pistonite/pure/result";
import type { Group } from "botw-recipe-sys";

import type { HostBinding } from "./HostBinding.ts";
import type {
    HostError,
    Stats,
    SearchFilter,
    OptimizedRecipeData,
} from "./types.ts";
import { unexpected } from "./types.ts";

/** Binding for Tauri application */
export class TauriBinding implements HostBinding {
    private title: string = "";
    async setTitle(title: string): Promise<void> {
        if (title === this.title) {
            return;
        }
        this.title = title;
        try {
            await invoke("set_title", { title });
        } catch (_) {
            // ignore set title error
        }
    }

    async initialize(): Promise<Void<HostError>> {
        try {
            await invoke("initialize");
            return {};
        } catch (e) {
            console.error(e);
            return { err: unexpected(errstr(e)) };
        }
    }

    async setInitializedHandler(handler: () => void): Promise<void> {
        await listen("initialized", handler);
    }

    async abortSearch(): Promise<Void<HostError>> {
        try {
            return await invoke("abort_search");
        } catch (e) {
            console.error(e);
            return { err: unexpected(errstr(e)) };
        }
    }

    async search(filter: SearchFilter): Promise<Void<HostError>> {
        try {
            return await invoke("search", { filter });
        } catch (e) {
            console.error(e);
            return { err: unexpected(errstr(e)) };
        }
    }

    async setSearchProgressHandler(
        handler: (percentage: number) => void,
    ): Promise<void> {
        await listen("search-progress", ({ payload }) =>
            handler(payload as number),
        );
    }

    async setSearchCompleteHandler(
        handler: (result: Result<Stats, HostError>) => void,
    ): Promise<void> {
        await listen("search-complete", ({ payload }) =>
            handler(payload as Result<Stats, HostError>),
        );
    }

    async abortFilter(): Promise<Void<HostError>> {
        try {
            return await invoke("abort_filter");
        } catch (e) {
            console.error(e);
            return { err: unexpected(errstr(e)) };
        }
    }

    async filter(include: Group[]): Promise<Void<HostError>> {
        try {
            return await invoke("filter", { include });
        } catch (e) {
            console.error(e);
            return { err: unexpected(errstr(e)) };
        }
    }

    async setFilterProgressHandler(
        handler: (percentage: number) => void,
    ): Promise<void> {
        await listen("filter-progress", ({ payload }) =>
            handler(payload as number),
        );
    }

    async setFilterCompleteHandler(
        handler: (result: Result<Stats, HostError>) => void,
    ): Promise<void> {
        await listen("filter-complete", ({ payload }) =>
            handler(payload as Result<Stats, HostError>),
        );
    }

    async cook(): Promise<Void<HostError>> {
        try {
            return await invoke("cook");
        } catch (e) {
            console.error(e);
            return { err: unexpected(errstr(e)) };
        }
    }

    async setCookCompleteHandler(
        handler: (result: Result<OptimizedRecipeData[], HostError>) => void,
    ): Promise<void> {
        await listen("cook-complete", ({ payload }) =>
            handler(payload as Result<OptimizedRecipeData[], HostError>),
        );
    }

    async getResultLimit(): Promise<number> {
        try {
            return await invoke("get_result_limit");
        } catch (e) {
            console.error(e);
            return 5000;
        }
    }
}
