#!/usr/bin/env python3
import argparse
import random
import re

INT_LEN = 36
FRAC_PUBLIC_LEN = 35
FRAC_SHADOW_LEN = 9
FRAC_INTERNAL_LEN = FRAC_PUBLIC_LEN + FRAC_SHADOW_LEN
TOTAL_LEN = INT_LEN + FRAC_INTERNAL_LEN

DIGIT_TOKEN = {
    -5: "NEG_FIVE",
    -4: "NEG_FOUR",
    -3: "NEG_THREE",
    -2: "NEG_TWO",
    -1: "NEG_ONE",
    0: "ZERO",
    1: "ONE",
    2: "TWO",
    3: "THREE",
    4: "FOUR",
}

NUMERIC_RE = re.compile(r"^[+-]?(?:\d+\.?\d*|\.\d+)(?:[eE][+-]?\d+)?$")


def normalize_numeric_literal(raw: str) -> str:
    text = raw.strip()
    if not text:
        raise ValueError("empty numeric literal")
    if "…" in text or "..." in text:
        raise ValueError("truncated literal (ellipsis found)")

    text = (
        text.replace("−", "-")
        .replace("﹣", "-")
        .replace("－", "-")
        .replace("＋", "+")
        .replace(",", "")
        .replace("_", "")
    )
    text = "".join(text.split())

    # Wolfram precision marks, e.g. 3.14`50 or 3.14`*^10.
    text = re.sub(r"`(?:\d+(?:\.\d*)?|\.\d+)?", "", text)
    # Wolfram scientific notation variants.
    text = re.sub(r"\*\^([+-]?\d+)", r"e\1", text)
    text = re.sub(r"(?i)(?:×|x|·)\s*10\^([+-]?\d+)", r"e\1", text)

    return text


def split_leading_sign(text: str):
    if text.startswith("+"):
        return False, text[1:]
    if text.startswith("-"):
        return True, text[1:]
    return False, text


def increment_decimal_digits_in_place(digits):
    for idx in range(len(digits) - 1, -1, -1):
        if digits[idx] < 9:
            digits[idx] += 1
            return False
        digits[idx] = 0
    return True


def round_half_up_decimal_buffers(int_digits, frac_digits, round_digit: int):
    if round_digit < 5:
        return False
    if increment_decimal_digits_in_place(frac_digits):
        return increment_decimal_digits_in_place(int_digits)
    return False


def parse_decimal_parts(body: str):
    if "." in body:
        int_part, frac_part = body.split(".", 1)
    else:
        int_part, frac_part = body, ""
    int_part = int_part or "0"

    if not int_part.isdigit() or (frac_part and not frac_part.isdigit()):
        raise ValueError("non-digit characters in decimal literal")
    if len(int_part) > INT_LEN:
        raise ValueError(f"integer part exceeds {INT_LEN} digits")

    int_digits = [0] * INT_LEN
    for i, ch in enumerate(int_part):
        int_digits[INT_LEN - len(int_part) + i] = ord(ch) - 48

    frac_digits = [0] * FRAC_INTERNAL_LEN
    copy_len = min(len(frac_part), FRAC_INTERNAL_LEN)
    for i in range(copy_len):
        frac_digits[i] = ord(frac_part[i]) - 48

    if len(frac_part) > FRAC_INTERNAL_LEN:
        round_digit = ord(frac_part[FRAC_INTERNAL_LEN]) - 48
        overflow = round_half_up_decimal_buffers(int_digits, frac_digits, round_digit)
        if overflow:
            raise ValueError(f"integer part exceeds {INT_LEN} digits after decimal rounding")

    return int_digits, frac_digits


def digit_at_power(coeff: str, point: int, power: int) -> int:
    idx = point - 1 - power
    if idx < 0 or idx >= len(coeff):
        return 0
    return ord(coeff[idx]) - 48


def parse_scientific_parts(body: str):
    mantissa, exponent_raw = body.split("e", 1) if "e" in body else body.split("E", 1)
    exponent = int(exponent_raw)

    if "." in mantissa:
        int_part, frac_part = mantissa.split(".", 1)
    else:
        int_part, frac_part = mantissa, ""
    int_part = int_part or "0"
    frac_part = frac_part or ""

    if not int_part.isdigit() or (frac_part and not frac_part.isdigit()):
        raise ValueError("non-digit characters in scientific mantissa")

    coeff = (int_part + frac_part).lstrip("0")
    if not coeff:
        return [0] * INT_LEN, [0] * FRAC_INTERNAL_LEN

    point = len(coeff) + exponent - len(frac_part)
    pre_int_len = 1 if point <= 0 else point
    if pre_int_len > INT_LEN:
        raise ValueError(f"integer part exceeds {INT_LEN} digits")

    int_digits = [0] * INT_LEN
    for idx in range(INT_LEN):
        power = INT_LEN - 1 - idx
        int_digits[idx] = digit_at_power(coeff, point, power)

    frac_digits = [0] * FRAC_INTERNAL_LEN
    for idx in range(FRAC_INTERNAL_LEN):
        power = -(idx + 1)
        frac_digits[idx] = digit_at_power(coeff, point, power)

    round_digit = 0
    round_digit_idx = point + FRAC_INTERNAL_LEN
    if 0 <= round_digit_idx < len(coeff):
        round_digit = ord(coeff[round_digit_idx]) - 48

    overflow = round_half_up_decimal_buffers(int_digits, frac_digits, round_digit)
    if overflow:
        raise ValueError(f"integer part exceeds {INT_LEN} digits after scientific rounding")

    return int_digits, frac_digits


def parse_literal_to_decimal_buffers(text: str):
    normalized = normalize_numeric_literal(text)
    if not NUMERIC_RE.match(normalized):
        raise ValueError(f"invalid numeric literal: {text!r}")

    negative, body = split_leading_sign(normalized)
    if "e" in body or "E" in body:
        int_digits, frac_digits = parse_scientific_parts(body)
    else:
        int_digits, frac_digits = parse_decimal_parts(body)
    return negative, int_digits, frac_digits


def trim_decimal_tail_zeros(digits):
    i = len(digits)
    while i > 0 and digits[i - 1] == 0:
        i -= 1
    return i


def balanced_digit_and_carry(value: int):
    balanced = ((value + 5) % 10) - 5  # -5..4
    carry = (value - balanced) // 10
    return balanced, carry


def convert_to_balanced(negative, int_u8, frac_u8):
    frac_sig_len = trim_decimal_tail_zeros(frac_u8)
    source_len = INT_LEN + frac_sig_len

    out_rev = []
    carry = 0
    for linear_idx in range(source_len - 1, -1, -1):
        d = int_u8[linear_idx] if linear_idx < INT_LEN else frac_u8[linear_idx - INT_LEN]
        signed = -d if negative else d
        bal, carry = balanced_digit_and_carry(signed + carry)
        out_rev.append(bal)

    while carry != 0:
        bal, carry = balanced_digit_and_carry(carry)
        out_rev.append(bal)

    while len(out_rev) > 1 and out_rev[-1] == 0:
        out_rev.pop()

    out_fwd = list(reversed(out_rev))
    lsd_power = INT_LEN - source_len
    msd_power = lsd_power + len(out_fwd) - 1

    max_int_power = INT_LEN - 1
    min_frac_power = -FRAC_INTERNAL_LEN
    if msd_power > max_int_power:
        raise ValueError("integer part exceeds capacity after balanced conversion")

    int_bal = [0] * INT_LEN
    frac_bal = [0] * FRAC_INTERNAL_LEN
    for idx, digit in enumerate(out_fwd):
        power = msd_power - idx
        if power < min_frac_power:
            raise ValueError("fractional part exceeds capacity after balanced conversion")
        if power >= 0:
            int_idx = max_int_power - power
            int_bal[int_idx] = digit
        else:
            frac_idx = (-power) - 1
            frac_bal[frac_idx] = digit

    linear = int_bal + frac_bal
    is_zero = all(d == 0 for d in linear)
    negative_out = negative and not is_zero

    radix_pos = INT_LEN - 1
    for i, d in enumerate(linear):
        if d != 0:
            radix_pos = i

    return negative_out, int_bal, frac_bal, radix_pos


def scaled_num_from_decimal_parts(negative, int_u8, frac_u8):
    i = 0
    for d in int_u8:
        i = i * 10 + d
    f = 0
    for d in frac_u8:
        f = f * 10 + d
    n = i * (10 ** FRAC_INTERNAL_LEN) + f
    return -n if (negative and n != 0) else n


def scaled_num_from_balanced(int_bal, frac_bal):
    n = 0
    linear = int_bal + frac_bal
    for idx, d in enumerate(linear):
        power = (INT_LEN - 1) - idx
        n += d * (10 ** (power + FRAC_INTERNAL_LEN))
    return n


def rust_tokens(digits):
    return ", ".join(f"ScalarDecimalDigit::{DIGIT_TOKEN[d]}" for d in digits)


def fuzz(count: int):
    accepted = 0
    attempts = 0
    while accepted < count:
        attempts += 1
        negative = random.choice([False, True])
        int_len = random.randint(1, INT_LEN)
        frac_len = random.randint(0, FRAC_INTERNAL_LEN)

        int_part = "".join(str(random.randint(0, 9)) for _ in range(int_len))
        int_part = str(random.randint(1, 9)) + int_part[1:]
        frac_part = "".join(str(random.randint(0, 9)) for _ in range(frac_len))
        text = f"{'-' if negative else ''}{int_part}" + (f".{frac_part}" if frac_len else "")

        try:
            negative0, iu8, fu8 = parse_literal_to_decimal_buffers(text)
            negative1, ib, fb, _ = convert_to_balanced(negative0, iu8, fu8)
        except ValueError:
            continue
        n0 = scaled_num_from_decimal_parts(negative0, iu8, fu8)
        n1 = scaled_num_from_balanced(ib, fb)
        if negative1:
            assert any(d != 0 for d in (ib + fb))
        if n0 != n1:
            raise AssertionError(f"mismatch for {text}: {n0} != {n1}")
        accepted += 1
    print(f"fuzz ok: {accepted} (attempts: {attempts})")


def main():
    ap = argparse.ArgumentParser(description="Convert scalar literals to balanced digits.")
    ap.add_argument("--value", type=str, help="decimal/scientific literal (Wolfram forms accepted)")
    ap.add_argument("--fuzz", type=int, default=0, help="random verification iterations")
    args = ap.parse_args()

    if args.fuzz > 0:
        fuzz(args.fuzz)

    if args.value:
        negative, int_u8, frac_u8 = parse_literal_to_decimal_buffers(args.value)
        negative_bal, int_bal, frac_bal, radix = convert_to_balanced(negative, int_u8, frac_u8)

        n0 = scaled_num_from_decimal_parts(negative, int_u8, frac_u8)
        n1 = scaled_num_from_balanced(int_bal, frac_bal)
        assert n0 == n1, "exact-value verification failed"

        print("normalized_negative:", negative)
        print("balanced_negative:", negative_bal)
        print("radix_position:", radix)
        print("int_balanced:", int_bal)
        print("frac_balanced_internal:", frac_bal)
        print("\nRust int_digits:")
        print(rust_tokens(int_bal))
        print("\nRust frac_digits (44):")
        print(rust_tokens(frac_bal))


if __name__ == "__main__":
    main()
