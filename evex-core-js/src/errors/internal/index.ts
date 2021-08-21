import {EvexError} from "../EvexError";

export class EvexNumberParsingError extends EvexError {

    readonly causedBy: string;

    constructor(s: string, origErr?: Error) {
        const msg = `An error occurred during the parsing of a "${s}" string as a number`;

        super(origErr, msg);

        this.causedBy = s;
    }

}

export class EvexInvalidArgumentLengthError extends EvexError {

    readonly expectedLength: number;

    readonly actualLength: number;

    constructor(expected: number, actual: number) {
        const msg = `Invalid number of arguments: ${expected} expected, but was ${actual}`;

        super(undefined, msg);

        this.expectedLength = expected;
        this.actualLength = actual;
    }

}