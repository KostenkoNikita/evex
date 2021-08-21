import {Nullable} from "../types";

export abstract class EvexError extends Error {

    readonly originalError: Nullable<Error>;

    readonly message: string;

    protected constructor(origErr?: Error, message?: string) {
        const msg = message ?? origErr?.message ?? "An internal error occurred";

        super(msg);

        this.originalError = origErr ?? null;
        this.message = msg;
    }

    toString() {
        return this.message;
    }

}