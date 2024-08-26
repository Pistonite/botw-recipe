import { Result, Void } from "@pistonite/pure/result";
import { errstr } from "@pistonite/pure/utils";
import { ActorFilter, FilterComplete, HostBinding, InitArg, SearchComplete, SearchFilter } from "./host";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event"


export class TauriBinding implements HostBinding {
    async initialize(arg: InitArg): Promise<Void<string>> {
        try {
            await invoke("initialize", { arg });
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
    async filterActors(filter: ActorFilter): Promise<Void<string>> {
        try {
            return await invoke("filter_actors", { filter });
        } catch (e) {
            console.error(e);
            return { err: errstr(e) };
        }
    }
    async setInitializedHandler(handler: () => void): Promise<void> {
        await listen("initialized", handler);
    }
    async setSearchCompleteHandler(handler: (result: Result<SearchComplete, string>) => void): Promise<void> {
        await listen("search-complete", ({payload}) => handler(payload as Result<SearchComplete, string>));
    }
    async setFilterCompleteHandler(handler: (result: Result<FilterComplete, string>) => void): Promise<void> {
        await listen("filter-complete", ({payload}) => handler(payload as Result<FilterComplete, string>));
    }

}