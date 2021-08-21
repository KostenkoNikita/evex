import {EvexNumber, EVEX_TWO_NUMBERS_PLUS_OPERATOR} from "./math";

const a = EvexNumber.fromString("42");
const b = EvexNumber.fromString("1337");

const sum = EVEX_TWO_NUMBERS_PLUS_OPERATOR.apply(a, b);

console.log(sum.get());