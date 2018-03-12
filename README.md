# rust-single-float-precision

Attempt to create a script that takes a number (base 10)
and displays its value as stored by `f32` type
(using single float precision IEEE 754).

## Warning

This is an experimental project. So far, the conversion
does not work for high values. Issues have been created for that.

The same working behavior is handled by `std::f32::to_bits()`.

## Example

Input:

```bash
10
```

Ouput:

```
0 10000010 01000000000000000000000
```

The example above contains the three parts of the IEEE 754 single-float precision format:
 * the `sign bit`,
 * the `exponent`,
 * the `fraction`

When a number is represented using the IEEE 754 normal form, each number part contains the following information:

```
(sign bit) 1.(fraction) x 2^(exponent)
```

## Float to IEEE 754 form

Let's consider the float: `7.3` to be represented on 32 bits.

### Define the sign

The number is positive, so the sign bit is equal to `0`.

### Define the exponent

We then deduce the exponent (divide the number until the number left decimal side is equal to 1):
 * 7.3 / 2 = 3.65
 * 3.65 / 2 = 1.825

The division has been applied twice, so the exponent value is `2`.

In order to find the exponent value, we add the result to `127`: `127 + 2 = 129`,
so `129d = 1000 0001b`.

### Define the fraction

The normal representation of the value is: `1.825 x 2^2`.

We now take the decimal part of the number `1.825` (so `0.825`) and generate 23 bits from it (for the fraction part):
 * 0.825 x 2 = 1.65         (1)
 * 0.65 x 2 = 1.3           (1)
 * 0.3 x 2 = 0.6            (0)
 * 0.6 x 2 = 1.2            (1)
 * 0.2 x 2 = 0.4            (0)
 * 0.4 x 2 = 0.8            (0)
 * 0.8 x 2 = 1.6            (1)
 * 0.6 <- value already met before, recursive pattern

So the fraction value is `11010011 00110011 0011001`.

The final IEEE 754 value is: `0 10000001 11010011 00110011 0011001`.
