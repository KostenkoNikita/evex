export function isValidNumber(arg: any): boolean {
    return !isNaN(parseFloat(arg)) && isFinite(arg);
}