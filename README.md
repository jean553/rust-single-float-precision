# rust-single-float-precision

Attempt to create a script that takes a number (base 10)
and displays its value as stored by `f32` type
(using single float precision IEEE 754).

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
