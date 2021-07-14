#[inline(always)]
pub fn is_digit(c: &char) -> bool {
    return c.is_digit(10);
}

#[inline(always)]
pub fn is_opening_bracket(c: &char) -> bool {
    return *c == '(';
}

#[inline(always)]
pub fn is_closing_bracket(c: &char) -> bool {
    return *c == ')';
}

#[inline(always)]
pub fn is_separator(c: &char) -> bool {
    return *c == ',';
}

#[inline(always)]
pub fn is_dot(c: &char) -> bool {
    return *c == '.';
}

#[inline(always)]
pub fn is_factorial(c: &char) -> bool {
    return *c == '!';
}

#[inline(always)]
pub fn is_plus_or_minus(c: &char) -> bool {
    let _c = *c;
    return _c == '+' || _c == '-';
}

#[inline(always)]
pub fn is_operator(c: &char) -> bool {
    let _c = *c;
    return _c == '+' ||
        _c == '-' ||
        _c == '*' ||
        _c == '/' ||
        _c == '^' ||
        _c == '%' ||
        is_factorial(c);
}

#[inline(always)]
pub fn is_digit_or_dot(c: &char) -> bool {
    return is_digit(c) || is_dot(c);
}

#[inline(always)]
pub fn is_exponent(c: &char) -> bool {
    let _c = *c;
    return _c == 'e' || _c == 'E';
}
