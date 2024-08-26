import { Result, Void } from "@pistonite/pure/result";
import { ActorFilter, FilterComplete, Host, HostBinding, InitArg, SearchComplete, SearchFilter } from "./host";

export class HostImpl implements Host {
    private binding: HostBinding;
    private initializePromise: Promise<Void<string>> | undefined = undefined;
    private searchHandles: number[] = [];
    private searchResolve: ((result: Result<SearchComplete, string>) => void) | undefined = undefined;
    private filterResolve: ((result: Result<FilterComplete, string>) => void) | undefined = undefined;
    private filterPromise: Promise<Result<FilterComplete, string>> | undefined = undefined;

    constructor(binding: HostBinding) {
        this.binding = binding;
    }
    public async bind(): Promise<void> {
        this.binding.setSearchCompleteHandler((result) => {
            const resolve = this.searchResolve;
            if (resolve) {
                this.searchResolve = undefined;
                if (result.err) {
                    // abort if an error is received
                    this.cancelSearch().then(() => resolve(result));
                } else {
                    resolve(result);
                }
            }
        });

        this.binding.setFilterCompleteHandler((result) => {
            const resolve = this.filterResolve;
            if (resolve) {
                this.filterResolve = undefined;
                this.filterPromise = undefined;
                resolve(result);
            }
        });
    }

    public initialize(arg: InitArg): Promise<Void<string>> {
        if (this.initializePromise) {
            return this.initializePromise;
        }
        this.initializePromise = new Promise((resolve) => {
            this.binding.setInitializedHandler(() => resolve({}))
            .then(() => {
                this.binding.initialize(arg).then((result) => {
                    if (result.err) {
                        resolve(result);
                    }
                    // wait for initialized event
                })
            })
        });
        return this.initializePromise;
    }
    public async search(filter: SearchFilter): Promise<Result<SearchComplete, string>> {
        const cancelResult = await this.cancelSearch();
        if (cancelResult.err) {
            return cancelResult;
        }
        return await new Promise((resolve) => {
            this.searchResolve = resolve;
            this.binding.search(filter).then((result) => {
                if ("err" in result) {
                    resolve({ err: result.err || "unknown error" });
                    return;
                }
                this.searchHandles = result.val;
                
            });
        });
    }
    public async cancelSearch(): Promise<Void<string>> {
        const handles = this.searchHandles;
        this.searchHandles = [];
        const promises = handles.map((handle) => this.binding.abort(handle));
        const results = await Promise.all(promises);
        const err = results.find((result) => "err" in result);
        if (err) {
            return err;
        }
        return {};
    }
    public filterActors(filter: ActorFilter): Promise<Result<FilterComplete, string>> {
        if (this.filterPromise) {
            return this.filterPromise;
        }
        this.filterPromise = new Promise((resolve) => {
            this.filterResolve = resolve;
            this.binding.filterActors(filter).then((result) => {
                if (result.err) {
                    resolve({ err: result.err });
                }
                // wait for filter complete event
            });
        });
        return this.filterPromise;
    }
    
}