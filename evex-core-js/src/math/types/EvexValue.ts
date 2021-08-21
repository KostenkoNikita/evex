import {EvexObject} from "../../types";

export abstract class EvexValue<T> extends EvexObject {
    protected readonly _: T;

    protected constructor(v: T) {
        super();

        this._ = v;
    }

    get(): T {
        return this._;
    }

    toString(): string {
        return String(this._);
    }
}