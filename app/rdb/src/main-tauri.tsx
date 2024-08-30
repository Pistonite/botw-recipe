import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { Result, Void } from "@pistonite/pure/result";
import { errstr } from "@pistonite/pure/utils";

import { HostBinding } from "host/HostBinding.ts";
import { SearchComplete, SearchFilter } from "host/types.ts";
import { Host } from "host/Host.ts";
import { boot } from "boot.tsx";

class TauriBinding implements HostBinding {
    async setTitle(title: string): Promise<void> {
        try {
            await invoke("set_title", {title});
        } catch (_) { }
    }
    async initialize(): Promise<Void<string>> {
        try {
            await invoke("initialize");
            return {};
        } catch (e) {
            console.error(e);
            return { err: errstr(e) };
        }
    }
    async abort(handle: number): Promise<Void<string>> {
        try {
            return await invoke("abort", { handle });
        } catch (e) {
            console.error(e);
            return { err: errstr(e) };
        }
    }
    async search(filter: SearchFilter): Promise<Result<number[], string>> {
        try {
            return await invoke("search", { filter });            
        } catch (e) {
            console.error(e);
            return { err: errstr(e) };
        }
    }
    // async filterActors(filter: ActorFilter): Promise<Void<string>> {
    //     try {
    //         return await invoke("filter_actors", { filter });
    //     } catch (e) {
    //         console.error(e);
    //         return { err: errstr(e) };
    //     }
    // }
    async setInitializedHandler(handler: () => void): Promise<void> {
        await listen("initialized", handler);
    }
    async setSearchProgressHandler(handler: (percentage: number) => void): Promise<void> {
        await listen("search-progress", ({payload}) => handler(payload as number));
    }
    async setSearchCompleteHandler(handler: (result: Result<SearchComplete, string>) => void): Promise<void> {
        await listen("search-complete", ({payload}) => handler(payload as Result<SearchComplete, string>));
    }
    // async setFilterCompleteHandler(handler: (result: Result<FilterComplete, string>) => void): Promise<void> {
    //     await listen("filter-complete", ({payload}) => handler(payload as Result<FilterComplete, string>));
    // }

}

boot(new Host(new TauriBinding()));
