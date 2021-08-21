import {isValidNumber} from "../../utils";
import {EvexInvalidArgumentLengthError, EvexNumberParsingError} from "../../errors";
import {EvexObject} from "../../types";
import {EvexValue} from "./EvexValue";

export {EvexValue} from "./EvexValue";

export interface ICalculable<T> {

    add(other: EvexValue<T>): EvexValue<T>;

}

export type AnyEvexValue = EvexValue<any>;

export class EvexNumber extends EvexValue<number> implements ICalculable<number> {

    static fromString(s: string) {
        if(!isValidNumber(s)) {
            throw new EvexNumberParsingError(s);
        }

        return new EvexNumber(+s);
    }

    add(other: EvexNumber): EvexNumber {
        return new EvexNumber(this._ + other._);
    }

}

export enum EvexOperatorAssociativity {
    LEFT = "LEFT",
    RIGHT = "RIGHT",
}

export abstract class EvexOperator extends EvexObject {

    protected readonly _content: string;

    protected readonly _associativity: EvexOperatorAssociativity;

    protected readonly _arity: number;

    protected constructor(content: string, associativity: EvexOperatorAssociativity, arity: number) {
        super();

        this._content = content;
        this._arity = arity;
        this._associativity = associativity;
    }

    apply(...args: AnyEvexValue[]): AnyEvexValue {
        if(args.length !== this._arity) {
            throw new EvexInvalidArgumentLengthError(this._arity, args.length);
        }

        return this.fn(...args);
    }

    protected abstract fn(...args: AnyEvexValue[]): AnyEvexValue;

    toString() {
        return this._content;
    }

}

export abstract class EvexBinaryOperator extends EvexOperator {

    protected constructor(content: string, associativity: EvexOperatorAssociativity) {
        super(content, associativity, 2);
    }

    protected abstract fn(arg1: AnyEvexValue, arg2: AnyEvexValue): AnyEvexValue;

}

export class EvexSameTypesAdditionOperator<TBase, T extends EvexValue<TBase> & ICalculable<TBase>> extends EvexBinaryOperator {

    constructor() {
        super(
            "+",
            EvexOperatorAssociativity.LEFT,
        );
    }

    protected fn(arg1: T, arg2: T): EvexValue<TBase> {
        return arg1.add(arg2);
    }

}