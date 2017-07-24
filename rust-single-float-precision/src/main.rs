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

    let mut decimal = significant_decimal - 1.0;

    println!(
        "Convert {} ({} - 1) to binary...",
        decimal,
        significant_decimal,
    );

    let mut significant_string = String::new();

    /* TODO: #1 the provided solution below does not work with high numbers */
    for counter in 0..23 {
        let result = decimal * 2.0;
        let truncate: u8 = result.trunc() as u8;
        significant_string += &truncate.to_string();
        decimal *= 10.0;
        decimal = decimal.fract();
    }

    /* TODO: #6 the sign is not handled at all for now
     * as the program input only accepts positive values */
    println!(
        "Final: 0 {} {}",
        binary_exponent,
        significant_string,
    );
}
