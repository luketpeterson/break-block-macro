
# break-block-macro

A little syntactic sugar to tide us over until the `?` operator can be used to exit try-catch scopes

The `bb` macro behaves like the `?` operator, but will break out of labeled scopes to achieve something that resembles try-catch.

See [this issue](https://github.com/rust-lang/rust/issues/31436) for background.

## Usage

```
use break_block_macro::bb;

let result = 'block: {
    let one = bb!(Ok("one"));
    assert_eq!(one, "one");

    let _two = bb!(Err("two"));
    Ok("three")
};
assert_eq!(result, Err("two"));
```
