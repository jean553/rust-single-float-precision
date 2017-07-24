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
