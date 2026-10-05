//! Plain decimal formatting of balanced Scale Stack coordinates.

pub(super) fn push_float_decimal_terms(terms: &mut Vec<(i32, i32)>, value: f32, scale_shift: i32) {
    if value == 0.0 {
        return;
    }

    let negative = value.is_sign_negative();
    let rendered = value.abs().to_string();
    let (mantissa, exponent) = match rendered.find(|c| c == 'e' || c == 'E') {
        Some(index) => (
            &rendered[..index],
            rendered[index + 1..]
                .parse::<i32>()
                .expect("f32 exponent formatting is decimal"),
        ),
        None => (rendered.as_str(), 0),
    };

    let fractional_digits = mantissa
        .split_once('.')
        .map_or(0_i32, |(_, fractional)| fractional.len() as i32);
    let digits = mantissa.chars().filter(|c| *c != '.').collect::<Vec<_>>();
    let least_exponent = scale_shift + exponent - fractional_digits;

    for (index, ch) in digits.iter().rev().enumerate() {
        let digit = ch
            .to_digit(10)
            .expect("f32 display contains decimal digits") as i32;
        if digit == 0 {
            continue;
        }
        terms.push((
            least_exponent + index as i32,
            if negative { -digit } else { digit },
        ));
    }
}

fn normalize_unsigned_decimal(digits: &mut Vec<i32>) {
    let mut index = 0;
    while index < digits.len() {
        if digits[index] >= 10 {
            let carry = digits[index] / 10;
            digits[index] %= 10;
            if index + 1 == digits.len() {
                digits.push(carry);
            } else {
                digits[index + 1] += carry;
            }
        }
        index += 1;
    }

    while digits.len() > 1 && digits.last() == Some(&0) {
        digits.pop();
    }
}

fn compare_unsigned_decimal(lhs: &[i32], rhs: &[i32]) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    let lhs_top = lhs.iter().rposition(|digit| *digit != 0);
    let rhs_top = rhs.iter().rposition(|digit| *digit != 0);

    match (lhs_top, rhs_top) {
        (None, None) => Ordering::Equal,
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        (Some(lhs_top), Some(rhs_top)) => match lhs_top.cmp(&rhs_top) {
            Ordering::Equal => {
                for index in (0..=lhs_top).rev() {
                    match lhs[index].cmp(&rhs[index]) {
                        Ordering::Equal => {}
                        other => return other,
                    }
                }
                Ordering::Equal
            }
            other => other,
        },
    }
}

fn subtract_unsigned_decimal(lhs: &[i32], rhs: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(lhs.len());
    let mut borrow = 0;

    for index in 0..lhs.len() {
        let mut value = lhs[index] - rhs.get(index).copied().unwrap_or(0) - borrow;
        if value < 0 {
            value += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }
        result.push(value);
    }

    debug_assert_eq!(borrow, 0);
    while result.len() > 1 && result.last() == Some(&0) {
        result.pop();
    }
    result
}

pub(super) fn format_decimal_terms(terms: &[(i32, i32)]) -> String {
    use std::cmp::Ordering;

    if terms.is_empty() {
        return "0".to_string();
    }

    let min_exponent = terms.iter().map(|(exponent, _)| *exponent).min().unwrap();
    let max_exponent = terms.iter().map(|(exponent, _)| *exponent).max().unwrap();
    let width = (max_exponent - min_exponent + 2) as usize;

    let mut positive = vec![0_i32; width];
    let mut negative = vec![0_i32; width];

    for &(exponent, coefficient) in terms {
        let index = (exponent - min_exponent) as usize;
        if coefficient >= 0 {
            positive[index] += coefficient;
        } else {
            negative[index] += -coefficient;
        }
    }

    normalize_unsigned_decimal(&mut positive);
    normalize_unsigned_decimal(&mut negative);

    let ordering = compare_unsigned_decimal(&positive, &negative);
    if ordering == Ordering::Equal {
        return "0".to_string();
    }

    let (negative_sign, magnitude) = match ordering {
        Ordering::Greater => (false, subtract_unsigned_decimal(&positive, &negative)),
        Ordering::Less => (true, subtract_unsigned_decimal(&negative, &positive)),
        Ordering::Equal => unreachable!(),
    };

    let highest_index = magnitude
        .iter()
        .rposition(|digit| *digit != 0)
        .expect("nonzero magnitude has a nonzero digit");
    let highest_exponent = min_exponent + highest_index as i32;

    let digit_at = |exponent: i32| -> i32 {
        let index = exponent - min_exponent;
        if index < 0 {
            0
        } else {
            magnitude.get(index as usize).copied().unwrap_or(0)
        }
    };

    let mut rendered = String::new();
    if negative_sign {
        rendered.push('-');
    }

    if highest_exponent >= 0 {
        for exponent in (0..=highest_exponent).rev() {
            rendered.push(char::from_digit(digit_at(exponent) as u32, 10).unwrap());
        }
    } else {
        rendered.push('0');
    }

    if min_exponent < 0 {
        let mut fractional = String::new();
        for exponent in (min_exponent..=-1).rev() {
            fractional.push(char::from_digit(digit_at(exponent) as u32, 10).unwrap());
        }

        while fractional.ends_with('0') {
            fractional.pop();
        }

        if !fractional.is_empty() {
            rendered.push('.');
            rendered.push_str(&fractional);
        }
    }

    rendered
}
