import type { Err, Result, Void } from "@pistonite/pure/result";
import { serial } from "@pistonite/pure/sync";
import { ActorToGroup, type Actor } from "botw-recipe-sys";

import type { HostBinding } from "./HostBinding.ts";
import type {
    HostError,
    HostProgressHandler,
    Stats,
    SearchFilter,
    OptimizedRecipeData,
} from "./types.ts";
import { unexpected } from "./types.ts";

/** The application host to execute external code */
export class Host {
    private binding: HostBinding;
    private initializePromise: Promise<Void<HostError>> | undefined = undefined;
    private searchResolve:
        | ((result: Result<Stats, HostError>) => void)
        | undefined = undefined;
    private filterResolve:
        | ((result: Result<Stats, HostError>) => void)
        | undefined = undefined;
    private cookResolve:
        | ((result: Result<OptimizedRecipeData[], HostError>) => void)
        | undefined = undefined;
    private doCook: () => Promise<Result<Result<OptimizedRecipeData[], HostError>, "cancel">>;

    constructor(binding: HostBinding) {
        this.binding = binding;
        this.doCook = serial({
            fn: () => () => {
                return new Promise<Result<OptimizedRecipeData[], HostError>>((resolve) => {
                    this.binding.cook().then((result) => {
                        if ("err" in result) {
                            resolve({
                                err: result.err || unexpected("cook"),
                            });
                            return;
                        }
                        this.cookResolve = resolve;
                    });
                });
            }
        });
    }

    public async bind(
        searchProgressHandler: HostProgressHandler,
        filterProgressHandler: HostProgressHandler,
    ): Promise<void> {
        await Promise.all([
            this.binding.setSearchCompleteHandler((result) => {
                const resolve = this.searchResolve;
                if (resolve) {
                    this.searchResolve = undefined;
                    resolve(result);
                }
            }),
            this.binding.setSearchProgressHandler(searchProgressHandler),

            this.binding.setFilterCompleteHandler((result) => {
                const resolve = this.filterResolve;
                if (resolve) {
                    this.filterResolve = undefined;
                    resolve(result);
                }
            }),
            this.binding.setFilterProgressHandler(filterProgressHandler),

            this.binding.setCookCompleteHandler((result) => {
                const resolve = this.cookResolve;
                if (resolve) {
                    this.cookResolve = undefined;
                    resolve(result);
                }
            }),
        ]);
    }

    public setTitle(title: string): void {
        this.binding.setTitle(title);
    }

    public initialize(): Promise<Void<HostError>> {
        if (this.initializePromise) {
            return this.initializePromise;
        }
        this.initializePromise = new Promise((resolve) => {
            this.binding
                .setInitializedHandler(() => resolve({}))
                .then(() => {
                    this.binding.initialize().then((result) => {
                        if (result.err) {
                            resolve(result);
                        }
                        // wait for initialized event
                    });
                });
        });
        return this.initializePromise;
    }

    public search(filter: SearchFilter): Promise<Result<Stats, HostError>> {
        return new Promise((resolve) => {
            this.binding.search(filter).then((result) => {
                if ("err" in result) {
                    resolve({
                        err: result.err || unexpected("search"),
                    });
                    return;
                }
                this.searchResolve = resolve;
            });
        });
    }

    public cancelSearch(): Promise<Void<HostError>> {
        return this.binding.abortSearch();
    }

    public filter(filter: Actor[]): Promise<Result<Stats, HostError>> {
        const groups = filter.map((actor) => ActorToGroup[actor]);
        return new Promise((resolve) => {
            this.binding.filter(groups).then((result) => {
                if ("err" in result) {
                    resolve({
                        err: result.err || unexpected("filter"),
                    });
                    return;
                }
                this.filterResolve = resolve;
            });
        });
    }

    public cancelFilter(): Promise<Void<HostError>> {
        return this.binding.abortFilter();
    }

    public async cook(): Promise<Result<OptimizedRecipeData[], HostError>> {
        const result = await this.doCook();
        if (result.err === "cancel") {
            // convert to a host error
            return { err: { type: "Aborted" } } satisfies Err<HostError>;
        }
        return result.val;
    }

    public async getResultLimit(): Promise<number> {
        return this.binding.getResultLimit();
    }
}
