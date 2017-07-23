use std::env::args;

fn main() {

    let decimal_string: String = args().nth(1).unwrap();
    let decimal: u32 = decimal_string.trim().parse().expect("parsing error");

    let mut value: u32 = 1;
    let mut last_value: u32 = 1;

    let mut pow: u32 = 0;
    let mut last_pow: u32 = 0;

    while value < decimal {

        last_value = value;
        last_pow = pow;

        pow += 1;
        value = 2u32.pow(pow);
    }

    println!(
        "2^{} (={}) is the smallest value before {}.",
        last_pow,
        last_value,
        decimal,
    );

    let decimal_exponent: u32 = 127 + last_pow;

    println!(
        "The exponent is {} (127 + {}).",
        decimal_exponent,
        last_pow,
    );

    let binary_exponent: String = format!(
        "{:b}",
        decimal_exponent,
    );

    println!(
        "Binary exponent: {}.",
        binary_exponent,
    );

    let significant_decimal: f32 = decimal as f32 / last_value as f32;

    println!(
        "Significant decimal is {} ({} / {}).",
        significant_decimal,
        decimal,
        last_value,
    );

    /* ignore the heading 1 (i.e 1.25 becomes 0.25)*/

    let significant_decimal_minus_one = significant_decimal - 1.0;

    println!(
        "Considered significant decimal is {}.",
        significant_decimal_minus_one,
    );

    let value: f32 = 0.45;

    /* TODO: #1 convert float into bits string representation,
     * cannot be done using format!() as f32 does not implement
     * the fmt::Binary trait */
}
