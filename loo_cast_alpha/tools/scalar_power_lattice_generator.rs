#!/usr/bin/env rust-script

use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

const MIN_EXP: i32 = -44;
const MAX_EXP: i32 = 35;
const MAX_SAFE_TOP_MANTISSA: u32 = 4; // at 10^35, 5..9 overflows after balanced carry

fn main() -> std::io::Result<()> {
    println!("generating power lattice...");

    let exe_dir = PathBuf::from(file!()).parent().unwrap().to_path_buf();
    let generated_dir = exe_dir.join("generated");
    create_dir_all(&generated_dir)?;
    let output_path = generated_dir.join("power_lattice.txt");

    let file = File::create(&output_path)?;
    let mut out = BufWriter::new(file);

    let mut emitted = 0usize;
    for exp in MIN_EXP..=MAX_EXP {
        emitted += emit_bucket(exp, &mut out)?;
    }

    out.flush()?;
    println!("done");
    println!("entries emitted: {}", emitted);
    println!("output: {}", output_path.display());
    Ok(())
}

fn emit_bucket(exp: i32, out: &mut impl Write) -> std::io::Result<usize> {
    let mut emitted = 0usize;
    let max_mantissa = if exp == MAX_EXP {
        MAX_SAFE_TOP_MANTISSA
    } else {
        9
    };

    for mantissa in 1u32..=max_mantissa {
        let name = if exp < 0 {
            negative_name(mantissa, exp)
        } else {
            positive_name(mantissa, exp)
        };

        let value = render_value(mantissa, exp);
        writeln!(out, "{}={}", name, value)?;
        emitted += 1;
    }

    Ok(emitted)
}

fn render_value(mantissa: u32, exp: i32) -> String {
    if exp >= 0 {
        let mut s = mantissa.to_string();
        s.push_str(&"0".repeat(exp as usize));
        s
    } else {
        let zeros = (-exp - 1) as usize;
        format!("0.{}{}", "0".repeat(zeros), mantissa)
    }
}

fn positive_name(mantissa: u32, exp: i32) -> String {
    let (prefix, scale) = folded_parts(mantissa, exp);
    if scale.is_empty() {
        prefix.to_string()
    } else {
        format!("{}_{}", prefix, scale)
    }
}

fn negative_name(mantissa: u32, exp: i32) -> String {
    let abs_exp = -exp;
    let numerator = numerator_word(mantissa);
    let denominator = denominator_word(abs_exp);
    format!("{numerator}_over_{denominator}")
}

fn folded_parts(mantissa: u32, exp: i32) -> (&'static str, &'static str) {
    match exp % 3 {
        0 => (digit_word(mantissa), magnitude_group(exp)),
        1 => (tens_word(mantissa), magnitude_group(exp - 1)),
        2 => (hundreds_word(mantissa), magnitude_group(exp - 2)),
        _ => unreachable!(),
    }
}

fn digit_word(n: u32) -> &'static str {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => unreachable!(),
    }
}

fn tens_word(n: u32) -> &'static str {
    match n {
        1 => "ten",
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => unreachable!(),
    }
}

fn hundreds_word(n: u32) -> &'static str {
    match n {
        1 => "one_hundred",
        2 => "two_hundred",
        3 => "three_hundred",
        4 => "four_hundred",
        5 => "five_hundred",
        6 => "six_hundred",
        7 => "seven_hundred",
        8 => "eight_hundred",
        9 => "nine_hundred",
        _ => unreachable!(),
    }
}

fn magnitude_group(exp: i32) -> &'static str {
    match exp {
        0 => "",
        1 => "ten",
        2 => "one_hundred",
        3 => "thousand",
        6 => "million",
        9 => "billion",
        12 => "trillion",
        15 => "quadrillion",
        18 => "quintillion",
        21 => "sextillion",
        24 => "septillion",
        27 => "octillion",
        30 => "nonillion",
        33 => "decillion",
        36 => "undecillion",
        39 => "duodecillion",
        42 => "tredecillion",
        _ => unreachable!(),
    }
}

fn numerator_word(mantissa: u32) -> String {
    if mantissa == 1 {
        "one".to_string()
    } else if mantissa < 10 {
        digit_word(mantissa).to_string()
    } else if mantissa < 100 {
        let t = mantissa / 10;
        let o = mantissa % 10;
        if o == 0 {
            tens_word(t).to_string()
        } else {
            format!("{}_{}", tens_word(t), digit_word(o))
        }
    } else {
        let h = mantissa / 100;
        let rem = mantissa % 100;
        if rem == 0 {
            format!("{}_hundred", digit_word(h))
        } else if rem < 10 {
            format!("{}_hundred_{}", digit_word(h), digit_word(rem))
        } else {
            let t = rem / 10;
            let o = rem % 10;
            if o == 0 {
                format!("{}_hundred_{}", digit_word(h), tens_word(t))
            } else {
                format!(
                    "{}_hundred_{}_{}",
                    digit_word(h),
                    tens_word(t),
                    digit_word(o)
                )
            }
        }
    }
}

fn denominator_word(abs_exp: i32) -> String {
    match abs_exp {
        1 => "ten".to_string(),
        2 => "one_hundred".to_string(),
        _ => {
            let rem = abs_exp % 3;
            let group_exp = abs_exp - rem;
            let group = magnitude_group(group_exp);
            match rem {
                0 => format!("one_{group}"),
                1 => format!("ten_{group}"),
                2 => format!("one_hundred_{group}"),
                _ => unreachable!(),
            }
        }
    }
}
